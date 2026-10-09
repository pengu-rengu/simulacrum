use std::iter::zip;

use crate::spec::common::{check_body_grids_unchanged, check_encounters_unchanged, check_nodes_unchanged};
use crate::spec::next::nodeworld::combat::check_engage;
use crate::spec::next::nodeworld::harvest::check_harvest;
use crate::spec::next::nodeworld::menu::{check_craft, check_exit, check_inspect};
use crate::spec::next::nodeworld::turn::{check_send_nodeworld_message, check_nodeworld_messages_unchanged, check_move_to};
use crate::state::state::State;
use crate::state::nodeworld::{NodeworldAction, NodeworldInput};


pub fn check_nodeworld_next_state(old_state: &State, new_state: &State, input: &NodeworldInput) {
    check_body_grids_unchanged(old_state, new_state);
    check_nodes_unchanged(old_state, new_state);

    if let Some(message) = &input.send_message {
        check_send_nodeworld_message(old_state, new_state, &message);
    } else {
        check_nodeworld_messages_unchanged(old_state, new_state);
    }

    if !matches!(input.action, Some(NodeworldAction::Engage { .. })) {
        check_encounters_unchanged(old_state, new_state);
    }

    match &input.action {
        Some(NodeworldAction::MoveTo(node_name)) => check_move_to(old_state, new_state, &node_name),
        Some(NodeworldAction::Harvest { poi_idx, tool_idxs }) => check_harvest(old_state, new_state, *poi_idx, tool_idxs),
        Some(NodeworldAction::Inspect { poi_idx }) => check_inspect(old_state, new_state, *poi_idx),
        Some(NodeworldAction::Craft(recipe_idx)) => check_craft(old_state, new_state, *recipe_idx),
        Some(NodeworldAction::ExitMenu) => check_exit(old_state, new_state),
        Some(NodeworldAction::Engage { poi_idx }) => check_engage(old_state, new_state, *poi_idx),
        None => {
            for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
                if i != old_state.agent_idx {
                    assert_eq!(new_agent.error_message, old_agent.error_message)
                }
                assert_eq!(new_agent.nodeworld.body_grid, old_agent.nodeworld.body_grid);
                assert_eq!(new_agent.nodeworld.open_menu, old_agent.nodeworld.open_menu);
                assert_eq!(new_agent.nodeworld.inventory, old_agent.nodeworld.inventory);
            }
        }
    }
}