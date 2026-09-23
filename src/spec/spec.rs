use crate::state::state::{State, Input, Message};
use serde_json::from_str;
use std::iter::zip;

pub fn check_increment(old_state: &State, new_state: &State) {
    let new_idx = old_state.agent_idx + 1;
    if new_idx == old_state.agents.len() {
        assert_eq!(new_state.agent_idx, 0);
        assert_eq!(new_state.turn, old_state.turn + 1);
    } else {
        assert_eq!(new_state.agent_idx, new_idx);
        assert_eq!(new_state.turn, old_state.turn)
    }
}

pub fn check_body_grids_unchanged(old_state: &State, new_state: &State) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        assert_eq!(old_agent.body_grid, new_agent.body_grid);
    }
}

pub fn check_send_message(old_state: &State, new_state: &State, content: &str) {
    let curr_node_idx = old_state.agents[old_state.agent_idx].node_idx;
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        if old_agent.node_idx != curr_node_idx { continue; }

        if let Some((last, rest)) = new_agent.node_messages_inbox.split_last() {
            let new_message = Message {
                sender_agent_idx: old_state.agent_idx,
                content: content.to_string()
            };
            assert_eq!(*last, new_message);
            assert_eq!(rest, old_agent.node_messages_inbox);
        } else {
            panic!("new messages cannot be empty")
        }
    }
}

pub fn check_next_state(old_state: &State, new_state: &State, input_str: &str) {
    check_increment(old_state, new_state);
    check_body_grids_unchanged(old_state, new_state);
    let Ok(input) = from_str::<Input>(input_str) else {
        assert_eq!(new_state.agents[old_state.agent_idx].error_message, Some("could not parse input json".to_string()));
        return;
    };
    assert_eq!(new_state.agents[old_state.agent_idx].error_message, None);
    if let Some(message) = input.send_message {
        check_send_message(old_state, new_state, &message);
    }
}

pub fn check_flush_state(old_state: &State, new_state: &State, output: &str) {
    assert_eq!(old_state.agent_idx, new_state.agent_idx);
    check_body_grids_unchanged(old_state, new_state);
    let agent_idx = new_state.agent_idx;
    
    let mut lines = vec!["Messages:\n".to_string()];

    for message in &old_state.agents[agent_idx].node_messages_inbox {
        lines.push(format!("[{}] {}", old_state.agents[message.sender_agent_idx].name, message.content));
    }
    
    if let Some(error_message) = &new_state.agents[agent_idx].error_message {
        lines.push(format!("Error: {}", error_message));
    }

    let expected_output = lines.join("\n");
    assert_eq!(output, expected_output);
    assert!(new_state.agents[agent_idx].node_messages_inbox.is_empty());
}