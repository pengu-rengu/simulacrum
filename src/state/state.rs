use serde::{Serialize, Deserialize};
use crate::state::escaperoom::{EscapeRoomAgent, EscapeRoomInput, Room};
use crate::state::nodeworld::{NodeworldAgent, NodeworldInput};
pub use crate::state::escaperoom::Direction;
pub use crate::state::nodeworld::{BodyCell, BodyGrid, CombatEncounter, DepositType, Enemy, Item, ItemStack, Menu, Node, NodeworldAction, PointOfInterest, Recipe, ToolAttribute};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Universe {
    Nodeworld, EscapeRoom
}


#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub turn: usize,
    pub agent_idx: usize,
    pub agents: Vec<Agent>,
    pub nodes: Vec<Node>,
    pub rooms: Vec<Room>
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]

pub struct Agent {
    pub name: String,
    pub error_message: Option<String>,
    pub universe: Universe,
    pub nodeworld: NodeworldAgent,
    pub escape_room: EscapeRoomAgent
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub sender_agent_idx: usize,
    pub content: String
}

#[derive(Deserialize)]
pub enum Input {
    NodeworldInput(NodeworldInput),
    EscapeRoomInput(EscapeRoomInput)
}