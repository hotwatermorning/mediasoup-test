use crate::room::Room;
use crate::util::get_env;
use actix::prelude::*;
use actix_web_actors::ws;
use event_listener_primitives::HandlerId;
use mediasoup::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;

pub mod messages;
use messages::{ClientMessage, InternalMessage, ServerMessage, TransportOptions};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
pub struct ParticipantId(Uuid);

impl fmt::Display for ParticipantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ParticipantId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Consumer/producer transports pair for the client
struct Transports {
    consumer: WebRtcTransport,
    producer: WebRtcTransport,
}

/// Actor that will represent WebSocket connection from the client, it will handle inbound and
/// outbound WebSocket messages in JSON.
///
/// See https://actix.rs/docs/websockets/ for official `actix-web` documentation.
pub struct ParticipantConnection {
    id: ParticipantId,
    name: String,
    /// RTP capabilities received from the client
    client_rtp_capabilities: Option<RtpCapabilities>,
    /// Consumers associated with this client, preventing them from being destroyed
    consumers: HashMap<ConsumerId, Consumer>,
    /// Producers associated with this client, preventing them from being destroyed
    producers: Vec<Producer>,
    /// Consumer and producer transports associated with this client
    transports: Transports,
    /// Room to which the client belongs
    room: Room,
    /// Event handlers that were attached and need to be removed when participant connection is
    /// destroyed
    attached_handlers: Vec<HandlerId>,
}

impl Drop for ParticipantConnection {
    fn drop(&mut self) {
        self.room.remove_participant(&self.id);
    }
}

impl ParticipantConnection {
    /// Create a new instance representing WebSocket connection
    pub async fn new(room: Room) -> Result<Self, String> {
        // We know that for videoroom example we'll need 2 transports, so we can create both
        // right away. This may not be the case for real-world applications or you may create
        // this at a different time and/or in different order.
        let mut transport_options =
            WebRtcTransportOptions::new(TransportListenIps::new(ListenIp {
                ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                announced_ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            }));
        transport_options.enable_tcp = true;
        transport_options.prefer_udp = true;
        let producer_transport = room
            .router()
            .create_webrtc_transport(transport_options.clone())
            .await
            .map_err(|error| format!("Failed to create producer transport: {error}"))?;

        let consumer_transport = room
            .router()
            .create_webrtc_transport(transport_options)
            .await
            .map_err(|error| format!("Failed to create consumer transport: {error}"))?;

        Ok(Self {
            id: ParticipantId::new(),
            name: "".to_string(),
            client_rtp_capabilities: None,
            consumers: HashMap::new(),
            producers: vec![],
            transports: Transports {
                consumer: consumer_transport,
                producer: producer_transport,
            },
            room,
            attached_handlers: Vec::new(),
        })
    }
}

