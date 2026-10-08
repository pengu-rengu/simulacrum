use serde::{Serialize, Deserialize};
use crate::state::state::Direction;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DoorCell {
    pub id: String,
    pub open: bool
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EscapeRoomCell {
    Empty, Wall, Door(DoorCell)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Room {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub cells: Vec<EscapeRoomCell>
}

#[derive(Deserialize)]
pub enum EscapeRoomAction {
    Move{ direction: Direction }, Interact { direction: Direction }
}

#[derive(Deserialize)]
pub struct EscapeRoomInput {
    pub actions: Vec<EscapeRoomAction>,
    pub send_message: Option<String>
}