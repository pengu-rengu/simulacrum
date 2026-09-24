use crate::state::state::{State, Item, ItemStack};
use crate::spec::common::{
    check_agent_unchanged, check_error_message, check_inventory, inventory_with,
    item_label, poi_deposit, poi_yield,
};

pub fn check_harvest(old_state: &State, new_state: &State, poi_idx: usize, tool_idx: Option<usize>, uses: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];
    let pois = &old_state.nodes[acting_agent.node_idx].pois;

    // a harvest never opens or closes a menu
    assert_eq!(new_state.agents[acting_agent_idx].open_menu, acting_agent.open_menu);

    let Some(poi) = pois.get(poi_idx) else {
        check_error_message(new_state, acting_agent_idx, Some(&format!("no poi at index {poi_idx}")));
        check_agent_unchanged(old_state, new_state);
        return;
    };

    let (Some((exposure, stability, _)), Some((harvested_item, needs_pickaxe))) =
        (poi_deposit(poi), poi_yield(poi)) else {
        check_error_message(new_state, acting_agent_idx, Some("nothing to harvest here"));
        check_agent_unchanged(old_state, new_state);
        return;
    };

    if uses == 0 {
        check_error_message(new_state, acting_agent_idx, Some("uses must be at least 1"));
        check_agent_unchanged(old_state, new_state);
        return;
    }

    let tool = match tool_idx {
        None => None,
        Some(tool_idx) => {
            let Some(item_stack) = acting_agent.inventory.get(tool_idx) else {
                check_error_message(
                    new_state,
                    acting_agent_idx,
                    Some(&format!("no item at inventory index {tool_idx}")),
                );
                check_agent_unchanged(old_state, new_state);
                return;
            };
            // the only tool in the game is the pickaxe, and it only bites on a vein
            let useful = matches!(item_stack.item, Item::CrudePickaxe { .. }) && needs_pickaxe;
            if !useful {
                check_error_message(
                    new_state,
                    acting_agent_idx,
                    Some(&format!("{} is no use here", item_label(&item_stack.item))),
                );
                check_agent_unchanged(old_state, new_state);
                return;
            }
            let Item::CrudePickaxe { durability } = item_stack.item else { unreachable!() };
            Some((tool_idx, durability))
        }
    };

    if needs_pickaxe && tool.is_none() {
        check_error_message(new_state, acting_agent_idx, Some("need a crude pickaxe to mine copper ore"));
        check_agent_unchanged(old_state, new_state);
        return;
    }

    if let Some((_, durability)) = tool {
        if durability < uses {
            check_error_message(
                new_state,
                acting_agent_idx,
                Some(&format!(
                    "{} has only {durability} swings left",
                    item_label(&Item::CrudePickaxe { durability })
                )),
            );
            check_agent_unchanged(old_state, new_state);
            return;
        }
    }

    // swinging always wears the tool, whether or not the deposit gives anything back
    let mut expected_inventory = acting_agent.inventory.clone();
    if let Some((tool_idx, durability)) = tool {
        let durability_left = durability - uses;
        if durability_left == 0 {
            expected_inventory.remove(tool_idx);
        } else {
            expected_inventory[tool_idx] = ItemStack {
                item: Item::CrudePickaxe { durability: durability_left },
                count: 1,
            };
        }
    }

    if uses > stability {
        check_error_message(
            new_state,
            acting_agent_idx,
            Some(&format!("too many swings: this takes at most {stability}")),
        );
        check_inventory(old_state, new_state, &expected_inventory);
        return;
    }

    let harvested_count = uses.min(exposure);
    let expected_inventory = inventory_with(&expected_inventory, &harvested_item, harvested_count);
    check_error_message(new_state, acting_agent_idx, None);
    check_inventory(old_state, new_state, &expected_inventory);
}
