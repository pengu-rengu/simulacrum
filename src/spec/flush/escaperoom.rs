use crate::state::state::State;
use crate::state::escaperoom::EscapeRoomCell;

pub fn check_flush_escaperoom_state(old_state: &State, new_state: &State, output: &str) {
    let mut expected_output = String::new();

    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx].escape_room;
    let room = &old_state.rooms[acting_agent.room_idx];

    expected_output += &format!("Room {}:\n", room.name);

    for (i, cell) in room.cells.iter().enumerate() {
        let x = i % room.width;
        let y = i / room.width;
        let char = if x == acting_agent.x && y == acting_agent.y {
            &(acting_agent_idx + 1).to_string()
        } else {
            match cell {
                EscapeRoomCell::Empty => " ",
                EscapeRoomCell::Wall => "x",
                EscapeRoomCell::Door(door) => {
                    let id = &door.id;
                    if door.open { &id.to_lowercase() } else { &id.to_uppercase() }
                }
            }
        };
        expected_output += char;
        if x == room.width - 1 {
            expected_output += "\n";
        }
    }

    if let Some(error_message) = &old_state.agents[acting_agent_idx].error_message {
        expected_output += &format!("Error: {}\n", error_message);
    }
    assert!(new_state.agents[acting_agent_idx].error_message.is_none());

    let messages = &acting_agent.messages_inbox;
    if !messages.is_empty() {
        expected_output += &format!("Messages:\n");
        for message in messages {
            expected_output += &format!("[{}] {}\n", old_state.agents[message.sender_agent_idx].name, message.content);
        }
    }
    assert!(new_state.agents[acting_agent_idx].escape_room.messages_inbox.is_empty());
    assert_eq!(output, expected_output);
}