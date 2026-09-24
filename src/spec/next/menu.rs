use crate::state::state::{State, PointOfInterest, Menu};
use crate::spec::common::{
    check_agent_unchanged, check_error_message, check_inventory, inventory_with,
    item_count, item_label, workbench_recipes,
};

/// Inspecting is how a menu is opened; only the workbench has one.
pub fn check_inspect(old_state: &State, new_state: &State, poi_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];
    let poi = old_state.nodes[acting_agent.node_idx].pois.get(poi_idx);

    let expected_error = match poi {
        None => Some(format!("no poi at index {poi_idx}")),
        Some(PointOfInterest::RuinedWorkbench) => None,
        Some(_) => Some("nothing to inspect here".to_string()),
    };
    let expected_open_menu = match poi {
        Some(PointOfInterest::RuinedWorkbench) => Some(Menu::Workbench),
        _ => acting_agent.open_menu.clone(),
    };

    assert_eq!(new_state.agents[acting_agent_idx].open_menu, expected_open_menu);
    check_error_message(new_state, acting_agent_idx, expected_error.as_deref());
    check_inventory(old_state, new_state, &acting_agent.inventory);
}

pub fn check_craft(old_state: &State, new_state: &State, recipe_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    // crafting never leaves the menu
    assert_eq!(new_state.agents[acting_agent_idx].open_menu, acting_agent.open_menu);

    if acting_agent.open_menu != Some(Menu::Workbench) {
        check_error_message(new_state, acting_agent_idx, Some("you are not at a workbench"));
        check_agent_unchanged(old_state, new_state);
        return;
    }

    let recipes = workbench_recipes();
    let Some(recipe) = recipes.get(recipe_idx) else {
        check_error_message(new_state, acting_agent_idx, Some(&format!("no recipe at index {recipe_idx}")));
        check_agent_unchanged(old_state, new_state);
        return;
    };

    let affordable = recipe.ingredients.iter()
        .all(|ingredient| item_count(&acting_agent.inventory, &ingredient.item) >= ingredient.count);
    if !affordable {
        check_error_message(
            new_state,
            acting_agent_idx,
            Some(&format!("not enough items to craft {}", item_label(&recipe.output.item))),
        );
        check_agent_unchanged(old_state, new_state);
        return;
    }

    let mut expected_inventory = acting_agent.inventory.clone();
    for ingredient in &recipe.ingredients {
        let item_stack_idx = expected_inventory.iter()
            .position(|item_stack| item_stack.item == ingredient.item)
            .unwrap();
        expected_inventory[item_stack_idx].count -= ingredient.count;
        if expected_inventory[item_stack_idx].count == 0 {
            expected_inventory.remove(item_stack_idx);
        }
    }
    let expected_inventory = inventory_with(&expected_inventory, &recipe.output.item, recipe.output.count);

    check_error_message(new_state, acting_agent_idx, None);
    check_inventory(old_state, new_state, &expected_inventory);
}

pub fn check_exit(old_state: &State, new_state: &State) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    assert_eq!(new_state.agents[acting_agent_idx].open_menu, None);
    match acting_agent.open_menu {
        Some(_) => check_error_message(new_state, acting_agent_idx, None),
        None => check_error_message(new_state, acting_agent_idx, Some("nothing to exit")),
    }

    check_inventory(old_state, new_state, &acting_agent.inventory);
}

/// While a menu is open the agent can only craft or exit.
pub fn check_menu_locked(old_state: &State, new_state: &State) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    assert_eq!(new_state.agents[acting_agent_idx].open_menu, acting_agent.open_menu);
    check_error_message(
        new_state,
        acting_agent_idx,
        Some("you are at the ruined workbench: craft or exit"),
    );
    check_inventory(old_state, new_state, &acting_agent.inventory);
}
