use crate::state::state::{State, Input};
use serde_json::from_str;

fn increment(state: &mut State) {
    state.agent_idx += 1;
    if state.agent_idx == state.agents.len() {
        state.agent_idx = 0;
        state.turn += 1;
    }
}

fn send_message(state: &mut State, message: &str) {
    let curr_node_idx = state.agents[state.agent_idx].node_idx;
    for agent in &mut state.agents {
        if agent.node_idx != curr_node_idx { continue; }
        agent.node_messages_inbox.push(message.to_string());
    }
}

pub fn flush_state(state: &State) -> (State, String) {
    let mut new_state = state.clone();
    let agent = &mut new_state.agents[new_state.agent_idx];
    let output = agent.node_messages_inbox.join("\n");
    agent.node_messages_inbox.clear();
    (new_state, output)
}

pub fn next_state(state: &State, input_str: &str) -> State {
    let mut new_state = state.clone();
    let Ok(input) = from_str::<Input>(input_str) else {
        return new_state;
    };
    if let Some(message) = input.send_message {
        send_message(&mut new_state, &message);
    }
    increment(&mut new_state);
    new_state
}
