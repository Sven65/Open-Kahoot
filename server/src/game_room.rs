use std::{collections::{HashMap, VecDeque}};
use tokio::sync::RwLock;


#[derive(serde::Serialize, Clone)]
pub struct GameRoom {
    pub id: String,
    pub host: String,
    pub players: Vec<String>
}


pub type GameRoomStore = HashMap<String, VecDeque<GameRoom>>;


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

    // pub async fn add_player_to_room (&self, room_id: &String, player: &String) {
    //     let mut binding = self.rooms.write().await;
    //     let rooms = binding.entry(room_id.clone()).or_default();
        
    //     println!("Rooms are {:#?}", );
    // }

    pub async fn insert(&self, room: GameRoom) {
        let mut binding = self.rooms.write().await;
        let rooms = binding.entry(room.id.clone()).or_default();
        rooms.push_front(room.clone());
    }
}