impl Actor for ParticipantConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("[participant_id {}] WebSocket connection created", self.id);

        // We know that both consumer and producer transports will be used, so we sent server
        // information about both in an initialization message alongside with router
        // capabilities to the client right after WebSocket connection is established
        let server_init_message = ServerMessage::Init {
            room_id: self.room.id(),
            consumer_transport_options: TransportOptions {
                id: self.transports.consumer.id(),
                dtls_parameters: self.transports.consumer.dtls_parameters(),
                ice_candidates: self.transports.consumer.ice_candidates().clone(),
                ice_parameters: self.transports.consumer.ice_parameters().clone(),
            },
            producer_transport_options: TransportOptions {
                id: self.transports.producer.id(),
                dtls_parameters: self.transports.producer.dtls_parameters(),
                ice_candidates: self.transports.producer.ice_candidates().clone(),
                ice_parameters: self.transports.producer.ice_parameters().clone(),
            },
            router_rtp_capabilities: self.room.router().rtp_capabilities().clone(),
        };

        let address = ctx.address();
        address.do_send(server_init_message);

        // Listen for new producers added to the room
        self.attached_handlers.push(self.room.on_producer_add({
            let own_participant_id = self.id;
            let address = address.clone();

            move |participant_id, name, producer| {
                if &own_participant_id == participant_id {
                    return;
                }
                address.do_send(ServerMessage::ProducerAdded {
                    participant_id: *participant_id,
                    name: name.to_string(),
                    producer_id: producer.id(),
                });
            }
        }));

        // Listen for producers removed from the the room
        self.attached_handlers.push(self.room.on_producer_remove({
            let own_participant_id = self.id;
            let address = address.clone();

            move |participant_id, producer_id| {
                if &own_participant_id == participant_id {
                    return;
                }
                address.do_send(ServerMessage::ProducerRemoved {
                    participant_id: *participant_id,
                    producer_id: *producer_id,
                });
            }
        }));

        // Notify client about any producers that already exist in the room
        for (participant_id, name, producer_id) in self.room.get_all_producers() {
            address.do_send(ServerMessage::ProducerAdded {
                participant_id,
                name: name.to_owned(),
                producer_id,
            });
        }
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("[participant_id {0}] WebSocket connection closed", self.id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ParticipantConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // Here we handle incoming WebSocket messages, intentionally not handling continuation
        // messages since we know all messages will fit into a single frame, but in real-world
        // apps you need to handle continuation frames too (`ws::Message::Continuation`)
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(message) => {
                    // Parse JSON into an enum and just send it back to the actor to be
                    // processed by another handler below, it is much more convenient to just
                    // parse it in one place and have typed data structure everywhere else
                    ctx.address().do_send(message);
                }
                Err(error) => {
                    eprintln!("Failed to parse client message: {error}\n{text}");
                }
            },
            Ok(ws::Message::Binary(bin)) => {
                eprintln!("Unexpected binary message: {bin:?}");
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<ClientMessage> for ParticipantConnection {
    type Result = ();

    fn handle(&mut self, message: ClientMessage, ctx: &mut Self::Context) {
        match message {
            ClientMessage::Init {
                name,
                rtp_capabilities,
            } => {
                // We need to know client's RTP capabilities, those are sent using
                // initialization message and are stored in connection struct for future use
                self.client_rtp_capabilities.replace(rtp_capabilities);
                self.name = name.clone();
                self.room.set_participant_name(self.id, name);
            }
            ClientMessage::ConnectProducerTransport { dtls_parameters } => {
                let participant_id = self.id;
                let address = ctx.address();
                let transport = self.transports.producer.clone();
                // Establish connection for producer transport using DTLS parameters received
                // from the client, but doing so in a background task since this handler is
                // synchronous
                actix::spawn(async move {
                    match transport
                        .connect(WebRtcTransportRemoteParameters { dtls_parameters })
                        .await
                    {
                        Ok(_) => {
                            address.do_send(ServerMessage::ConnectedProducerTransport);
                            println!(
                                "[participant_id {participant_id}] Producer transport connected"
                            );
                        }
                        Err(error) => {
                            eprintln!("Failed to connect producer transport: {error}");
                            address.do_send(InternalMessage::Stop);
                        }
                    }
                });
            }
            ClientMessage::Produce {
                kind,
                rtp_parameters,
            } => {
                let participant_id = self.id;
                let address = ctx.address();
                let transport = self.transports.producer.clone();
                let room = self.room.clone();
                // Use producer transport to create a new producer on the server with given RTP
                // parameters
                actix::spawn(async move {
                    match transport
                        .produce(ProducerOptions::new(kind, rtp_parameters))
                        .await
                    {
                        Ok(producer) => {
                            let id = producer.id();
                            address.do_send(ServerMessage::Produced { id });
                            // Add producer to the room so that others can consume it
                            room.add_producer(participant_id, producer.clone());
                            // Producer is stored in a hashmap since if we don't do it, it will
                            // get destroyed as soon as its instance goes out out scope
                            address.do_send(InternalMessage::SaveProducer(producer));
                            println!(
                                "[participant_id {participant_id}] {kind:?} producer created: {id}"
                            );
                        }
                        Err(error) => {
                            eprintln!(
                                "[participant_id {participant_id}] Failed to create {kind:?} producer: {error}"
                            );
                            address.do_send(InternalMessage::Stop);
                        }
                    }
                });
            }
            ClientMessage::ConnectConsumerTransport { dtls_parameters } => {
                let participant_id = self.id;
                let address = ctx.address();
                let transport = self.transports.consumer.clone();
                // The same as producer transport, but for consumer transport
                actix::spawn(async move {
                    match transport
                        .connect(WebRtcTransportRemoteParameters { dtls_parameters })
                        .await
                    {
                        Ok(_) => {
                            address.do_send(ServerMessage::ConnectedConsumerTransport);
                            println!(
                                "[participant_id {participant_id}] Consumer transport connected"
                            );
                        }
                        Err(error) => {
                            eprintln!(
                                "[participant_id {participant_id}] Failed to connect consumer transport: {error}"
                            );
                            address.do_send(InternalMessage::Stop);
                        }
                    }
                });
            }
            ClientMessage::Consume { producer_id } => {
                let participant_id = self.id;
                let address = ctx.address();
                let transport = self.transports.consumer.clone();
                let rtp_capabilities = match self.client_rtp_capabilities.clone() {
                    Some(rtp_capabilities) => rtp_capabilities,
                    None => {
                        eprintln!(
                            "[participant_id {participant_id}] Client should send RTP capabilities before \
                            consuming"
                        );
                        return;
                    }
                };
                // Create consumer for given producer ID, while first making sure that RTP
                // capabilities were sent by the client prior to that
                actix::spawn(async move {
                    let mut options = ConsumerOptions::new(producer_id, rtp_capabilities);
                    options.paused = true;

                    match transport.consume(options).await {
                        Ok(consumer) => {
                            let id = consumer.id();
                            let kind = consumer.kind();
                            let rtp_parameters = consumer.rtp_parameters().clone();
                            address.do_send(ServerMessage::Consumed {
                                id,
                                producer_id,
                                kind,
                                rtp_parameters,
                            });
                            // Consumer is stored in a hashmap since if we don't do it, it will
                            // get destroyed as soon as its instance goes out out scope
                            address.do_send(InternalMessage::SaveConsumer(consumer));
                            println!(
                                "[participant_id {participant_id}] {kind:?} consumer created: {id}"
                            );
                        }
                        Err(error) => {
                            eprintln!(
                                "[participant_id {participant_id}] Failed to create consumer: {error}"
                            );
                            address.do_send(InternalMessage::Stop);
                        }
                    }
                });
            }
            ClientMessage::ConsumerResume { id } => {
                if let Some(consumer) = self.consumers.get(&id).cloned() {
                    let participant_id = self.id;
                    actix::spawn(async move {
                        match consumer.resume().await {
                            Ok(_) => {
                                println!(
                                    "[participant_id {}] Successfully resumed {:?} consumer {}",
                                    participant_id,
                                    consumer.kind(),
                                    consumer.id(),
                                );
                            }
                            Err(error) => {
                                println!(
                                    "[participant_id {}] Failed to resume {:?} consumer {}: {}",
                                    participant_id,
                                    consumer.kind(),
                                    consumer.id(),
                                    error,
                                );
                            }
                        }
                    });
                }
            }
            ClientMessage::StartRecording { output_name } => {
                let participant_id = self.id;
                let mut room = self.room.clone();
                actix::spawn(async move {
                    match room.start_recording(&participant_id, &output_name).await {
                        Ok(_) => {
                            println!(
                                "[participant_id {}] Successfully started recording",
                                participant_id,
                            );
                        }
                        Err(error) => {
                            println!(
                                "[participant_id {}] Failed to start recording {}",
                                participant_id, error,
                            );
                        }
                    }
                });
            }
            ClientMessage::StopRecording {} => {
                let participant_id = self.id;
                let mut room = self.room.clone();
                actix::spawn(async move {
                    match room.stop_recording(&participant_id).await {
                        Ok(_) => {
                            println!(
                                "[participant_id {}] Successfully stopped recording",
                                participant_id,
                            );
                        }
                        Err(error) => {
                            println!(
                                "[participant_id {}] Failed to stop recording {}",
                                participant_id, error,
                            );
                        }
                    }
                });
            }
        }
    }
}

/// Simple handler that will transform typed server messages into JSON and send them over to the
/// client over WebSocket connection
impl Handler<ServerMessage> for ParticipantConnection {
    type Result = ();

    fn handle(&mut self, message: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&message).unwrap());
    }
}

/// Convenience handler for internal messages, these actions require mutable access to the
/// connection struct and having such message handler makes it easy to use from background tasks
/// where otherwise Mutex would have to be used instead
impl Handler<InternalMessage> for ParticipantConnection {
    type Result = ();

    fn handle(&mut self, message: InternalMessage, ctx: &mut Self::Context) {
        match message {
            InternalMessage::Stop => {
                ctx.stop();
            }
            InternalMessage::SaveProducer(producer) => {
                // Retain producer to prevent it from being destroyed
                self.producers.push(producer);
            }
            InternalMessage::SaveConsumer(consumer) => {
                self.consumers.insert(consumer.id(), consumer);
            }
        }
    }
}
