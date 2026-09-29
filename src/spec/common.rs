use crate::state::state::{State, Item, ItemStack, PointOfInterest};
use std::iter::zip;

pub fn item_label(item: &Item) -> String {
    match item {
        Item::CrudePickaxe { durability } => format!("CrudePickaxe (durability {durability})"),
        _ => format!("{:?}", item),
    }
}

pub fn item_count(agent_inventory: &Vec<ItemStack>, item: &Item) -> usize {
    agent_inventory.iter()
        .find(|item_stack| item_stack.item == *item)
        .map(|item_stack| item_stack.count)
        .unwrap_or(0)
}

/// `count` of `item` added to `inventory`, keeping tools unstacked and one stack per other item.
pub fn inventory_with(inventory: &Vec<ItemStack>, item: &Item, count: usize) -> Vec<ItemStack> {
    let mut new_inventory = inventory.clone();
    if count == 0 { return new_inventory; }

    match item {
        Item::CrudePickaxe { .. } => new_inventory.push(ItemStack { item: item.clone(), count }),
        _ => {
            if let Some(item_stack) = new_inventory.iter_mut().find(|item_stack| item_stack.item == *item) {
                item_stack.count += count;
            } else {
                new_inventory.push(ItemStack { item: item.clone(), count });
            }
        }
    }
    new_inventory
}

pub fn check_inventory(old_state: &State, new_state: &State, expected_inventory: &Vec<ItemStack>) {
    assert_eq!(new_state.agents[old_state.agent_idx].inventory, *expected_inventory);
}

pub fn check_error_and_unchanged(old_state: &State, new_state: &State, expected: &str) {
    let acting_agent_idx = old_state.agent_idx;
    let old_agent = &old_state.agents[acting_agent_idx];
    let new_agent = &new_state.agents[acting_agent_idx];
    assert_eq!(new_agent.error_message, Some(expected.to_string()));
    assert_eq!(new_agent.open_menu, old_agent.open_menu);
    assert_eq!(new_agent.node_idx, old_agent.node_idx);
    

    check_inventory(old_state, new_state, &old_agent.inventory);
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
