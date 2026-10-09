use serde::{Serialize, Deserialize};
use crate::state::state::{BodyGrid, ItemStack, Menu, Message, Universe};

#[derive(Serialize, Deserialize, Clone)]

pub struct Agent {
    pub name: String,
    pub error_message: Option<String>,
    pub universe: Universe,
    pub nodeworld: NodeworldAgent,
    pub escape_room: EscapeRoomAgent
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct NodeworldAgent {
    pub node_idx: usize,
    pub node_messages_inbox: Vec<Message>,
    pub body_grid: BodyGrid,
    pub inventory: Vec<ItemStack>,
    pub open_menu: Option<Menu>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EscapeRoomAgent {
    pub room_idx: usize,
    pub x: usize,
    pub y: usize,
    pub messages_inbox: Vec<Message>
}
