mod util;
mod socket_type;
mod game_room;
mod player;

use std::{collections::HashMap, time::Instant};

use axum::routing::get;
use game_room::{Answer, Question, RoomStore};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;


#[macro_use]
extern crate lazy_static;

use crate::{game_room::{GameRoom, GameState}, player::{calculate_points, Player}, socket_type::{SocketErrorMessage, SocketEventType}};

#[derive(Debug, serde::Deserialize)]
struct SentInAnswer {
    room_id: String,
    answer: String
}

#[derive(Debug, serde::Serialize)]
struct PointsOutMessage {
    pub points: f64,
    pub time_taken: f64,
}

#[derive(Debug, serde::Serialize)]
struct QuestionOut {
    pub question: Question,
    pub max_time: f64,
}

#[derive(Debug, serde::Deserialize)]
struct JoinMessage {
    pub room_id: String,
    pub name: String,
}

lazy_static! {
    static ref GAMEROOM_STORE: RoomStore = RoomStore::new();
    static ref QUESTION_TIME: f64 = 30.0;
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on(
        SocketEventType::Join,
        |socket: SocketRef, Data::<JoinMessage>(data)| async move {
            info!("Received join {:?}", data.room_id);

            info!("Trying to to join room {:#?}. Available: {:#?}", data.room_id, socket.rooms());
            
            let room = GAMEROOM_STORE.get_room_clone(&data.room_id).await;

            if room.is_none() {
                info!("Failed to join room {:#?} as it does not exist. {:#?}", data.room_id, socket.rooms());
                let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {
                    error_type: SocketEventType::JoinFailed,
                    error: "Failed to join as room doesn't exist.".to_string(),
                });
                return;
            }

            let room = &mut room.unwrap();

            room.insert_player(Player {
                id: socket.id.to_string(),
                points: 0.0,
                name: Some(data.name),
            });

            GAMEROOM_STORE.insert(room.clone()).await;

            let _ = socket.leave_all();
            let _ = socket.join(data.room_id.clone());

            println!("Room clients {:#?}", socket.within(data.room_id.clone()));
            
            let _ = socket.emit(SocketEventType::RoomJoined, data.room_id);
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
            players: HashMap::new(),
            state: GameState {
                show_question: false,
                current_question_id: "starter-question".to_string(),
                is_game_over: false,
                question_started: None,
            },
            questions: vec![
                Question {
                    answers: vec![],
                    correct_answer_id: "".to_string(),
                    question: "This should never be shown".to_string(),
                    id: "starter-question".to_string(),
                    max_time: 30.0,
                },
                Question {
                    answers: vec![
                        Answer {
                            answer: "One".to_string(),
                            id: "1".to_string()
                        },
                        Answer {
                            answer: "Two".to_string(),
                            id: "2".to_string()
                        },
                        Answer {
                            answer: "Three".to_string(),
                            id: "3".to_string()
                        },
                        Answer {
                            answer: "Four".to_string(),
                            id: "4".to_string()
                        },
                    ],
                    correct_answer_id: "1".to_string(),
                    id: "q-1".to_string(),
                    question: "What is the answer?".to_string(),
                    max_time: 30.0,
                },
                Question {
                    answers: vec![
                        Answer {
                            answer: "Five".to_string(),
                            id: "5".to_string()
                        },
                        Answer {
                            answer: "Six".to_string(),
                            id: "6".to_string()
                        },
                        Answer {
                            answer: "Seven".to_string(),
                            id: "7".to_string()
                        },
                        Answer {
                            answer: "Eight".to_string(),
                            id: "8".to_string()
                        },
                    ],
                    correct_answer_id: "5".to_string(),
                    id: "q-2".to_string(),
                    question: "What is the second answer?".to_string(),
                    max_time: 30.0,
                }
            ]
        }).await;


        socket.emit(SocketEventType::RoomCreated, room_code).unwrap();
    });

    socket.on(SocketEventType::SendAnswer, |socket: SocketRef, Data::<SentInAnswer>(data)| async move {
        // TODO: Check if player is in a room
        let answer = data.answer;
        let room_id = data.room_id;

        info!("Got answer {} for room id {}", answer, room_id);

        if let Some(mut room) = GAMEROOM_STORE.get_room_clone(&room_id).await {
            let question_started = room.state.question_started.unwrap(); // Clone the field
            info!("question started at {:#?}", question_started);
            let question = room.get_current_question().unwrap();
            info!("Question is {:#?}", question);
            if answer == question.correct_answer_id {
                let question_clone = question.clone();
                if let Some(player) = room.get_player_mut(socket.id.to_string()) {
                    info!("Player {:#?}", player);
                    let duration = question_started.elapsed(); // Use the cloned field
                    let points = calculate_points(duration.as_secs_f64(), question_clone.max_time, 1000.0);
                    player.add_points(points);
    
                    // Insert the modified room back into the store
                    GAMEROOM_STORE.insert(room.clone()).await;
    
                    let _ = socket.emit(SocketEventType::SendPoints, PointsOutMessage {
                        points: points,
                        time_taken: duration.as_secs_f64(),
                    });
                } else {
                    let _ = socket.emit(SocketEventType::SendPoints, 0);
                }
            } else {
                info!("Sent in answer doesn't match");
            }
        }

    });

    socket.on(SocketEventType::HideQuestion, |socket: SocketRef, Data::<String>(room)| async move {
        // TODO: Host check

        let _ = socket.to(room).emit(SocketEventType::HideQuestion, "");
    });

    socket.on(SocketEventType::NextQuestion, |socket: SocketRef, Data::<String>(room_id)| async move {
        // TODO: Host check

        let room = GAMEROOM_STORE.get_room_clone(&room_id).await;

        if let Some(mut room) = room {
            room.prepare_next_question();

            let question = room.get_current_question();

            let _ = socket.emit(SocketEventType::SendQuestion, question.clone());
            let _ = socket.to(room_id.clone()).emit(SocketEventType::SendQuestion, question);
            let _ = socket.to(room_id.clone()).emit(SocketEventType::ShowQuestion, "");
            room.state.question_started = Some(Instant::now());

            GAMEROOM_STORE.insert(room.clone()).await;

            if room.state.is_game_over {
                let _ = socket.emit(SocketEventType::GameOver, "");
                let _ = socket.to(room_id).emit(SocketEventType::GameOver, "");
            }
        } else {
            let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Room doesn't exist.".to_string(), error_type: SocketEventType::NextQuestion });
        }
    });

    socket.on(SocketEventType::GetScores, |socket: SocketRef, Data::<String>(room_id)| async move {
        if let Some(room) = GAMEROOM_STORE.get_room_clone(&room_id).await {
            let scores = room.get_players_sorted_by_score();
            let _ = socket.emit(SocketEventType::GetScores, (scores,));
        } else {
            let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Room doesn't exist.".to_string(), error_type: SocketEventType::GetScores });
        }
    });


    // TODO: Logic for when socket disconnects (Remove as player, remove room if socket is host)
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
