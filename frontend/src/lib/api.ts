/* eslint-disable no-console */
import { Device } from 'mediasoup-client';
import type { MediaKind, RtpCapabilities, RtpParameters } from 'mediasoup-client/lib/RtpParameters';
import type { DtlsParameters, TransportOptions, Transport } from 'mediasoup-client/lib/Transport';
import type { ConsumerOptions } from 'mediasoup-client/lib/Consumer';

type Brand<K, T> = K & { __brand: T };

type RoomId = Brand<string, 'RoomId'>;
type ParticipantId = Brand<string, 'ParticipantId'>;
type ConsumerId = Brand<string, 'ConsumerId'>;
type ProducerId = Brand<string, 'ProducerId'>;

interface ServerInit {
	action: 'Init';
	roomId: RoomId;
	consumerTransportOptions: TransportOptions;
	producerTransportOptions: TransportOptions;
	routerRtpCapabilities: RtpCapabilities;
}

interface ServerProducerAdded {
	action: 'ProducerAdded';
	participantId: ParticipantId;
	name: string;
	producerId: ProducerId;
}

interface ServerProducerRemoved {
	action: 'ProducerRemoved';
	participantId: ParticipantId;
	producerId: ProducerId;
}

interface ServerConnectedProducerTransport {
	action: 'ConnectedProducerTransport';
}

interface ServerProduced {
	action: 'Produced';
	id: ProducerId;
}

interface ServerConnectedConsumerTransport {
	action: 'ConnectedConsumerTransport';
}

interface ServerConsumed {
	action: 'Consumed';
	id: ConsumerId;
	kind: MediaKind;
	rtpParameters: RtpParameters;
}

type ServerMessage =
	ServerInit |
	ServerProducerAdded |
	ServerProducerRemoved |
	ServerConnectedProducerTransport |
	ServerProduced |
	ServerConnectedConsumerTransport |
	ServerConsumed;

interface ClientInit {
	action: 'Init';
	name: string;
	rtpCapabilities: RtpCapabilities;
}

interface ClientConnectProducerTransport {
	action: 'ConnectProducerTransport';
	dtlsParameters: DtlsParameters;
}

interface ClientConnectConsumerTransport {
	action: 'ConnectConsumerTransport';
	dtlsParameters: DtlsParameters;
}

interface ClientProduce {
	action: 'Produce';
	kind: MediaKind;
	rtpParameters: RtpParameters;
}

interface ClientConsume {
	action: 'Consume';
	producerId: ProducerId;
}

interface ClientConsumerResume {
	action: 'ConsumerResume';
	id: ConsumerId;
}

type ClientMessage =
	ClientInit |
	ClientConnectProducerTransport |
	ClientProduce |
	ClientConnectConsumerTransport |
	ClientConsume |
	ClientConsumerResume;

export class Participant
{
	private _name = "";
	private readonly mediaStream = new MediaStream();
  public readonly id: ParticipantId;

	constructor(
		public readonly id_: ParticipantId
	)
	{
    this.id = id_;
  }

	public get name() { return this._name; }
	public set name(name: string) { this._name = name; }

	public addTrack(track: MediaStreamTrack): void
	{
		this.mediaStream.addTrack(track);
	}

	public deleteTrack(track: MediaStreamTrack): void
	{
		this.mediaStream.removeTrack(track);
	}

	public hasTracks(): boolean
	{
		return this.mediaStream.getTracks().length > 0;
	}

  public bind(video: HTMLVideoElement): void
  {
    video.srcObject = this.mediaStream;
  }
}

export class Participants
{
	private participants = new Map<ParticipantId, Participant>();
	private producerIdToTrack = new Map<ProducerId, MediaStreamTrack>();
  private updateTrigger: () => void;

  public constructor(updateTriggerFunc: () => void)
  {
    this.updateTrigger = updateTriggerFunc;
  }

