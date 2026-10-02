use crate::spec::common::tool_durability;
use crate::state::state::Item;
use crate::state::agent::Agent;

pub fn check_inventory(agent: &Agent) {
    let nodeworld_agent = &agent.nodeworld;
    for item_stack in &nodeworld_agent.inventory {
        if let Item::Tool { attributes, .. } = &item_stack.item {
            assert!(attributes.iter().all(|(_, amount)| *amount > 0));
        }
        if let Some(durability) = tool_durability(&item_stack.item) {
            assert!(durability > 0);
            assert!(item_stack.count == 1);
        }
    }

    let items = nodeworld_agent.inventory.iter().map(|item_stack| &item_stack.item).collect::<Vec<&Item>>();
    // tools stay unstacked; every other item has one stack
    for item_stack in nodeworld_agent.inventory.iter() {
        assert!(item_stack.count > 0);
        if tool_durability(&item_stack.item).is_none() {
            assert_eq!(items.iter().filter(|item| ***item == item_stack.item).count(), 1);
        }
    }
}
