use crate::state::state::State; 
use crate::state::nodeworld::{Item, ItemStack, CombatEncounter, DepositType, ToolAttribute};
use std::iter::zip;

/// Durability of a tool, or None for items that are not tools.
pub fn tool_durability(item: &Item) -> Option<usize> {
    match item {
        Item::Tool { durability, .. } => Some(*durability),
        Item::Material { .. } => None
    }
}

pub fn deposit_type_label(deposit_type: &DepositType) -> &'static str {
    match deposit_type {
        DepositType::Rock => "rock",
        DepositType::Forage => "forage"
    }
}

/// A material by name; a tool with what it works on, what each swing does, and how many swings it has left.
pub fn item_label(item: &Item) -> String {
    match item {
        Item::Material { name } => name.clone(),
        Item::Tool { name, deposit_type, attributes, durability } => {
            let mut parts = vec![deposit_type_label(deposit_type).to_string()];
            for (attribute, amount) in attributes {
                let attribute_label = match attribute {
                    ToolAttribute::Chipping => "chipping",
                    ToolAttribute::Drilling => "drilling"
                };
                parts.push(format!("{attribute_label} {amount}"));
            }
            parts.push(format!("durability {durability}"));
            format!("{} ({})", name, parts.join(", "))
        }
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

/// Why the acting agent cannot move, harvest, inspect or engage right now, if it cannot.
pub fn action_blocked(state: &State) -> Option<&'static str> {
    let acting_agent_idx = state.agent_idx;
    if state.agents[acting_agent_idx].nodeworld.open_menu.is_some() {
        return Some("you cannot do that while a menu is open");
    }

    let in_combat = state.nodes.iter()
        .flat_map(|node| &node.combat_encounters)
        .any(|CombatEncounter::Pve { agent_idxs, .. }| agent_idxs.contains(&acting_agent_idx));
    if in_combat {
        return Some("you cannot do that while in combat");
    }

    None
}

pub fn check_error_and_nodeworld_unchanged(old_state: &State, new_state: &State, expected: &str) {
    let acting_agent_idx = old_state.agent_idx;
    let old_agent = &old_state.agents[acting_agent_idx];
    let new_agent = &new_state.agents[acting_agent_idx];
    assert_eq!(new_agent.error_message, Some(expected.to_string()));
    assert_eq!(new_agent.nodeworld.open_menu, old_agent.nodeworld.open_menu);
    assert_eq!(new_agent.nodeworld.node_idx, old_agent.nodeworld.node_idx);
    assert_eq!(new_agent.nodeworld.inventory, old_agent.nodeworld.inventory);
}

pub fn check_body_grids_unchanged(old_state: &State, new_state: &State) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        assert_eq!(old_agent.nodeworld.body_grid, new_agent.nodeworld.body_grid);
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
