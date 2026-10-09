use crate::spec::common::{check_body_grids_unchanged, check_encounters_unchanged, check_nodes_unchanged};
use crate::state::state::State;
use crate::state::nodeworld::Menu;
use crate::spec::flush::nodeworld::sections::{inventory_lines, message_lines, error_lines};
use crate::spec::flush::nodeworld::views::{crafting_menu_lines, node_lines};

pub fn check_nodeworld_flush_state(old_state: &State, new_state: &State, output: &str) {
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