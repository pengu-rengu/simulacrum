use crate::state::state::{State, Input};
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

pub fn check_send_message(old_state: &State, new_state: &State, message: &str) {
    let curr_node_idx = old_state.agents[old_state.agent_idx].node_idx;
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        if old_agent.node_idx != curr_node_idx { continue; }

        if let Some((last, rest)) = new_agent.node_messages_inbox.split_last() {
            assert_eq!(last, message);
            assert_eq!(rest, old_agent.node_messages_inbox);
        } else {
            panic!("new messages cannot be empty")
        }
    }
}

pub fn check_next_state(old_state: &State, new_state: &State, input_str: &str) {
    let Ok(input) = from_str::<Input>(input_str) else {
        //assert_eq!(new_state.agents[new_state.agent_idx].output, "Error: could not parse input json");
        return;
    };
    check_increment(old_state, new_state);
    if let Some(message) = input.send_message {
        check_send_message(old_state, new_state, &message);
    }
}

pub fn check_flush_state(old_state: &State, new_state: &State, output: &str) {
    assert_eq!(old_state.agent_idx, new_state.agent_idx);
    let agent_idx = new_state.agent_idx;
    
    let expected_output = old_state.agents[agent_idx].node_messages_inbox.join("\n");
    assert_eq!(output, expected_output);
    assert!(new_state.agents[agent_idx].node_messages_inbox.is_empty());
}