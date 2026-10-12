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
        EscapeRoomCell::Exit(_) => true,
        EscapeRoomCell::Spawn(_) => true
    }
}

fn escaperoom_move(agent: &EscapeRoomAgent, rooms: &[Room], direction: &Direction) -> EscapeRoomAgent {
    let room = &rooms[agent.room_idx];
    let (next_x, next_y) = step(agent.x, agent.y, direction, room);
    let cell = &room.cells[next_y * room.width + next_x];

    let mut expected_agent = agent.clone();
    if passable(&cell) {
        expected_agent.x = next_x;
        expected_agent.y = next_y;
    }
    expected_agent
}

fn escaperoom_interact(agent: &EscapeRoomAgent, rooms: &[Room], acting_agent_idx: usize, n_agents: usize, direction: &Direction) -> (EscapeRoomAgent, Vec<Room>) {
    let mut expected_agent = agent.clone();
    let mut expected_rooms = rooms.to_vec();
    let room_idx = agent.room_idx;
    let room = &rooms[room_idx];
    let expected_room = &mut expected_rooms[room_idx];
    let (next_x, next_y) = step(agent.x, agent.y, direction, room);
    

    match &room.cells[next_y * room.width + next_x] {
        EscapeRoomCell::Door(door) => {
            let door_id = &door.id;
            for cell in &mut expected_room.cells {
                if let EscapeRoomCell::Door(door) = cell {
                    if door.id == door_id.clone() {
                        door.open = !door.open;
                    }
                }
            }
        }
        EscapeRoomCell::Exit(agent_idxs) => {
            if agent_idxs.len() == n_agents {
                if rooms.get(room_idx + 1).is_some() {
                    let next_room = rooms[room_idx + 1].clone();
                    expected_agent.room_idx += 1;
                    let mut found_spawn = false;
                    for (i, cell) in next_room.cells.iter().enumerate() {
                        if let EscapeRoomCell::Spawn(spawn_agent_idx) = cell
                        && *spawn_agent_idx == acting_agent_idx {
                            expected_agent.x = i % next_room.width;
                            expected_agent.y = i / next_room.width;
                            found_spawn = true;
                            break;
                        }
                    }
                    if !found_spawn { panic!("spawn not found"); }
                } else {
                    expected_agent.finished = true;
                }
            } else {
                for cell in &mut expected_room.cells {
                    if let EscapeRoomCell::Exit(agent_idxs) = cell {
                        agent_idxs.insert(acting_agent_idx);
                    }
                }
            }
            
        }
        _ => {}
    }

    (expected_agent, expected_rooms)
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

    let mut expected_rooms = old_state.rooms.clone();

    for action in &input.actions {
        match action {
            EscapeRoomAction::Move { direction } => {
                expected_agent = escaperoom_move(&expected_agent, &expected_rooms, direction);
            }
            EscapeRoomAction::Interact { direction } => {
                (expected_agent, expected_rooms) = escaperoom_interact(&expected_agent, &expected_rooms, acting_agent_idx, old_state.agents.len(), direction);
            }
        }
    }

    assert_eq!(new_state.rooms, expected_rooms);
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
