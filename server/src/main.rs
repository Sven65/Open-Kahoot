mod util;
mod socket_type;
mod game_room;
mod player;
mod db;
mod api;

use std::{collections::HashMap, time::Instant};

use chrono::Utc;
use game_room::{Question, RoomStore};
use socketioxide::{
    extract::{Data, SocketRef}, socket::DisconnectReason, SocketIo
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;


#[macro_use]
extern crate lazy_static;

use crate::{api::quiz::{get_quiz_by_id, ReturnedQuestion}, db::establish_connection, game_room::{GameRoom, GameState}, player::{calculate_points, Player}, socket_type::{SocketErrorMessage, SocketEventType}};

#[derive(Debug, serde::Deserialize)]
struct SentInAnswer {
    room_id: String,
    answer: String
}

#[derive(Debug, serde::Serialize)]
struct PointsOutMessage {
    pub points: f32,
    pub time_taken: f32,
}

#[derive(Debug, serde::Serialize)]
struct QuestionOut {
    pub question: Question,
    pub max_time: f32,
}

#[derive(Debug, serde::Deserialize)]
struct JoinMessage {
    pub room_id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
struct ChangeStateMessage {
    pub room_id: String,
    pub state: String,
}

const LAST_QUESTION_ID: &str = "LAST-QUESTION";
const FIRST_QUESTION_ID: &str = "FIRST-QUESTION";

lazy_static! {
    static ref GAMEROOM_STORE: RoomStore = RoomStore::new();
    static ref CURRENT_QUIZ_ID: String = "gfnfsogns".to_string();
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);


    socket.on_disconnect(|socket: SocketRef, reason: DisconnectReason| async move {
        info!("Disconnected socket was in rooms {:#?}, because {:#?}", socket.rooms(), reason);
        let player_id = socket.id.to_string();

        if let Some(mut room) = GAMEROOM_STORE.get_player_rooms_cloned(player_id.clone()).await {
            let cloned_room = room.clone();

            if room.host == player_id {

                GAMEROOM_STORE.remove(cloned_room).await;
                
                let _ = socket.to(room.id.clone()).emit(SocketEventType::RoomClosed, "Host left");

                return;
            }

            let player = cloned_room.get_player(player_id.clone());
            room.remove_player(player_id.clone());

            let _ = socket.to(room.id.clone()).emit(SocketEventType::PlayerLeft, player.unwrap());

            GAMEROOM_STORE.insert(room).await;
        } else {
            info!("No room found for disconnected socket.");
        }
    });

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

            let player = Player {
                id: socket.id.to_string(),
                points: 0.0,
                name: Some(data.name),
            };
            
            room.insert_player(player.clone());

            GAMEROOM_STORE.insert(room.clone()).await;

            let _ = socket.leave_all();
            let _ = socket.join(data.room_id.clone());

            println!("Room clients {:#?}", socket.within(data.room_id.clone()));
            
            let _ = socket.emit(SocketEventType::RoomJoined, data.room_id.clone());
            let _ = socket.to(data.room_id.clone()).emit(SocketEventType::PlayerJoined, player);
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

        let mut conn = establish_connection();

        let quiz = get_quiz_by_id(CURRENT_QUIZ_ID.to_string(), &mut conn).await;

        if quiz.is_err() {
            let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Tried to load a quiz that doesn't exist.".to_string(), error_type: SocketEventType::CreateRoom });
            return
        }

        let quiz = quiz.unwrap();

        let mut questions: Vec<ReturnedQuestion> = vec![
            ReturnedQuestion {
                answers: vec![],
                correct_answer_id: Some("0".to_string()),
                question: "This should never be shown".to_string(),
                id: FIRST_QUESTION_ID.to_string(),
                max_time: 30.0,
                max_points: 1000.0,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
                question_rank: 0,
                quiz_id: CURRENT_QUIZ_ID.to_string(),
            }
        ];

        questions.append(&mut quiz.questions.clone());

        questions.push(ReturnedQuestion {
            answers: vec![],
            correct_answer_id: Some("0".to_string()),
            question: "This should never be shown.".to_string(),
            id: LAST_QUESTION_ID.to_string(),
            max_time: 30.0,
            max_points: 1000.0,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            question_rank: std::i32::MAX,
            quiz_id: CURRENT_QUIZ_ID.to_string(),
        });

        GAMEROOM_STORE.insert(GameRoom {
            id: room_code.clone(),
            host: socket.id.to_string(),
            players: HashMap::new(),
            state: GameState {
                show_question: false,
                current_question_id: FIRST_QUESTION_ID.to_string(),
                is_game_over: false,
                question_started: None,
                answer_count: 0,
                client_state: "UNKNOWN".to_string(),
            },
            questions: questions,
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
            if question.correct_answer_id.is_none() {
                let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Question does not have a correct answer.".to_string(), error_type: SocketEventType::SendAnswer });
                return
            }

            if answer == question.correct_answer_id.clone().unwrap() {
                let question_clone = question.clone();
                if let Some(player) = room.get_player_mut(socket.id.to_string()) {
                    info!("Player {:#?}", player);
                    let duration = question_started.elapsed(); // Use the cloned field
                    let points = calculate_points(duration.as_secs_f32(), question_clone.max_time, 1000.0);
                    player.add_points(points);

                    room.add_answer_count(1);
    
                    // Insert the modified room back into the store
                    GAMEROOM_STORE.insert(room.clone()).await;
    
                    let _ = socket.emit(SocketEventType::SendPoints, PointsOutMessage {
                        points: points,
                        time_taken: duration.as_secs_f32(),
                    });

                    if room.has_all_players_answered() {
                        room.set_answer_count(0);
                        GAMEROOM_STORE.insert(room.clone()).await;

                        let scores = room.get_players_sorted_by_score();
                        let _ = socket.to(room.id).emit(SocketEventType::GetScores, (scores,));
                    }
                } else {
                    let _ = socket.emit(SocketEventType::SendPoints, 0);
                }
            } else {
                room.add_answer_count(1);

                if room.has_all_players_answered() {
                    room.set_answer_count(0);
                    GAMEROOM_STORE.insert(room.clone()).await;

                    let scores = room.get_players_sorted_by_score();
                    let _ = socket.to(room.id).emit(SocketEventType::GetScores, (scores,));
                }

                info!("Sent in answer doesn't match");
            }
        }

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
                let _ = socket.emit(SocketEventType::ChangeState, "ENDED");
                let _ = socket.to(room_id).emit(SocketEventType::ChangeState, "ENDED");

                let scores = room.get_players_sorted_by_score();
                let _ = socket.emit(SocketEventType::GetScores, (scores,));
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

    socket.on(SocketEventType::ChangeState, |socket: SocketRef, Data::<ChangeStateMessage>(data)| async move {
        // TODO: Host check

        info!("Changing state for socket");

        if let Some(mut room) = GAMEROOM_STORE.get_room_clone(&data.room_id).await {
            room.set_client_state(data.state.clone());

            GAMEROOM_STORE.insert(room).await;

            let _ = socket.emit(SocketEventType::ChangeState, data.state.clone());
            let _ = socket.to(data.room_id).emit(SocketEventType::ChangeState, data.state);
        } else {
            info!("No room exists");
            let _ = socket.emit(SocketEventType::Error, SocketErrorMessage {error: "Room doesn't exist.".to_string(), error_type: SocketEventType::ChangeState });
        }
    });

    // TODO: Logic for when socket disconnects (Remove as player, remove room if socket is host)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::builder().build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .with_state(io)
        .nest("/api", api::api_router())
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
