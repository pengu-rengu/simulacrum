use serde::{Serialize, Deserialize};
use crate::state::state::{Message, BodyGrid, ItemStack, Menu};

#[derive(Serialize, Deserialize, Clone)]

pub struct Agent {
    pub name: String,
    pub nodeworld: NodeworldAgent,
    pub escape_room: EscapeRoomAgent
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NodeworldAgent {
    pub node_idx: usize,
    pub node_messages_inbox: Vec<Message>,
    pub error_message: Option<String>,
    pub body_grid: BodyGrid,
    pub inventory: Vec<ItemStack>,
    pub open_menu: Option<Menu>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EscapeRoomAgent {
    pub room_idx: usize,
    pub x: usize,
    pub y: usize
}
