use crate::spec::assumptions::{agents::check_agent, inventory::check_inventory, combat::check_combat};
use crate::spec::next::escaperoom::escaperoom::check_escaperoom_next_state;
use std::collections::HashSet;
use std::iter::zip;
use crate::state::state::{Input, State, Universe};
use crate::state::nodeworld::{Menu, NodeworldAction};
use crate::state::escaperoom::{EscapeRoomInput};
use crate::spec::common::{check_body_grids_unchanged, check_encounters_unchanged, check_error_and_nodeworld_unchanged, check_nodes_unchanged};
use serde_json::from_str;

pub fn check_error_and_unchanged(old_state: &State, new_state: &State, error_msg: &str) {
    assert_eq!(new_state.agents, old_state.agents);
    assert_eq!(new_state.nodes, old_state.nodes);
    assert_eq!(new_state.rooms, old_state.rooms);

    let acting_agent_idx = old_state.agent_idx;

    for (i, (new_agent, old_agent)) in zip(&new_state.agents, &old_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.error_message, Some(error_msg.to_string()));
        } else {
            assert_eq!(new_agent.error_message, old_agent.error_message);
        }
        assert_eq!(new_agent.nodeworld, old_agent.nodeworld);
        assert_eq!(new_agent.escape_room, old_agent.escape_room);
    }
}

pub fn assumptions(state: &State) {
    assert!(state.agents.len() > 0);
    assert!(state.agent_idx < state.agents.len());
    assert!(state.nodes.len() > 0);
    let agent_names  = state.agents.iter().map(|agent| &agent.name).collect::<HashSet<&String>>();
    assert_eq!(agent_names.len(), state.agents.len());

    for agent in &state.agents {
        check_agent(state, agent);
        check_inventory(agent);
    }
    let node_names = state.nodes.iter().map(|node| &node.name).collect::<HashSet<&String>>();
    assert_eq!(node_names.len(), state.nodes.len());

    check_combat(state);
}

pub fn check_next_state(old_state: &State, new_state: &State, input_str: &str) {
    let new_idx = old_state.agent_idx + 1;
    if new_idx == old_state.agents.len() {
        assert_eq!(new_state.agent_idx, 0);
        assert_eq!(new_state.turn, old_state.turn + 1);
    } else {
        assert_eq!(new_state.agent_idx, new_idx);
        assert_eq!(new_state.turn, old_state.turn)
    }

    let Ok(input) = from_str::<Input>(input_str) else {
        check_error_and_unchanged(old_state, new_state, "could not parse input json");
        return;
    };

    match &old_state.agents[old_state.agent_idx].universe {
        Universe::Nodeworld => {
            panic!("not implemented");
            /*
            if let Input::NodeworldInput(nodeworld_input) = &input {
                check_nodeworld_next_state(old_state, new_state, &nodeworld_input);
            } else {
                check_error_and_unchanged(old_state, new_state, "expected NodeworldInput");
            }
            */
        }
        Universe::EscapeRoom => {
            if let Input::EscapeRoomInput(escaperoom_input) = &input {
                check_escaperoom_next_state(old_state, new_state, &escaperoom_input);
            } else {
                check_error_and_unchanged(old_state, new_state, "expected EscapeRoomInput");
            }
        }
    }
}

pub fn check_flush_state(old_state: &State, new_state: &State, output: &str) {
    
}
