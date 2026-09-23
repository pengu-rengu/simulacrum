use crate::state::state::{State, Input, Action, Message, Item, ItemStack, PointOfInterest};
use serde_json::from_str;
use std::iter::zip;

pub fn check_increment(old_state: &State, new_state: &State) {
    let new_idx = old_state.agent_idx + 1;
    if new_idx == old_state.agents.len() {
        assert_eq!(new_state.agent_idx, 0);
        assert_eq!(new_state.turn, old_state.turn + 1);
    } else {
        assert_eq!(new_state.agent_idx, new_idx);
        assert_eq!(new_state.turn, old_state.turn)
    }
}

pub fn check_body_grids_unchanged(old_state: &State, new_state: &State) {
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        assert_eq!(old_agent.body_grid, new_agent.body_grid);
    }
}

pub fn item_count(agent_inventory: &Vec<ItemStack>, item: &Item) -> usize {
    agent_inventory.iter()
        .find(|item_stack| item_stack.item == *item)
        .map(|item_stack| item_stack.count)
        .unwrap_or(0)
}

/// Every agent keeps its node and inventory, except the acting agent when `acting_agent_changes`.
pub fn check_agents_idle(old_state: &State, new_state: &State, acting_agent_changes: bool) {
    for (agent_idx, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if acting_agent_changes && agent_idx == old_state.agent_idx { continue; }
        assert_eq!(old_agent.node_idx, new_agent.node_idx);
        assert_eq!(old_agent.inventory, new_agent.inventory);
    }
}

pub fn check_error_message(new_state: &State, acting_agent_idx: usize, expected: Option<&str>) {
    assert_eq!(
        new_state.agents[acting_agent_idx].error_message,
        expected.map(|error_message| error_message.to_string())
    );
}

/// The acting agent's inventory changed by exactly `deltas`, and nothing else moved or gained items.
pub fn check_inventory_deltas(old_state: &State, new_state: &State, deltas: &[(Item, i64)]) {
    let acting_agent_idx = old_state.agent_idx;
    let old_inventory = &old_state.agents[acting_agent_idx].inventory;
    let new_inventory = &new_state.agents[acting_agent_idx].inventory;

    for item in [Item::Stick, Item::Resin, Item::SmoothPebble, Item::CopperOre, Item::CrudePickaxe] {
        let delta = deltas.iter()
            .find(|(delta_item, _)| *delta_item == item)
            .map(|(_, delta)| *delta)
            .unwrap_or(0);
        let expected = (item_count(old_inventory, &item) as i64 + delta) as usize;
        assert_eq!(item_count(new_inventory, &item), expected);
    }

    assert_eq!(old_state.agents[acting_agent_idx].node_idx, new_state.agents[acting_agent_idx].node_idx);
    check_agents_idle(old_state, new_state, true);
}

pub fn check_move_to(old_state: &State, new_state: &State, node_name: &str) {
    let acting_agent_idx = old_state.agent_idx;
    let target_node_idx = old_state.nodes.iter().position(|node| node.name == node_name);

    match target_node_idx {
        Some(target_node_idx) => {
            assert_eq!(new_state.agents[acting_agent_idx].node_idx, target_node_idx);
            check_error_message(new_state, acting_agent_idx, None);
        }
        None => {
            assert_eq!(
                new_state.agents[acting_agent_idx].node_idx,
                old_state.agents[acting_agent_idx].node_idx
            );
            check_error_message(new_state, acting_agent_idx, Some(&format!("no node named {node_name}")));
        }
    }

    assert_eq!(old_state.agents[acting_agent_idx].inventory, new_state.agents[acting_agent_idx].inventory);
    check_agents_idle(old_state, new_state, true);
}

