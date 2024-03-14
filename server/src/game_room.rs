use std::collections::HashMap;
use tokio::sync::RwLock;
use std::time::Instant;

use crate::{api::quiz_types::ReturnedQuestion, db::models::RealAnswerColor, player::Player};


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
    pub max_time: f64,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct GameState {
    pub show_question: bool,
    pub current_question_id: String,
    pub is_game_over: bool,
    #[serde(with = "serde_millis")]
    pub question_started: Option<Instant>,
    pub answer_count: usize,
    pub client_state: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct GameRoom {
    pub id: String,
    pub host: String,
    pub state: GameState,
    pub players: HashMap<String, Player>,
    pub questions: Vec<ReturnedQuestion>,
}


pub type GameRoomStore = HashMap<String, GameRoom>;


impl GameRoom {
    pub fn get_current_question (&self) -> Option<&ReturnedQuestion> {
        self.questions.iter().find(|item| {
            item.id == Some(self.state.current_question_id.clone())
        })
    }

    pub fn get_next_question_id (&self) -> Option<String> {
        let current_idx = self.questions.iter().position(|item| {
            item.id == Some(self.state.current_question_id.clone())
        });

        if current_idx.is_none() {
            return None
        }

        let current_idx = current_idx.unwrap();
        let next_question = self.questions.get(current_idx + 1);

        if let Some(next_question) = next_question {
            return next_question.id.clone()
        } else {
            return None
        }
    }

    pub fn set_current_question_id (&mut self, id: String) {
        self.state.current_question_id = id;
    }

    pub fn prepare_next_question(&mut self) {

        if let Some(next_question_id) = self.get_next_question_id() {
            self.set_current_question_id(next_question_id.clone());

            if next_question_id == crate::LAST_QUESTION_ID {
                self.state.is_game_over = true;
            }
        } else {
            self.state.is_game_over = true;
        }
    }

    pub fn get_player(&self, id: String) -> Option<&Player> {
       self.players.get(&id)
    }

    pub fn get_player_mut(&mut self, id: String) -> Option<&mut Player> {
        self.players.get_mut(&id)

    }

    pub fn insert_player(&mut self, player: Player) {
        self.players.insert(player.clone().id, player);
    }

    pub fn remove_player(&mut self, player_id: String) {
        self.players.remove(&player_id);
    }

    pub fn get_players_sorted_by_score(&self) -> Vec<Player> {
        let cloned_players = &mut self.players.clone();
        let mut players_vec: Vec<Player> = cloned_players.drain().map(|(_, player)| player).collect();
        
        players_vec.sort_by(|a, b| {
           b.points.total_cmp(&a.points)
        });

        players_vec
    }

    pub fn has_all_players_answered(&self) -> bool {
        self.players.iter().all(|player| player.1.has_answered)
    }

    pub fn set_client_state(&mut self, state: String) {
        self.state.client_state = state;
    }

    pub fn reset_answers(&mut self) {
        let players = self.players.clone();

        for (_, mut player) in players {
            player.has_answered = false;
            self.insert_player(player);
        }
    }

    pub fn count_answer_colors(&self) -> HashMap<RealAnswerColor, i32> {
        let mut color_counts: HashMap<RealAnswerColor, i32> = HashMap::new();

        for player in self.players.values() {
            let color = self.answer_id_to_color(&player.answer_id.clone().unwrap());
            *color_counts.entry(color).or_insert(0) += 1;
        }

        color_counts
    }

    pub fn answer_id_to_color(&self, answer_id: &String) -> RealAnswerColor {
        let index = answer_id.chars().fold(0, |acc, c| acc + c as usize) % 4; // Assuming 4 colors
        match index {
            0 => RealAnswerColor::Red,
            1 => RealAnswerColor::Green,
            2 => RealAnswerColor::Blue,
            3 => RealAnswerColor::Yellow,
            _ => unreachable!(), // This should never happen
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

    pub async fn get_room_clone(&self, room: &String) -> Option<GameRoom> {
        // Acquire a read lock on self.rooms
        let rooms_guard = self.rooms.read().await;

        // Access the GameRoom if it exists
        rooms_guard.get(room).cloned()
    }

    pub async fn get_player_rooms_cloned(&self, player_id: String) -> Option<GameRoom> {
        let rooms_guard = self.rooms.read().await;
        let cloned_rooms = rooms_guard.clone();


        if let Some(found_room) = cloned_rooms.values().find(|room| room.players.contains_key(&player_id)) {
            return Some(found_room.clone());
        } else {
            let room = cloned_rooms.values().find(|room| room.host == player_id);

            return room.cloned();
        }

    }


    pub async fn insert(&self, room: GameRoom) {
        let mut rooms = self.rooms.write().await;
        rooms.insert(room.id.clone(), room);
    }

    pub async fn remove(&self, room: GameRoom) {
        let mut rooms = self.rooms.write().await;
        rooms.remove(&room.id);
    }
}