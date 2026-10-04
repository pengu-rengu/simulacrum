use crate::state::state::{State, Message};
use crate::spec::common::{action_blocked, check_error_and_unchanged};
use std::iter::zip;

pub fn check_move_to(old_state: &State, new_state: &State, node_name: &str) {
    if let Some(error) = action_blocked(old_state) {
        check_error_and_unchanged(old_state, new_state, error);
        return;
    }

    let acting_agent_idx = old_state.agent_idx;
    let Some(target_node_idx) = old_state.nodes.iter().position(|node| node.name == node_name) else {
        check_error_and_unchanged(old_state, new_state, &format!("no node named {node_name}"));
        return;
    };

    for (i,(old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.nodeworld.node_idx, target_node_idx);
            assert_eq!(new_agent.nodeworld.error_message, None);
        } else {
            assert_eq!(new_agent.nodeworld.node_idx, old_agent.nodeworld.node_idx);
            assert_eq!(new_agent.nodeworld.error_message, old_agent.nodeworld.error_message);
        }

        assert_eq!(new_agent.nodeworld.open_menu, old_agent.nodeworld.open_menu);
        assert_eq!(new_agent.nodeworld.inventory, old_agent.nodeworld.inventory);
    }
}

pub fn check_send_message(old_state: &State, new_state: &State, content: &str) {
    let curr_node_idx = old_state.agents[old_state.agent_idx].nodeworld.node_idx;
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        if old_agent.nodeworld.node_idx != curr_node_idx {
            assert_eq!(new_agent.nodeworld.node_messages_inbox, old_agent.nodeworld.node_messages_inbox);
            continue;
        }

        if let Some((last, rest)) = new_agent.nodeworld.node_messages_inbox.split_last() {
            let new_message = Message {
                sender_agent_idx: old_state.agent_idx,
                content: content.to_string()
            };
            assert_eq!(*last, new_message);
            assert_eq!(rest, old_agent.nodeworld.node_messages_inbox);
        } else {
            panic!("new messages cannot be empty")
        }
    }
}

pub fn check_messages_unchanged(old_state: &State, new_state: &State) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        assert_eq!(new_agent.nodeworld.node_messages_inbox, old_agent.nodeworld.node_messages_inbox);
    }
}