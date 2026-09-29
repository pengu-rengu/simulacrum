use crate::state::state::{State, PointOfInterest, Item, ItemStack, DepositType};
use crate::spec::common::{check_error_and_unchanged, item_label};
use std::collections::{HashMap};
use std::iter::zip;

pub fn tool_deposit_type(item: Option<&Item>) -> Option<DepositType> {
    match item {
        Some(Item::CrudePickaxe { .. }) => Some(DepositType::Rock),
        None => Some(DepositType::Forage),
        _ => None,
    }
}

pub fn tool_durability(item: &Item) -> usize {
    match item {
        Item::CrudePickaxe { durability } => *durability,
        _ => panic!("{} is not a tool. this shouldn't be reachable", item_label(item)),
    }
}

pub fn update_tool_durability(item: &mut Item, uses: usize) {
    match item {
        Item::CrudePickaxe { durability } => {
            *durability -= uses;
        }
        _ => {}
    }
}

struct DepositState {
    exposed: usize,
    stability: usize,
    reserves: usize,
    total_yield: usize
}

impl DepositState {
    fn update(&mut self, tool: Option<&Item>) {
        match tool {
            Some(Item::CrudePickaxe { .. }) | None => {
                if self.exposed > 0 {
                    self.exposed -= 1;
                    self.stability -= 1;
                    self.total_yield += 1;
                }
                
            },
            _ => {}
        }
    }
}

pub fn check_harvest(old_state: &State, new_state: &State, poi_idx: usize, tool_idxs: Vec<Option<usize>>) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];

    let Some(poi) = old_state.nodes[acting_agent.node_idx].pois.get(poi_idx) else {
        check_error_and_unchanged(old_state, new_state, &format!("no poi at index {poi_idx}"));
        return;
    };

    let PointOfInterest::ResourceDeposit { name: _, type_, exposed, stability, reserves, yield_ } = poi else { 
        check_error_and_unchanged(old_state, new_state, "cannot harvest this poi");
        return;
    };

    let mut state = DepositState {
        exposed: *exposed,
        stability: *stability,
        reserves: *reserves,
        total_yield: 0
    };
    let mut tool_uses = HashMap::<usize, usize>::new();
    let mut collapsed = false;

    for tool_idx in tool_idxs {
        let tool = if let Some(idx) = tool_idx {
            let Some(item_stack) = acting_agent.inventory.get(idx) else {
                check_error_and_unchanged(old_state, new_state, &format!("no item at inventory index {idx}"));
                return;
            };
            Some(&item_stack.item)
        } else { None };
        if Some(type_) != tool_deposit_type(tool).as_ref() {
            check_error_and_unchanged(old_state, new_state, &format!("this tool has no use here"));
            return;
        }
        
        state.update(tool);
        if let Some(idx) = tool_idx {
            let uses = tool_uses.entry(idx).or_insert(0);
            *uses += 1;

            if *uses >= tool_durability(&tool.unwrap()) {
                break;
            }
        }

        if state.stability <= 0 {
            collapsed = true;
            break;
        }
    };

    let mut yield_in_inventory = false;
    let mut expected_inventory = acting_agent.inventory.clone();
    for (i, item_stack) in expected_inventory.iter_mut().enumerate() {
        if tool_uses.contains_key(&i) {
            update_tool_durability(&mut item_stack.item, tool_uses[&i]);
        }
        if &item_stack.item == yield_ {
            item_stack.count += state.total_yield;
            yield_in_inventory = true;
        }
    }
    if !yield_in_inventory {
        let new_item_stack = ItemStack {
            item: yield_.clone(),
            count: state.total_yield,
        };
        expected_inventory.push(new_item_stack);
    }
    
    // a pickaxe worn down to nothing breaks
    expected_inventory.retain(|item_stack| {
        item_stack.count > 0 && !matches!(item_stack.item, Item::CrudePickaxe { durability: 0 })
    });

    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.inventory, expected_inventory);
            if collapsed {
                assert_eq!(new_agent.error_message, Some(format!("deposit collapsed")));
            } else {
                assert_eq!(new_agent.error_message, None);
            }
        } else {
            assert_eq!(new_agent.inventory, old_agent.inventory);
            assert_eq!(new_agent.error_message, old_agent.error_message);
        }
        assert_eq!(new_agent.open_menu, old_agent.open_menu);
        assert_eq!(new_agent.node_idx, old_agent.node_idx);
    }

}