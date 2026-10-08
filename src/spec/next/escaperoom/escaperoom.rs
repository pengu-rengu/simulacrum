use std::iter::zip;

use crate::spec::common::check_encounters_unchanged;
use crate::spec::next::turn::{check_escaperoom_message_unchanged, check_nodeworld_messages_unchanged, check_send_escaperoom_message};
use crate::state::escaperoom::{EscapeRoomAction, EscapeRoomCell, EscapeRoomInput};
use crate::state::state::{Direction, State};

fn step(x: usize, y: usize, direction: &Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => y.checked_sub(1).map(|y| (x, y)),
        Direction::Down => y.checked_add(1).map(|y| (x, y)),
        Direction::Left => x.checked_sub(1).map(|x| (x, y)),
        Direction::Right => x.checked_add(1).map(|x| (x, y))
    }
}

fn passable(cell: &EscapeRoomCell) -> bool {
    match cell {
        EscapeRoomCell::Empty => true,
        EscapeRoomCell::Door(door) => door.open,
        EscapeRoomCell::Wall => false
    }
}

pub fn check_escaperoom_next_state(old_state: &State, new_state: &State, input: &EscapeRoomInput) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];
    let mut x = acting_agent.escape_room.x;
    let mut y = acting_agent.escape_room.y;
    let room_idx = acting_agent.escape_room.room_idx;
    let mut rooms = old_state.rooms.clone();

    for action in &input.actions {
        match action {
            EscapeRoomAction::Move { direction } => {
                let Some((next_x, next_y)) = step(x, y, direction) else { continue };
                let room = &rooms[room_idx];
                if next_x >= room.width || next_y >= room.height { continue; }
                let Some(cell) = room.cells.get(next_y * room.width + next_x) else { continue };
                if passable(cell) {
                    x = next_x;
                    y = next_y;
                }
            }
            EscapeRoomAction::Interact { direction } => {
                let Some((next_x, next_y)) = step(x, y, direction) else { continue };
                let id = {
                    let room = &rooms[room_idx];
                    if next_x >= room.width || next_y >= room.height { continue; }
                    let Some(EscapeRoomCell::Door(door)) = room.cells.get(next_y * room.width + next_x) else { continue };
                    door.id.clone()
                };
                for cell in &mut rooms[room_idx].cells {
                    if let EscapeRoomCell::Door(door) = cell {
                        if door.id == id {
                            door.open = !door.open;
                        }
                    }
                }
            }
        }
    }

    assert_eq!(new_state.rooms, rooms);
    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.escape_room.x, x);
            assert_eq!(new_agent.escape_room.y, y);
            assert_eq!(new_agent.escape_room.room_idx, old_agent.escape_room.room_idx);
            assert_eq!(new_agent.nodeworld.error_message, None);
        } else {
            assert_eq!(new_agent.escape_room.room_idx, old_agent.escape_room.room_idx);
            assert_eq!(new_agent.escape_room.x, old_agent.escape_room.x);
            assert_eq!(new_agent.escape_room.y, old_agent.escape_room.y);
            assert_eq!(new_agent.nodeworld.error_message, old_agent.nodeworld.error_message);
        }
        assert_eq!(new_agent.nodeworld.inventory, old_agent.nodeworld.inventory);
        assert_eq!(new_agent.nodeworld.open_menu, old_agent.nodeworld.open_menu);
        assert_eq!(new_agent.nodeworld.node_idx, old_agent.nodeworld.node_idx);
    }
    check_encounters_unchanged(old_state, new_state);

    check_nodeworld_messages_unchanged(old_state, new_state);
    if let Some(message) = &input.send_message {
        check_send_escaperoom_message(old_state, new_state, message);
    } else {
        check_escaperoom_message_unchanged(old_state, new_state);
    }
}
