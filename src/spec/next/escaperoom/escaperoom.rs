use std::iter::zip;

use crate::state::agent::EscapeRoomAgent;
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
        EscapeRoomCell::Wall => false,
        EscapeRoomCell::Exit => false,
        EscapeRoomCell::Spawn(_) => true
    }
}

fn escaperoom_move(agent: &mut EscapeRoomAgent, rooms: &[Room], direction: &Direction) {
    let room = &rooms[agent.room_idx];
    let (next_x, next_y) = step(agent.x, agent.y, direction, room);
    let cell = &room.cells[next_y * room.width + next_x];
    if passable(&cell) {
        agent.x = next_x;
        agent.y = next_y;
    }
}

fn escaperoom_interact(agent: &mut EscapeRoomAgent, acting_agent_idx: usize, rooms: &mut [Room], direction: &Direction) {
    let room_idx = agent.room_idx;
    let faced = {
        let room = &rooms[room_idx];
        let (next_x, next_y) = step(agent.x, agent.y, direction, room);
        room.cells[next_y * room.width + next_x].clone()
    };

    match faced {
        EscapeRoomCell::Door(door) => {
            let door_id = door.id;
            for cell in &mut rooms[room_idx].cells {
                if let EscapeRoomCell::Door(door) = cell {
                    if door.id == door_id {
                        door.open = !door.open;
                    }
                }
            }
        }
        EscapeRoomCell::Exit => {
            if rooms.get(room_idx + 1).is_some() {
                let next_room = rooms[room_idx + 1].clone();
                agent.room_idx += 1;
                let mut found_spawn = false;
                for (i, cell) in next_room.cells.iter().enumerate() {
                    if let EscapeRoomCell::Spawn(spawn_agent_idx) = cell
                    && *spawn_agent_idx == acting_agent_idx {
                        agent.x = i % next_room.width;
                        agent.y = i / next_room.width;
                        found_spawn = true;
                        break;
                    }
                }
                if !found_spawn { panic!("spawn not found"); }
            } else {
                agent.finished = true;
            }
        }
        _ => {}
    }
}

pub fn check_escaperoom_next_state(old_state: &State, new_state: &State, input: &EscapeRoomInput) {
    let acting_agent_idx = old_state.agent_idx;
    let mut expected_agent = old_state.agents[acting_agent_idx].escape_room.clone();
    if expected_agent.finished {
        for (new_agent, old_agent) in zip(&new_state.agents, &old_state.agents) {
            assert_eq!(new_agent.escape_room, old_agent.escape_room);
        }
        return;
    }

    let mut rooms = old_state.rooms.clone();

    for action in &input.actions {
        match action {
            EscapeRoomAction::Move { direction } => {
                escaperoom_move(&mut expected_agent, &rooms, direction);
            }
            EscapeRoomAction::Interact { direction } => {
                escaperoom_interact(&mut expected_agent, acting_agent_idx, &mut rooms, direction);
            }
        }
    }

    assert_eq!(new_state.rooms, rooms);
    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        let expected = if i == acting_agent_idx { &expected_agent } else { &old_agent.escape_room };
        assert_eq!(new_agent.escape_room.room_idx, expected.room_idx);
        assert_eq!(new_agent.escape_room.x, expected.x);
        assert_eq!(new_agent.escape_room.y, expected.y);
        assert_eq!(new_agent.escape_room.finished, expected.finished);
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
