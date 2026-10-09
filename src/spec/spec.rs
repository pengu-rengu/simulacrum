use crate::spec::assumptions::{agents::check_agent, inventory::check_inventory, combat::check_combat};
use crate::spec::next::escaperoom::escaperoom::check_escaperoom_next_state;
use crate::spec::next::nodeworld::nodeworld::check_nodeworld_next_state;
use std::collections::HashSet;
use crate::state::state::{Input, Menu, NodeworldAction, State, Universe};
use crate::spec::common::{check_body_grids_unchanged, check_encounters_unchanged, check_error_and_unchanged, check_nodes_unchanged};
use crate::spec::flush::{
    sections::{error_lines, inventory_lines, message_lines},
    views::{node_lines, crafting_menu_lines},
};
use serde_json::from_str;

pub fn assumptions(state: &State) {
    assert!(state.agents.len() > 0);
    assert!(state.agent_idx < state.agents.len());
    assert!(state.nodes.len() > 0);

    for agent in &state.agents {
        check_agent(state, agent);
        check_inventory(agent);
    }

    let agent_names  = state.agents.iter().map(|agent| &agent.name).collect::<HashSet<&String>>();
    assert_eq!(agent_names.len(), state.agents.len());

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
    
    check_body_grids_unchanged(old_state, new_state);
    check_nodes_unchanged(old_state, new_state);

    let Ok(input) = from_str::<Input>(input_str) else {
        check_error_and_unchanged(old_state, new_state, "could not parse input json");
        check_encounters_unchanged(old_state, new_state);
        return;
    };

    match &old_state.agents[old_state.agent_idx].universe {
        Universe::Nodeworld => {
            if let Input::NodeworldInput(nodeworld_input) = &input {
                check_nodeworld_next_state(old_state, new_state, &nodeworld_input);
            } else {
                check_error_and_unchanged(old_state, new_state, "expected NodeworldInput");
            }
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
    assert_eq!(old_state.agent_idx, new_state.agent_idx);
    check_body_grids_unchanged(old_state, new_state);
    check_nodes_unchanged(old_state, new_state);
    check_encounters_unchanged(old_state, new_state);

    let agent_idx = new_state.agent_idx;
    let agent = &old_state.agents[agent_idx];

    // an open menu replaces the node view; the agent still holds items and still hears the room
    let mut lines = match &agent.nodeworld.open_menu {
        Some(Menu::CraftingMenu { recipes }) => crafting_menu_lines(recipes),
        None => node_lines(old_state, agent),
    };
    lines.extend(inventory_lines(agent));
    lines.extend(message_lines(old_state, agent));
    lines.extend(error_lines(&new_state.agents[agent_idx].error_message));

    assert_eq!(output, lines.join("\n"));
    assert!(new_state.agents[agent_idx].nodeworld.node_messages_inbox.is_empty());
}
