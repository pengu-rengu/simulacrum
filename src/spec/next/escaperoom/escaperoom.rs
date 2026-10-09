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
    let maybe_next_room = rooms.get(agent.room_idx + 1)
                                            .map(|room| room.clone());

    let room = &mut rooms[agent.room_idx];
    let (next_x, next_y) = step(agent.x, agent.y, direction, room);

    let door_id = {
        let EscapeRoomCell::Door(door) = &room.cells[next_y * room.width + next_x] else { return };
        door.id.clone()
    };

    for cell in &mut room.cells {
        match cell {
            EscapeRoomCell::Door(door) => {
                if door.id == door_id {
                    door.open = !door.open;
                }
            }
            EscapeRoomCell::Exit => {
                if let Some(next_room) = &maybe_next_room {
                    agent.room_idx += 1;

                    let mut found_spawn = false;
                    for (i, cell) in next_room.cells.iter().enumerate() {
                        if let EscapeRoomCell::Spawn(agent_idx) = cell
                        && *agent_idx == acting_agent_idx {
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
}

pub fn check_escaperoom_next_state(old_state: &State, new_state: &State, input: &EscapeRoomInput) {
    let acting_agent_idx = old_state.agent_idx;
    let mut expected_agent = old_state.agents[acting_agent_idx].escape_room.clone();
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
        if i == acting_agent_idx {
            assert_eq!(new_agent.escape_room, expected_agent);
        } else {
            assert_eq!(new_agent.escape_room, old_agent.escape_room);
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
