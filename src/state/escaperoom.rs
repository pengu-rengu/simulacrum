use serde::{Serialize, Deserialize};
use crate::state::state::Direction;

#[derive(Serialize, Deserialize, Clone)]
pub struct DoorCell {
    pub id: String,
    pub open: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EscapeRoomCell {
    Empty, Wall, Door(DoorCell)
}

#[derive(Serialize, Deserialize, Clone)]
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
    pub action: Option<EscapeRoomAction>,
    pub send_message: Option<String>
}