pub fn check_interact(old_state: &State, new_state: &State, poi_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];
    let pois = &old_state.nodes[acting_agent.node_idx].pois;

    let Some(poi) = pois.get(poi_idx) else {
        check_error_message(new_state, acting_agent_idx, Some(&format!("no poi at index {poi_idx}")));
        check_inventory_deltas(old_state, new_state, &[]);
        return;
    };

    match poi {
        PointOfInterest::Thornbush => {
            check_error_message(new_state, acting_agent_idx, None);
            check_inventory_deltas(old_state, new_state, &[(Item::Stick, 1)]);
        }
        PointOfInterest::AmberBole => {
            check_error_message(new_state, acting_agent_idx, None);
            check_inventory_deltas(old_state, new_state, &[(Item::Resin, 1)]);
        }
        PointOfInterest::SmoothPebble => {
            check_error_message(new_state, acting_agent_idx, None);
            check_inventory_deltas(old_state, new_state, &[(Item::SmoothPebble, 1)]);
        }
        PointOfInterest::CopperOreVein => {
            if item_count(&acting_agent.inventory, &Item::CrudePickaxe) > 0 {
                check_error_message(new_state, acting_agent_idx, None);
                check_inventory_deltas(old_state, new_state, &[(Item::CopperOre, 1)]);
            } else {
                check_error_message(new_state, acting_agent_idx, Some("need a crude pickaxe to mine copper ore"));
                check_inventory_deltas(old_state, new_state, &[]);
            }
        }
        PointOfInterest::RuinedWorkbench => {
            let enough_sticks = item_count(&acting_agent.inventory, &Item::Stick) >= 10;
            let enough_pebbles = item_count(&acting_agent.inventory, &Item::SmoothPebble) >= 10;
            if enough_sticks && enough_pebbles {
                check_error_message(new_state, acting_agent_idx, None);
                check_inventory_deltas(old_state, new_state, &[
                    (Item::Stick, -10),
                    (Item::SmoothPebble, -10),
                    (Item::CrudePickaxe, 1),
                ]);
            } else {
                check_error_message(
                    new_state,
                    acting_agent_idx,
                    Some("need 10 stick and 10 smooth pebble to craft a crude pickaxe"),
                );
                check_inventory_deltas(old_state, new_state, &[]);
            }
        }
    }
}

pub fn check_send_message(old_state: &State, new_state: &State, content: &str) {
    let curr_node_idx = old_state.agents[old_state.agent_idx].node_idx;
    for (old_agent, new_agent) in zip(&old_state.agents, &new_state.agents) {
        if old_agent.node_idx != curr_node_idx { continue; }

        if let Some((last, rest)) = new_agent.node_messages_inbox.split_last() {
            let new_message = Message {
                sender_agent_idx: old_state.agent_idx,
                content: content.to_string()
            };
            assert_eq!(*last, new_message);
            assert_eq!(rest, old_agent.node_messages_inbox);
        } else {
            panic!("new messages cannot be empty")
        }
    }
}

pub fn check_next_state(old_state: &State, new_state: &State, input_str: &str) {
    check_increment(old_state, new_state);
    check_body_grids_unchanged(old_state, new_state);
    let Ok(input) = from_str::<Input>(input_str) else {
        check_error_message(new_state, old_state.agent_idx, Some("could not parse input json"));
        return;
    };

    match input.action {
        Some(Action::MoveTo(node_name)) => check_move_to(old_state, new_state, &node_name),
        Some(Action::Interact(poi_idx)) => check_interact(old_state, new_state, poi_idx),
        None => {
            check_error_message(new_state, old_state.agent_idx, None);
            check_agents_idle(old_state, new_state, false);
        }
    }

    if let Some(message) = input.send_message {
        check_send_message(old_state, new_state, &message);
    }
}

pub fn check_flush_state(old_state: &State, new_state: &State, output: &str) {
    assert_eq!(old_state.agent_idx, new_state.agent_idx);
    check_body_grids_unchanged(old_state, new_state);
    let agent_idx = new_state.agent_idx;
    let agent = &old_state.agents[agent_idx];
    let node = &old_state.nodes[agent.node_idx];

    let mut lines = vec![format!("Node: {} ({})", node.name, node.biome), "".to_string()];

    lines.push("Agents:".to_string());
    for other_agent in &old_state.agents {
        if other_agent.node_idx != agent.node_idx { continue; }
        lines.push(other_agent.name.clone());
    }
    lines.push("".to_string());

    lines.push("POIs:".to_string());
    for (poi_idx, poi) in node.pois.iter().enumerate() {
        lines.push(format!("[{}] {:?}", poi_idx, poi));
    }
    lines.push("".to_string());

    lines.push("Inventory:".to_string());
    for item_stack in &agent.inventory {
        lines.push(format!("{} {:?}", item_stack.count, item_stack.item));
    }
    lines.push("".to_string());

    lines.push("Messages:".to_string());
    for message in &agent.node_messages_inbox {
        lines.push(format!("[{}] {}", old_state.agents[message.sender_agent_idx].name, message.content));
    }

    if let Some(error_message) = &new_state.agents[agent_idx].error_message {
        lines.push("".to_string());
        lines.push(format!("Error: {}", error_message));
    }

    let expected_output = lines.join("\n");
    assert_eq!(output, expected_output);
    assert!(new_state.agents[agent_idx].node_messages_inbox.is_empty());
}
