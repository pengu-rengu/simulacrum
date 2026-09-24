use crate::state::state::{State, Item, ItemStack, Recipe, PointOfInterest};
use std::iter::zip;

pub fn workbench_recipes() -> Vec<Recipe> {
    vec![Recipe {
        output: ItemStack { item: Item::CrudePickaxe { durability: 20 }, count: 1 },
        ingredients: vec![
            ItemStack { item: Item::Stick, count: 10 },
            ItemStack { item: Item::SmoothPebble, count: 10 },
        ],
    }]
}

pub fn item_label(item: &Item) -> String {
    match item {
        Item::CrudePickaxe { durability } => format!("CrudePickaxe (durability {durability})"),
        _ => format!("{:?}", item),
    }
}

pub fn poi_label(poi: &PointOfInterest) -> String {
    let name = match poi {
        PointOfInterest::Thornbush { .. } => "Thornbush",
        PointOfInterest::AmberBole { .. } => "AmberBole",
        PointOfInterest::SmoothPebble { .. } => "SmoothPebble",
        PointOfInterest::CopperOreVein { .. } => "CopperOreVein",
        PointOfInterest::RuinedWorkbench => return "RuinedWorkbench".to_string(),
    };
    let (exposure, stability, reserves) = poi_deposit(poi).unwrap();
    format!("{name} (exposure {exposure}, stability {stability}, reserves {reserves})")
}

/// The constants a deposit is worked against, or None for something that cannot be harvested.
pub fn poi_deposit(poi: &PointOfInterest) -> Option<(usize, usize, usize)> {
    match poi {
        PointOfInterest::Thornbush { exposure, stability, reserves }
        | PointOfInterest::AmberBole { exposure, stability, reserves }
        | PointOfInterest::SmoothPebble { exposure, stability, reserves }
        | PointOfInterest::CopperOreVein { exposure, stability, reserves } => {
            Some((*exposure, *stability, *reserves))
        }
        PointOfInterest::RuinedWorkbench => None,
    }
}

/// What one swing yields, and whether a crude pickaxe is required for it.
pub fn poi_yield(poi: &PointOfInterest) -> Option<(Item, bool)> {
    match poi {
        PointOfInterest::Thornbush { .. } => Some((Item::Stick, false)),
        PointOfInterest::AmberBole { .. } => Some((Item::Resin, false)),
        PointOfInterest::SmoothPebble { .. } => Some((Item::SmoothPebble, false)),
        PointOfInterest::CopperOreVein { .. } => Some((Item::CopperOre, true)),
        PointOfInterest::RuinedWorkbench => None,
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
        _ => match new_inventory.iter_mut().find(|item_stack| item_stack.item == *item) {
            Some(item_stack) => item_stack.count += count,
            None => new_inventory.push(ItemStack { item: item.clone(), count }),
        },
    }
    new_inventory
}

pub fn check_body_grids_unchanged(old_state: &State, new_state: &State) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        assert_eq!(old_agent.body_grid, new_agent.body_grid);
    }
}

/// Every agent keeps its node and inventory, except the acting agent when `acting_agent_changes`.
pub fn check_agents_idle(old_state: &State, new_state: &State, acting_agent_changes: bool) {
    for (agent_idx, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if acting_agent_changes && agent_idx == old_state.agent_idx { continue; }
        assert_eq!(old_agent.node_idx, new_agent.node_idx);
        assert_eq!(old_agent.inventory, new_agent.inventory);
        assert_eq!(old_agent.open_menu, new_agent.open_menu);
    }
}

pub fn check_error_message(new_state: &State, acting_agent_idx: usize, expected: Option<&str>) {
    let expected_error_msg = expected.map(|error_message| error_message.to_string());
    assert_eq!(new_state.agents[acting_agent_idx].error_message, expected_error_msg);
}

/// The acting agent's inventory is exactly `expected_inventory`, and nothing else moved.
pub fn check_inventory(old_state: &State, new_state: &State, expected_inventory: &Vec<ItemStack>) {
    let acting_agent_idx = old_state.agent_idx;
    assert_eq!(new_state.agents[acting_agent_idx].inventory, *expected_inventory);
    assert_eq!(old_state.agents[acting_agent_idx].node_idx, new_state.agents[acting_agent_idx].node_idx);
    check_agents_idle(old_state, new_state, true);
}

/// The acting agent changed nothing about itself but its error message.
pub fn check_agent_unchanged(old_state: &State, new_state: &State) {
    let acting_agent_idx = old_state.agent_idx;
    let old_agent = &old_state.agents[acting_agent_idx];
    assert_eq!(new_state.agents[acting_agent_idx].open_menu, old_agent.open_menu);
    check_inventory(old_state, new_state, &old_agent.inventory);
}

/// Nothing an agent does changes the world: deposits are constant.
pub fn check_nodes_unchanged(old_state: &State, new_state: &State) {
    assert_eq!(old_state.nodes.len(), new_state.nodes.len());
    for (old_node, new_node) in zip(&old_state.nodes, &new_state.nodes) {
        assert_eq!(old_node.name, new_node.name);
        assert_eq!(old_node.biome, new_node.biome);
        assert_eq!(old_node.pois, new_node.pois);
    }
}
