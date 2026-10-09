use std::iter::zip;

use crate::state::nodeworld::{Item, Menu, PointOfInterest};
use crate::state::state::State;
use crate::spec::common::{action_blocked, check_error_and_nodeworld_unchanged, inventory_with};

/// Inspecting is how a menu is opened; only the workbench has one.
pub fn check_inspect(old_state: &State, new_state: &State, poi_idx: usize) {
    if let Some(error) = action_blocked(old_state) {
        check_error_and_nodeworld_unchanged(old_state, new_state, error);
        return;
    }

    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];
    let poi = old_state.nodes[acting_agent.nodeworld.node_idx].pois.get(poi_idx);

    match poi {
        None => check_error_and_nodeworld_unchanged(old_state, new_state, &format!("no poi at index {poi_idx}")),
        Some(PointOfInterest::Inspectable { name: _, menu }) => {
            assert_eq!(new_state.agents[acting_agent_idx].nodeworld.open_menu, Some(menu.clone()));
            assert_eq!(new_state.agents[acting_agent_idx].error_message, None);
            assert_eq!(new_state.agents[acting_agent_idx].nodeworld.inventory, old_state.agents[acting_agent_idx].nodeworld.inventory);
        }
        Some(_) => check_error_and_nodeworld_unchanged(old_state, new_state, "nothing to inspect here"),
    }
}

pub fn check_craft(old_state: &State, new_state: &State, recipe_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    let Some(Menu::CraftingMenu{ recipes }) = &acting_agent.nodeworld.open_menu else {
        check_error_and_nodeworld_unchanged(old_state, new_state, "crafting menu not open");
        return;
    };

    let Some(recipe) = recipes.get(recipe_idx) else {
        check_error_and_nodeworld_unchanged(old_state, new_state, &format!("no recipe at index {recipe_idx}"));
        return;
    };

    let mut expected_inventory = acting_agent.nodeworld.inventory.clone();
    // an ingredient is a material, matched by name
    for (ingredient_name, ingredient_count) in &recipe.ingredients {
        let mut affordable = true;
        let item_stack_idx = expected_inventory.iter().position(|item_stack| {
            matches!(&item_stack.item, Item::Material { name } if name == ingredient_name)
        });
        if let Some(idx) = item_stack_idx {
            if expected_inventory[idx].count >= *ingredient_count {
                expected_inventory[idx].count -= *ingredient_count;
            } else {
                affordable = false;
            }
        } else {
            affordable = false;
        }

        if !affordable {
            check_error_and_nodeworld_unchanged(old_state, new_state, "not enough items to craft recipe");
            return;
        }
    }

    let mut expected_inventory = inventory_with(&expected_inventory, &recipe.output.item, recipe.output.count);
    expected_inventory.retain(|item_stack| item_stack.count > 0);

    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.nodeworld.inventory, expected_inventory);
            assert_eq!(new_agent.error_message, None);
        } else {
            assert_eq!(new_agent.nodeworld.inventory, old_agent.nodeworld.inventory);
            assert_eq!(new_agent.error_message, old_agent.error_message);
        }
        assert_eq!(new_agent.nodeworld.open_menu, old_agent.nodeworld.open_menu);
        assert_eq!(new_agent.nodeworld.node_idx, old_agent.nodeworld.node_idx);
    }
}

pub fn check_exit(old_state: &State, new_state: &State) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    if acting_agent.nodeworld.open_menu.is_none() {
        check_error_and_nodeworld_unchanged(old_state, new_state, "nothing to exit");
        return;
    }

    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.nodeworld.open_menu, None);
            assert_eq!(new_agent.error_message, None);
        } else {
            assert_eq!(new_agent.nodeworld.open_menu, old_agent.nodeworld.open_menu);
            assert_eq!(new_agent.error_message, old_agent.error_message);
        }
        assert_eq!(new_agent.nodeworld.inventory, old_agent.nodeworld.inventory);
        assert_eq!(new_agent.nodeworld.node_idx, old_agent.nodeworld.node_idx);
    }
}
