use std::iter::zip;

use crate::state::state::{Menu, PointOfInterest, State};
use crate::spec::common::{check_error_and_unchanged, check_inventory, inventory_with};

/// Inspecting is how a menu is opened; only the workbench has one.
pub fn check_inspect(old_state: &State, new_state: &State, poi_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];
    let poi = old_state.nodes[acting_agent.node_idx].pois.get(poi_idx);

    match poi {
        None => check_error_and_unchanged(old_state, new_state, &format!("no poi at index {poi_idx}")),
        Some(PointOfInterest::Inspectable { name: _, menu }) => {
            assert_eq!(new_state.agents[acting_agent_idx].open_menu, Some(menu.clone()));
            assert_eq!(new_state.agents[acting_agent_idx].error_message, None);
            assert_eq!(new_state.agents[acting_agent_idx].inventory, old_state.agents[acting_agent_idx].inventory);
        }
        Some(_) => check_error_and_unchanged(old_state, new_state, "nothing to inspect here"),
    }
}

pub fn check_craft(old_state: &State, new_state: &State, recipe_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    let Some(Menu::CraftingMenu{ recipes }) = &acting_agent.open_menu else {
        check_error_and_unchanged(old_state, new_state, "crafting menu not open");
        return;
    };

    let Some(recipe) = recipes.get(recipe_idx) else {
        check_error_and_unchanged(old_state, new_state, &format!("no recipe at index {recipe_idx}"));
        return;
    };

    let mut expected_inventory = acting_agent.inventory.clone();
    for ingredient in &recipe.ingredients {
        let mut affordable = true;
        let item_stack_idx = expected_inventory.iter().position(|item_stack| item_stack.item == ingredient.item);
        if let Some(idx) = item_stack_idx {
            if expected_inventory[idx].count >= ingredient.count {
                expected_inventory[idx].count -= ingredient.count;
            } else {
                affordable = false;
            }
        } else {
            affordable = false;
        }

        if !affordable {
            check_error_and_unchanged(old_state, new_state, "not enough items to craft recipe");
            return;
        }
    }

    let mut expected_inventory = inventory_with(&expected_inventory, &recipe.output.item, recipe.output.count);
    expected_inventory.retain(|item_stack| item_stack.count > 0);

    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.inventory, expected_inventory);
            assert_eq!(new_agent.error_message, None);
        } else {
            assert_eq!(new_agent.inventory, old_agent.inventory);
            assert_eq!(new_agent.error_message, old_agent.error_message);
        }
        assert_eq!(new_agent.open_menu, old_agent.open_menu);
        assert_eq!(new_agent.node_idx, old_agent.node_idx);
    }
}

/*
pub fn check_craft(old_state: &State, new_state: &State, recipe_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    // crafting never leaves the menu
    assert_eq!(new_state.agents[acting_agent_idx].open_menu, acting_agent.open_menu);

    if acting_agent.open_menu != Some(Menu::CraftMenu { recipes: () }) {
        check_error_and_unchanged(old_state, new_state, "you are not at a workbench");
        return;
    }

    let Some(recipe) = WORKBENCH_RECIPES.get(recipe_idx) else {
        check_error_and_unchanged(old_state, new_state, &format!("no recipe at index {recipe_idx}"));
        return;
    };

    let affordable = recipe.ingredients.iter()
        .all(|ingredient| item_count(&acting_agent.inventory, &ingredient.item) >= ingredient.count);
    if !affordable {
        check_error_and_unchanged(old_state, new_state, &format!("not enough items to craft {}", item_label(&recipe.output.item)));
        return;
    }

    let mut expected_inventory = acting_agent.inventory.clone();
    for ingredient in recipe.ingredients {
        let item_stack_idx = expected_inventory.iter()
            .position(|item_stack| item_stack.item == ingredient.item)
            .unwrap();
        expected_inventory[item_stack_idx].count -= ingredient.count;
        if expected_inventory[item_stack_idx].count == 0 {
            expected_inventory.remove(item_stack_idx);
        }
    }
    let expected_inventory = inventory_with(&expected_inventory, &recipe.output.item, recipe.output.count);

    assert_eq!(new_state.agents[acting_agent_idx].error_message, None);
    check_inventory(old_state, new_state, &expected_inventory);
}
*/

pub fn check_exit(old_state: &State, new_state: &State) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    if acting_agent.open_menu.is_none() {
        check_error_and_unchanged(old_state, new_state, "nothing to exit");
        return;
    }

    assert_eq!(new_state.agents[acting_agent_idx].open_menu, None);
    assert_eq!(new_state.agents[acting_agent_idx].error_message, None);
    check_inventory(old_state, new_state, &acting_agent.inventory);
}
