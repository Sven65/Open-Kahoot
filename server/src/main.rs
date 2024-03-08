mod state;
mod util;
mod socket_type;
mod game_room;

use axum::routing::get;
use game_room::{GameRoomStore, RoomStore};
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use util::generate_random_number_string;


#[macro_use]
extern crate lazy_static;

use crate::{game_room::GameRoom, socket_type::{SocketErrorMessage, SocketEventType}};

#[derive(Debug, serde::Deserialize)]
struct SentAnswer {
    room_id: String,
    answer: String
}


lazy_static! {
    static ref GAMEROOM_STORE: RoomStore = RoomStore::new();
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on(
        SocketEventType::Join,
        |socket: SocketRef, Data::<String>(room)| async move {
            info!("Received join {:?}", room);

            info!("Trying to to join room {:#?}. Available: {:#?}", room, socket.rooms());
            

            if (!GAMEROOM_STORE.has_room(&room).await) {
                info!("Failed to join room {:#?} as it does not exist. {:#?}", room, socket.rooms());
                let _ = socket.emit("join_failed", SocketErrorMessage {
                    error_type: SocketEventType::JoinFailed,
                    error: "Failed to join as room doesn't exist.".to_string(),
                });
                return;
            }

            let _ = socket.leave_all();
            let _ = socket.join(room.clone());

            println!("Room clients {:#?}", socket.within(room.clone()));
            
            let _ = socket.emit(SocketEventType::RoomJoined, room);
        },
    );

    socket.on(
        "hello",
        |socket: SocketRef| async move {
            info!("Got hello");
           
            socket.emit("hello", "to you too").unwrap();
        },
    );

    socket.on(SocketEventType::CreateRoom, |socket: SocketRef| async move {
        info!("Creating room");
        let room_code = util::generate_random_number_string(6);

        let _ = socket.leave_all();
        let _ = socket.join(room_code.clone());
        info!("Rooms are now {:#?}", socket.rooms());

        GAMEROOM_STORE.insert(GameRoom {
            id: room_code.clone(),
            host: socket.id.to_string(),
            players: vec![]
        }).await;


        socket.emit(SocketEventType::RoomCreated, room_code).unwrap();
    });

    socket.on(SocketEventType::SendAnswer, |socket: SocketRef, Data::<SentAnswer>(data)| async move {
        // TODO: Check if player is in a room
        let answer = data.answer;
        let room = data.room_id;

        info!("Got answer {} for room id {}", answer, room);

        let _ = socket.emit(SocketEventType::SendPoints, 100);
    });

    socket.on(SocketEventType::ShowQuestion, |socket: SocketRef, Data::<String>(room)| async move {
        // TODO: Host check
        let _ = socket.to(room).emit(SocketEventType::ShowQuestion, "");
    });

    socket.on(SocketEventType::HideQuestion, |socket: SocketRef, Data::<String>(room)| async move {
        // TODO: Host check

        let _ = socket.to(room).emit(SocketEventType::HideQuestion, "");
    });
}

async fn handler(axum::extract::State(io): axum::extract::State<SocketIo>) {
    info!("handler called");
    let _ = io.emit("hello", "world");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/hello", get(handler))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer)
        );

    info!("Starting server on port {}", "3000");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
