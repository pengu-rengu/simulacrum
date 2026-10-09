use serde::{Serialize, Deserialize};
use crate::state::state::Message;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DoorCell {
    pub id: String,
    pub open: bool
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EscapeRoomCell {
    Empty, Wall, Door(DoorCell), Exit, Spawn(usize)
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


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EscapeRoomAgent {
    pub room_idx: usize,
    pub x: usize,
    pub y: usize,
    pub messages_inbox: Vec<Message>,
    pub finished: bool
}


#[derive(Deserialize)]
pub enum Direction { Up, Down, Left, Right }