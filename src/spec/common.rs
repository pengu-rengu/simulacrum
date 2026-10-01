use crate::state::state::{State, Item, ItemStack};
use std::iter::zip;

/// Durability of a tool, or None for items that are not tools.
pub fn tool_durability(item: &Item) -> Option<usize> {
    match item {
        Item::CrudePickaxe { durability }
        | Item::CopperPickaxe { durability }
        | Item::CopperDrill { durability } => Some(*durability),
        _ => None
    }
}

pub fn item_label(item: &Item) -> String {
    match item {
        Item::CrudePickaxe { durability } => format!("CrudePickaxe (durability {durability})"),
        Item::CopperPickaxe { durability } => format!("CopperPickaxe (durability {durability})"),
        Item::CopperDrill { durability } => format!("CopperDrill (durability {durability})"),
        _ => format!("{:?}", item),
    }
}

pub fn inventory_with(inventory: &Vec<ItemStack>, item: &Item, count: usize) -> Vec<ItemStack> {
    let mut new_inventory = inventory.clone();
    if count == 0 { return new_inventory; }

    // tools stay unstacked
    if tool_durability(item).is_some() {
        new_inventory.push(ItemStack { item: item.clone(), count });
    } else if let Some(item_stack) = new_inventory.iter_mut().find(|item_stack| item_stack.item == *item) {
        item_stack.count += count;
    } else {
        new_inventory.push(ItemStack { item: item.clone(), count });
    }
    new_inventory
}

pub fn check_error_and_unchanged(old_state: &State, new_state: &State, expected: &str) {
    let acting_agent_idx = old_state.agent_idx;
    let old_agent = &old_state.agents[acting_agent_idx];
    let new_agent = &new_state.agents[acting_agent_idx];
    assert_eq!(new_agent.error_message, Some(expected.to_string()));
    assert_eq!(new_agent.open_menu, old_agent.open_menu);
    assert_eq!(new_agent.node_idx, old_agent.node_idx);
    assert_eq!(new_agent.inventory, old_agent.inventory);
}

pub fn check_body_grids_unchanged(old_state: &State, new_state: &State) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        assert_eq!(old_agent.body_grid, new_agent.body_grid);
    }
}

pub fn check_nodes_unchanged(old_state: &State, new_state: &State) {
    assert_eq!(old_state.nodes.len(), new_state.nodes.len());
    for (old_node, new_node) in zip(&old_state.nodes, &new_state.nodes) {
        assert_eq!(old_node.name, new_node.name);
        assert_eq!(old_node.biome, new_node.biome);
        assert_eq!(old_node.pois, new_node.pois);
    }
}

pub fn check_encounters_unchanged(old_state: &State, new_state: &State) {
    for (old_node, new_node) in zip(&old_state.nodes, &new_state.nodes) {
        assert_eq!(old_node.combat_encounters, new_node.combat_encounters);
    }
}
