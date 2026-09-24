use crate::state::state::{State, Message};
use crate::spec::common::{check_agents_idle, check_error_message};
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

pub fn check_move_to(old_state: &State, new_state: &State, node_name: &str) {
    let acting_agent_idx = old_state.agent_idx;
    let target_node_idx = old_state.nodes.iter().position(|node| node.name == node_name);

    match target_node_idx {
        Some(target_node_idx) => {
            assert_eq!(new_state.agents[acting_agent_idx].node_idx, target_node_idx);
            check_error_message(new_state, acting_agent_idx, None);
        }
        None => {
            assert_eq!(
                new_state.agents[acting_agent_idx].node_idx,
                old_state.agents[acting_agent_idx].node_idx
            );
            check_error_message(new_state, acting_agent_idx, Some(&format!("no node named {node_name}")));
        }
    }

    assert_eq!(old_state.agents[acting_agent_idx].inventory, new_state.agents[acting_agent_idx].inventory);
    assert_eq!(new_state.agents[acting_agent_idx].open_menu, None);
    check_agents_idle(old_state, new_state, true);
}

/// The message reaches whoever shared the node at the start of the turn, the sender included.
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