	public addTrack(
		participantId: ParticipantId,
		name: string,
		producerId: ProducerId,
		track: MediaStreamTrack,
  ): void
	{
		this.producerIdToTrack.set(producerId, track);
		const p = this.getOrCreateParticipant(participantId);
		p.name = name;
		p.addTrack(track);
    this.updateTrigger();
	}

	public deleteTrack(participantId: ParticipantId, producerId: ProducerId)
	{
		const track = this.producerIdToTrack.get(producerId);

		if (track)
		{
			const participant = this.getParticipant(participantId);
			if (participant !== undefined) {
				participant.deleteTrack(track);
				if (!participant.hasTracks())
				{
					this.participants.delete(participantId);
				}
			}
		}
		this.updateTrigger();
	}

	getOrCreateParticipant(id: ParticipantId): Participant
	{
		let participant = this.participants.get(id);

		if (!participant)
		{
			participant = new Participant(id);
			this.participants.set(id, participant);
		}

		return participant;
	}

	getParticipant(id: ParticipantId): Participant | undefined
	{
		return this.participants.get(id);
	}

  getParticipants(): {id: string, name: string}[]
  {
    return [...this.participants.entries()].map(([id, data]) => {
			return {
				id,
				name: data.name,
			};
		});
  }

  bind(id: string, video: HTMLVideoElement): void
  {
    let p = this.participants.get(id as ParticipantId);
    if(!p) {
      return;
    }

    p.bind(video);
  }
}

const getIceServers = ():RTCIceServer[] => {
	return [{
		urls: (import.meta.env.VITE_TURN_URLS as string).split(","),
		username: import.meta.env.VITE_TURN_USERNAME,
		credential: import.meta.env.VITE_TURN_CREDENTIAL,
	}];
}

let shouldUseTurnServer: boolean = false;

