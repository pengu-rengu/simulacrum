use crate::state::state::{State, Input, Message};
use serde_json::from_str;

pub fn flush_state(state: &State) -> (State, String) {
    let mut new_state = state.clone();
    let agent = &mut new_state.agents[new_state.agent_idx];

    let mut lines = vec!["Messages:\n".to_string()];
    for message in &agent.node_messages_inbox {
        lines.push(format!("[{}] {}", state.agents[message.sender_agent_idx].name, message.content));
    }
    if let Some(error_message) = &agent.error_message {
        lines.push(format!("Error: {}", error_message));
    }

    let output = lines.join("\n");
    agent.node_messages_inbox.clear();
    (new_state, output)
}

pub fn next_state(state: &State, input_str: &str) -> State {
    let mut new_state = state.clone();
    let curr_agent_idx = state.agent_idx;

    match from_str::<Input>(input_str) {
        Err(_) => {
            new_state.agents[curr_agent_idx].error_message = Some("could not parse input json".to_string());
        }
        Ok(input) => {
            new_state.agents[curr_agent_idx].error_message = None;
            if let Some(content) = input.send_message {
                let curr_node_idx = state.agents[curr_agent_idx].node_idx;
                for agent in &mut new_state.agents {
                    if agent.node_idx != curr_node_idx { continue; }
                    agent.node_messages_inbox.push(Message {
                        sender_agent_idx: curr_agent_idx,
                        content: content.clone(),
                    });
                }
            }
        }
    }

    new_state.agent_idx += 1;
    if new_state.agent_idx == new_state.agents.len() {
        new_state.agent_idx = 0;
        new_state.turn += 1;
    }
    new_state
}
