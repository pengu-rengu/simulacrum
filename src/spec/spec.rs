use crate::state::state::{State, Input, Action, Menu};
use crate::spec::common::{
    check_agents_idle, check_body_grids_unchanged, check_error_message, check_nodes_unchanged,
};
use crate::spec::next::{
    harvest::check_harvest,
    menu::{check_craft, check_exit, check_inspect, check_menu_locked},
    turn::{check_increment, check_move_to, check_send_message},
};
use crate::spec::flush::{
    sections::{error_lines, inventory_lines, message_lines},
    views::{node_lines, workbench_lines},
};
use serde_json::from_str;

pub fn check_next_state(old_state: &State, new_state: &State, input_str: &str) {
    check_increment(old_state, new_state);
    check_body_grids_unchanged(old_state, new_state);
    check_nodes_unchanged(old_state, new_state);

    let Ok(input) = from_str::<Input>(input_str) else {
        check_error_message(new_state, old_state.agent_idx, Some("could not parse input json"));
        return;
    };

    let menu_open = old_state.agents[old_state.agent_idx].open_menu.is_some();
    match input.action {
        Some(Action::MoveTo(_)) | Some(Action::Harvest { .. }) | Some(Action::Inspect { .. })
            if menu_open =>
        {
            check_menu_locked(old_state, new_state);
        }
        Some(Action::MoveTo(node_name)) => check_move_to(old_state, new_state, &node_name),
        Some(Action::Harvest { poi_idx, tool_idx, uses }) => {
            check_harvest(old_state, new_state, poi_idx, tool_idx, uses)
        }
        Some(Action::Inspect { poi_idx }) => check_inspect(old_state, new_state, poi_idx),
        Some(Action::Craft(recipe_idx)) => check_craft(old_state, new_state, recipe_idx),
        Some(Action::Exit) => check_exit(old_state, new_state),
        None => {
            check_error_message(new_state, old_state.agent_idx, None);
            check_agents_idle(old_state, new_state, false);
        }
    }

    if let Some(message) = input.send_message {
        check_send_message(old_state, new_state, &message);
    }
}

pub fn check_flush_state(old_state: &State, new_state: &State, output: &str) {
    assert_eq!(old_state.agent_idx, new_state.agent_idx);
    check_body_grids_unchanged(old_state, new_state);
    check_nodes_unchanged(old_state, new_state);

    let agent_idx = new_state.agent_idx;
    let agent = &old_state.agents[agent_idx];

    // an open menu replaces the node view; the agent still holds items and still hears the room
    let mut lines = match agent.open_menu {
        Some(Menu::Workbench) => workbench_lines(),
        None => node_lines(old_state, agent),
    };
    lines.extend(inventory_lines(agent));
    lines.extend(message_lines(old_state, agent));
    lines.extend(error_lines(&new_state.agents[agent_idx].error_message));

    assert_eq!(output, lines.join("\n"));
    assert!(new_state.agents[agent_idx].node_messages_inbox.is_empty());
}
