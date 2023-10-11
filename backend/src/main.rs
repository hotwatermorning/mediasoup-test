use actix_web::web::{Data, Payload, Query};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use mediasoup::prelude::*;
use serde::Deserialize;
use std::net::IpAddr;

mod participant;
mod recording;
mod room;
mod rooms_registry;
mod util;

// use participant::ParticipantConnection;
// use room::RoomId;
// use rooms_registry::RoomsRegistry;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QueryParameters {
    room_id: Option<room::RoomId>,
}

/// Function that receives HTTP request on WebSocket route and upgrades it to WebSocket connection.
///
/// See https://actix.rs/docs/websockets/ for official `actix-web` documentation.
async fn ws_index(
    query_parameters: Query<QueryParameters>,
    request: HttpRequest,
    worker_manager: Data<WorkerManager>,
    rooms_registry: Data<rooms_registry::RoomsRegistry>,
    stream: Payload,
) -> Result<HttpResponse, Error> {
    // WebSocket のクエリにルーム ID が含まれている
    let room = match query_parameters.room_id {
        Some(room_id) => {
            rooms_registry
                .get_or_create_room(&worker_manager, room_id)
                .await
        }
        None => rooms_registry.create_room(&worker_manager).await,
    };

    let room = match room {
        Ok(room) => room,
        Err(error) => {
            eprintln!("{error}");

            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    match participant::ParticipantConnection::new(room).await {
        Ok(echo_server) => ws::start(echo_server, &request, stream),
        Err(error) => {
            eprintln!("{error}");

            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.local").ok();
    dotenv::dotenv().ok(); // load if .env file exist

    env_logger::init();

    let addr = util::get_env::<IpAddr>("LISTEN_IP").unwrap();
    let port = util::get_env::<u16>("PORT").unwrap();

    // We will reuse the same worker manager across all connections, this is more than enough for
    // this use case
    let worker_manager = Data::new(WorkerManager::new());
    // Rooms registry will hold all the active rooms
    let rooms_registry = Data::new(rooms_registry::RoomsRegistry::default());

    log::info!("Listening on {}:{}", addr, port);

    HttpServer::new(move || {
        App::new()
            .app_data(worker_manager.clone())
            .app_data(rooms_registry.clone())
            .route("/ws", web::get().to(ws_index))
    })
    // 2 threads is plenty for this example, default is to have as many threads as CPU cores
    .workers(2)
    .bind(format!("{}:{}", addr, port))?
    .run()
    .await
}
