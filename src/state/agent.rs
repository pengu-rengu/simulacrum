use serde::{Serialize, Deserialize};
use crate::state::state::{Message, BodyGrid, ItemStack, Menu};

#[derive(Serialize, Deserialize, Clone)]

pub struct Agent {
    pub name: String,
    pub nodeworld: NodeworldAgent
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