export async function init(
	name: string,
  participants: Participants,
  sendPreview: HTMLVideoElement,
)
{
	const roomId = (new URL(location.href)).searchParams.get('roomId') as RoomId | undefined;
	const wsUrl = new URL(import.meta.env.VITE_SFU_WEBSOCKET_URL);
	console.log(`wsUrl: ${wsUrl}`);

	if (roomId)
	{
		wsUrl.searchParams.set('roomId', roomId);
	}

	const ws = new WebSocket(wsUrl.toString());

	function send(message: ClientMessage)
	{
		ws.send(JSON.stringify(message));
	}

	const device = new Device();
	let producerTransport: Transport | undefined;
	let consumerTransport: Transport | undefined;

	let sequentialMessages: Promise<void> = Promise.resolve();
	const waitingForResponse: Map<ServerMessage['action'], Function> = new Map();

	const onmessage = async (message: ServerMessage) =>
	{
		switch (message.action)
		{
			case 'Init': {
				console.log("on init");
				if (!roomId)
				{
					const url = new URL(location.href);

					url.searchParams.set('roomId', message.roomId);
					history.pushState({}, '', url.toString());
				}
				// It is expected that server will send initialization message right after
				// WebSocket connection is established
				await device.load({
					routerRtpCapabilities : message.routerRtpCapabilities
				});

				// Send client-side initialization message back right away
				send({
					action          : 'Init',
					name            : name,
					rtpCapabilities : device.rtpCapabilities
				});

				// Producer transport is needed to send audio and video to SFU
        if(shouldUseTurnServer) {
          message.producerTransportOptions.iceTransportPolicy = "relay";
          message.producerTransportOptions.iceServers = getIceServers();
        }
				producerTransport = device.createSendTransport(
					message.producerTransportOptions
				);

				producerTransport
					.on('connect', ({ dtlsParameters }, success) =>
					{
						// Send request to establish producer transport connection
						send({
							action : 'ConnectProducerTransport',
							dtlsParameters
						});
						// And wait for confirmation, but, obviously, no error handling,
						// which you should definitely have in real-world applications
						waitingForResponse.set('ConnectedProducerTransport', () =>
						{
							success();
							console.log('Producer transport connected');
						});
					})
					.on('produce', ({ kind, rtpParameters }, success) =>
					{
						// Once connection is established, send request to produce
						// audio or video track
						send({
							action : 'Produce',
							kind,
							rtpParameters
						});
						// And wait for confirmation, but, obviously, no error handling,
						// which you should definitely have in real-world applications
						waitingForResponse.set('Produced', ({ id }: { id: string }) =>
						{
							success({ id });
						});
					});

				// Request microphone and camera access, in real-world apps you may want
				// to do this separately so that audio-only and video-only cases are
				// handled nicely instead of failing completely
				const mediaStream = await navigator.mediaDevices.getUserMedia({
					audio : true,
					video : {
						width : {
							ideal : 1280
						},
						height : {
							ideal : 720
						},
						frameRate : {
							ideal : 60
						}
					}
				});

				sendPreview.srcObject = mediaStream;

				// And create producers for all tracks that were previously requested
				for (const track of mediaStream.getTracks())
				{
					const producer = await producerTransport.produce({ track });

					console.log(`${track.kind} producer created:`, producer);
				}

				// Producer transport will be needed to receive produced tracks
        if(shouldUseTurnServer) {
          message.consumerTransportOptions.iceTransportPolicy = "relay";
          message.consumerTransportOptions.iceServers = getIceServers();
        }
				consumerTransport = device.createRecvTransport(
					message.consumerTransportOptions
				);

				consumerTransport
					.on('connect', ({ dtlsParameters }, success) =>
					{
						// Send request to establish consumer transport connection
						send({
							action : 'ConnectConsumerTransport',
							dtlsParameters
						});
						// And wait for confirmation, but, obviously, no error handling,
						// which you should definitely have in real-world applications
						waitingForResponse.set('ConnectedConsumerTransport', () =>
						{
							success();
							console.log('Consumer transport connected');
						});
					});
				break;
			}
			case 'ProducerAdded': {
				await new Promise((resolve) =>
				{
					// Send request to consume producer
					send({
						action     : 'Consume',
						producerId : message.producerId
					});
					// And wait for confirmation, but, obviously, no error handling,
					// which you should definitely have in real-world applications
					waitingForResponse.set('Consumed', async (consumerOptions: ConsumerOptions) =>
					{
						// Once confirmation is received, corresponding consumer
						// can be created client-side
						const consumer = await (consumerTransport as Transport).consume(
							consumerOptions
						);

						console.log(`${consumer.kind} consumer created:`, consumer);

						// Consumer needs to be resumed after being created in
						// paused state (see official documentation about why:
						// https://mediasoup.org/documentation/v3/mediasoup/api/#transport-consume)
						send({
							action : 'ConsumerResume',
							id     : consumer.id as ConsumerId
						});

						participants
							.addTrack(message.participantId, message.name, message.producerId, consumer.track);
						resolve(undefined);
					});
				});
				break;
			}
			case 'ProducerRemoved': {
				participants
					.deleteTrack(message.participantId, message.producerId);

				break;
			}
			default: {
				console.error('Received unexpected message', message);
			}
		}
	};

	ws.onmessage = (message) =>
	{
		const decodedMessage: ServerMessage = JSON.parse(message.data);

		// All other messages go here and are assumed to be notifications
		// that correspond to previously sent requests
		const callback = waitingForResponse.get(decodedMessage.action);

		if (callback)
		{
			waitingForResponse.delete(decodedMessage.action);
			callback(decodedMessage);
		}
		else
		{
			// Simple hack to make sure we process all messages in order, in real-world apps
			// messages it would be useful to have messages being processed concurrently
			sequentialMessages = sequentialMessages
				.then(() =>
				{
					return onmessage(decodedMessage);
				})
				.catch((error) =>
				{
					console.error('Unexpected error during message handling:', error);
				});
		}
	};
	ws.onerror = console.error;
}
