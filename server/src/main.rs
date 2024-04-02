mod util;
mod middleware;
mod game_room;
mod player;
mod db;
mod api;
mod app_state;
mod socket;
mod email;

use std::sync::Arc;

use dotenvy::dotenv;
use game_room::RoomStore;
use socketioxide::{
    extract::SocketRef,
    SocketIo
};

use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use crate::{app_state::AppState, socket::{change_state::change_state, create_room::create_room, get_scores::get_scores, join::join, next_question::next_question, on_disconnect::on_disconnect, send_answer::send_answer}};


#[macro_use]
extern crate lazy_static;

use crate::socket::socket_type::SocketEventType;

const LAST_QUESTION_ID: &str = "LAST-QUESTION";
const FIRST_QUESTION_ID: &str = "FIRST-QUESTION";
const PLAYER_NAME_LENGTH_LIMIT: usize = 24;

lazy_static! {
    static ref GAMEROOM_STORE: RoomStore = RoomStore::new();
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on_disconnect(on_disconnect);
    socket.on(SocketEventType::Join, join);
    socket.on(SocketEventType::CreateRoom, create_room);
    socket.on(SocketEventType::SendAnswer, send_answer);
    socket.on(SocketEventType::NextQuestion, next_question);
    socket.on(SocketEventType::GetScores, get_scores);
    socket.on(SocketEventType::ChangeState, change_state);

    // TODO: Logic for when socket disconnects (Remove as player, remove room if socket is host)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    dotenv().ok();

    let state = Arc::new(AppState::new().await);
    let socket_state = Arc::clone(&state);

    let (layer, io) = SocketIo::builder().with_state(socket_state).build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .with_state(io)
        .nest("/api", api::api_router(Arc::clone(&state)))
        .layer(
            ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer)
        )
        .layer(axum::middleware::from_fn_with_state(Arc::clone(&state), crate::middleware::auth_session));

    info!("Starting server on port {}", "3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
