use std::iter::zip;

use crate::state::escaperoom::{EscapeRoomAction, EscapeRoomCell, EscapeRoomInput, Room};
use crate::state::state::{Message, State};
use crate::state::escaperoom::Direction;

fn step(x: usize, y: usize, direction: &Direction, room: &Room) -> (usize, usize) {
    let (next_x, next_y) = match direction {
        Direction::Up => (x, y.saturating_sub(1)),
        Direction::Down => (x, y + 1),
        Direction::Left => (x.saturating_sub(1), y),
        Direction::Right => (x + 1, y)
    };

    if next_x >= room.width || next_y >= room.height {
        return (x, y);
    }

    (next_x, next_y)
}

fn passable(cell: &EscapeRoomCell) -> bool {
    match cell {
        EscapeRoomCell::Empty => true,
        EscapeRoomCell::Door(door) => door.open,
        EscapeRoomCell::Wall => false
    }
}

fn escaperoom_move(room: &Room, coords: (usize, usize), direction: &Direction) -> (usize, usize) {
    let (next_x, next_y) = step(coords.0, coords.1, direction, room);
    let cell = &room.cells[next_y * room.width + next_x];
    if passable(&cell) {
        (next_x, next_y)
    } else {
        coords
    }
}

fn escaperoom_interact(room: &mut Room, coords: (usize, usize), direction: &Direction) {
    let (next_x, next_y) = step(coords.0, coords.1, direction, room);

    let door_id = {
        let EscapeRoomCell::Door(door) = &room.cells[next_y * room.width + next_x] else { return };
        door.id.clone()
    };

    for cell in &mut room.cells {
        if let EscapeRoomCell::Door(door) = cell {
            if door.id == door_id {
                door.open = !door.open;
            }
        }
    }
}

pub fn check_escaperoom_next_state(old_state: &State, new_state: &State, input: &EscapeRoomInput) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx].escape_room;
    let mut x = acting_agent.x;
    let mut y = acting_agent.y;
    let room_idx = acting_agent.room_idx;
    let mut rooms = old_state.rooms.clone();

    for action in &input.actions {
        match action {
            EscapeRoomAction::Move { direction } => {
                (x, y) = escaperoom_move(&rooms[room_idx], (x, y), direction);
            }
            EscapeRoomAction::Interact { direction } => {
                escaperoom_interact(&mut rooms[room_idx], (x, y), direction);
            }
        }
    }

    assert_eq!(new_state.rooms, rooms);
    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.escape_room.x, x);
            assert_eq!(new_agent.escape_room.y, y);
            assert_eq!(new_agent.escape_room.room_idx, old_agent.escape_room.room_idx);
        } else {
            assert_eq!(new_agent.escape_room.room_idx, old_agent.escape_room.room_idx);
            assert_eq!(new_agent.escape_room.x, old_agent.escape_room.x);
            assert_eq!(new_agent.escape_room.y, old_agent.escape_room.y);
        }
    }
    if let Some(message) = &input.send_message {
        check_send_escaperoom_message(old_state, new_state, message);
    } else {
        check_escaperoom_message_unchanged(old_state, new_state);
    }
}


pub fn check_send_escaperoom_message(old_state: &State, new_state: &State, content: &str) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        if let Some((last, rest)) = new_agent.escape_room.messages_inbox.split_last() {
            let new_message = Message {
                sender_agent_idx: old_state.agent_idx,
                content: content.to_string()
            };
            assert_eq!(*last, new_message);
            assert_eq!(rest, old_agent.escape_room.messages_inbox);
        } else {
            panic!("new messages cannot be empty")
        }
    }
}

pub fn check_escaperoom_message_unchanged(old_state: &State, new_state: &State) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        assert_eq!(new_agent.escape_room.messages_inbox, old_agent.escape_room.messages_inbox);
    }
}
