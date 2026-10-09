use crate::state::state::State;
use crate::state::escaperoom::EscapeRoomCell;

pub fn check_flush_escaperoom_state(old_state: &State, new_state: &State, output: &str) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx].escape_room;

    if acting_agent.finished {
        assert_eq!(output, "you won!");
        return;
    }

    let mut expected_output = String::new();

    
    let room = &old_state.rooms[acting_agent.room_idx];

    expected_output += &format!("Room: {}\n\n", room.name);

    for (i, cell) in room.cells.iter().enumerate() {
        let x = i % room.width;
        let y = i / room.width;

        let mut agent_on_cell = false;
        for (agent_idx, agent) in old_state.agents.iter().enumerate() {
            if agent.escape_room.room_idx != acting_agent.room_idx { continue; }
            if x == agent.escape_room.x && y == agent.escape_room.y {
                expected_output += &(agent_idx + 1).to_string();
                agent_on_cell = true;
                break;
            }
        }
        
        if !agent_on_cell {
            match cell {
                EscapeRoomCell::Empty | EscapeRoomCell::Spawn(_) => expected_output += " ",
                EscapeRoomCell::Wall => expected_output += "x",
                EscapeRoomCell::Door(door) => {
                    if door.open {
                        expected_output += &door.id.to_lowercase();
                    } else {
                        expected_output += &door.id.to_uppercase();
                    }
                },
                EscapeRoomCell::Exit => expected_output += "E"
            }
        }
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