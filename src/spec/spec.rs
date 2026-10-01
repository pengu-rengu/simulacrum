use crate::state::state::{State, Input, Action, Menu};
use crate::spec::common::{check_body_grids_unchanged, check_encounters_unchanged, check_error_and_unchanged, check_nodes_unchanged};
use crate::spec::next::{
    combat::check_engage,
    harvest::check_harvest,
    menu::{check_craft, check_exit, check_inspect},
    turn::{check_increment, check_move_to, check_send_message, check_messages_unchanged},
};
use crate::spec::flush::{
    sections::{error_lines, inventory_lines, message_lines},
    views::{node_lines, crafting_menu_lines},
};
use serde_json::from_str;
use std::iter::zip;

pub fn check_next_state(old_state: &State, new_state: &State, input_str: &str) {
    check_increment(old_state, new_state);
    check_body_grids_unchanged(old_state, new_state);
    check_nodes_unchanged(old_state, new_state);

    let Ok(input) = from_str::<Input>(input_str) else {
        check_error_and_unchanged(old_state, new_state, "could not parse input json");
        check_encounters_unchanged(old_state, new_state);
        return;
    };

    // only engaging starts or joins a fight
    if !matches!(input.action, Some(Action::Engage { .. })) {
        check_encounters_unchanged(old_state, new_state);
    }

    if let Some(message) = input.send_message {
        check_send_message(old_state, new_state, &message);
    } else {
        check_messages_unchanged(old_state, new_state);
    }

    match input.action {
        Some(Action::MoveTo(_)) | Some(Action::Harvest { .. }) | Some(Action::Inspect { .. }) | Some(Action::Engage { .. })
            if old_state.agents[old_state.agent_idx].open_menu.is_some() =>
        {
            check_error_and_unchanged(old_state, new_state, "you cannot do that while a menu is open");
            check_encounters_unchanged(old_state, new_state);
        }
        Some(Action::MoveTo(node_name)) => check_move_to(old_state, new_state, &node_name),
        Some(Action::Harvest { poi_idx, tool_idxs }) => check_harvest(old_state, new_state, poi_idx, tool_idxs),
        Some(Action::Inspect { poi_idx }) => check_inspect(old_state, new_state, poi_idx),
        Some(Action::Craft(recipe_idx)) => check_craft(old_state, new_state, recipe_idx),
        Some(Action::Exit) => check_exit(old_state, new_state),
        Some(Action::Engage { poi_idx }) => check_engage(old_state, new_state, poi_idx),
        None => {
            for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
                if i != old_state.agent_idx {
                    assert_eq!(new_agent.error_message, old_agent.error_message)
                }
                assert_eq!(new_agent.body_grid, old_agent.body_grid);
                assert_eq!(new_agent.open_menu, old_agent.open_menu);
                assert_eq!(new_agent.inventory, old_agent.inventory);
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
    let mut lines = match &agent.open_menu {
        Some(Menu::CraftingMenu { recipes }) => crafting_menu_lines(recipes),
        None => node_lines(old_state, agent),
    };
    lines.extend(inventory_lines(agent));
    lines.extend(message_lines(old_state, agent));
    lines.extend(error_lines(&new_state.agents[agent_idx].error_message));

    assert_eq!(output, lines.join("\n"));
    assert!(new_state.agents[agent_idx].node_messages_inbox.is_empty());
}
