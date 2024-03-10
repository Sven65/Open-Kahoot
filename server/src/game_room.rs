use std::{collections::{HashMap, VecDeque}, sync::Arc};
use tokio::sync::{RwLock};
use tracing::info;


#[derive(serde::Serialize, Clone, Debug)]
pub struct Answer {
    pub id: String,
    pub answer: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Question {
    pub id: String,
    pub question: String,
    pub answers: Vec<Answer>,
    pub correct_answer_id: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct GameState {
    pub show_question: bool,
    pub current_question_id: String,
    pub is_game_over: bool,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct GameRoom {
    pub id: String,
    pub host: String,
    pub state: GameState,
    pub players: Vec<String>,
    pub questions: Vec<Question>,
}


pub type GameRoomStore = HashMap<String, GameRoom>;

impl GameRoom {
    pub fn get_current_question (&self) -> Option<&Question> {
        self.questions.iter().find(|item| {
            item.id == self.state.current_question_id
        })
    }

    pub fn get_next_question_id (&self) -> Option<String> {
        let current_idx = self.questions.iter().position(|item| {
            item.id == self.state.current_question_id
        });

        if current_idx.is_none() {
            return None
        }

        let current_idx = current_idx.unwrap();
        let next_question = self.questions.get(current_idx + 1);

        if let Some(next_question) = next_question {
            return Some(next_question.id.clone())
        } else {
            return None
        }
    }

    pub fn set_current_question_id (&mut self, id: String) {
        info!("Setting current q id to {}", id);

        self.state.current_question_id = id;
    }

    pub fn prepare_next_question(&mut self) {
        info!("Prepping next question");

        if let Some(next_question_id) = self.get_next_question_id() {
            self.set_current_question_id(next_question_id);
        } else {
            info!("Setting game over");
            self.state.is_game_over = true;
        }
    }
}

#[derive(Default)]
pub struct RoomStore {
    pub rooms: RwLock<GameRoomStore>,
}

impl RoomStore {
    pub fn new () -> Self {
        Self {
            rooms: RwLock::new(GameRoomStore::new())
        }
    }

    pub async fn has_room (&self, room: &String) -> bool {
        let rooms = self.rooms.read().await;

        rooms.contains_key(room)
    }

    pub async fn get_room_clone(&self, room: &String) -> Option<GameRoom> {
        // Acquire a read lock on self.rooms
        let rooms_guard = self.rooms.read().await;

        // Access the GameRoom if it exists
        rooms_guard.get(room).cloned()
    }

    // pub async fn add_player_to_room (&self, room_id: &String, player: &String) {
    //     let mut binding = self.rooms.write().await;
    //     let rooms = binding.entry(room_id.clone()).or_default();
        
    //     println!("Rooms are {:#?}", );
    // }

    pub async fn insert(&self, room: GameRoom) {
        let mut rooms = self.rooms.write().await;
        rooms.insert(room.id.clone(), room);
    }
}