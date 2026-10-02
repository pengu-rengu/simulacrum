use crate::state::state::{State, Input, Action, Message, Item, ItemStack, PointOfInterest, DepositType, Menu, CombatEncounter};
use serde_json::from_str;

pub fn flush_state(state: &State) -> (State, String) {
    let mut new_state = state.clone();
    let agent_idx = new_state.agent_idx;
    let agent = &state.agents[agent_idx];

    // the spec cannot be imported from here, so only the label formatting is restated
    let item_label = |item: &Item| match item {
        Item::CrudePickaxe { durability } => format!("CrudePickaxe (durability {durability})"),
        Item::CopperPickaxe { durability } => format!("CopperPickaxe (durability {durability})"),
        Item::CopperDrill { durability } => format!("CopperDrill (durability {durability})"),
        _ => format!("{:?}", item)
    };

    let mut lines = vec![];

    match &agent.open_menu {
        // an open menu replaces the node view, but the agent still holds items and still hears the room
        Some(Menu::CraftingMenu { recipes }) => {
            lines.push("Crafting menu: craft or exit".to_string());
            lines.push("".to_string());

            lines.push("Recipes:".to_string());
            for (recipe_idx, recipe) in recipes.iter().enumerate() {
                let ingredients = recipe.ingredients.iter()
                    .map(|ingredient| format!("{} {}", ingredient.count, item_label(&ingredient.item)))
                    .collect::<Vec<String>>()
                    .join(", ");
                lines.push(format!("[{}] {} <- {}", recipe_idx, item_label(&recipe.output.item), ingredients));
            }
            lines.push("".to_string());
        }
        None => {
            let node = &state.nodes[agent.node_idx];
            lines.push(format!("Node: {} ({})", node.name, node.biome));
            lines.push("".to_string());

            lines.push("Agents:".to_string());
            for other_agent in &state.agents {
                if other_agent.node_idx != agent.node_idx { continue; }
                lines.push(other_agent.name.clone());
            }
            lines.push("".to_string());

            lines.push("POIs:".to_string());
            for (poi_idx, poi) in node.pois.iter().enumerate() {
                let label = match poi {
                    PointOfInterest::ResourceDeposit { name, type_, exposed, stability, reserves, .. } => {
                        let type_label = match type_ {
                            DepositType::Rock => "rock",
                            DepositType::Forage => "forage"
                        };
                        format!("{} ({}, exposed {}, stability {}, reserves {})", name, type_label, exposed, stability, reserves)
                    }
                    PointOfInterest::Inspectable { name, .. } => name.clone(),
                    PointOfInterest::EnemyGroup { name, enemies } => {
                        let enemy_names = enemies.iter()
                            .map(|enemy| enemy.name.clone())
                            .collect::<Vec<String>>()
                            .join(", ");
                        format!("{} (enemies: {})", name, enemy_names)
                    }
                };
                lines.push(format!("[{}] {}", poi_idx, label));
            }
            lines.push("".to_string());

            // each fight is listed under the enemy group it is against
            lines.push("Combat encounters:".to_string());
            for CombatEncounter::Pve { enemy_group_poi_idx, agent_idxs, .. } in &node.combat_encounters {
                let group_name = match &node.pois[*enemy_group_poi_idx] {
                    PointOfInterest::EnemyGroup { name, .. } => name.clone(),
                    _ => String::new()
                };
                let fighters = agent_idxs.iter()
                    .map(|agent_idx| state.agents[*agent_idx].name.clone())
                    .collect::<Vec<String>>()
                    .join(", ");
                lines.push(format!("[{}] {}: {}", enemy_group_poi_idx, group_name, fighters));
            }
            lines.push("".to_string());
        }
    }

    lines.push("Inventory:".to_string());
    for item_stack in &agent.inventory {
        lines.push(format!("{} {}", item_stack.count, item_label(&item_stack.item)));
    }
    lines.push("".to_string());

    lines.push("Messages:".to_string());
    for message in &agent.node_messages_inbox {
        lines.push(format!("[{}] {}", state.agents[message.sender_agent_idx].name, message.content));
    }

    if let Some(error_message) = &agent.error_message {
        lines.push("".to_string());
        lines.push(format!("Error: {}", error_message));
    }

    let output = lines.join("\n");
    new_state.agents[agent_idx].node_messages_inbox.clear();
    (new_state, output)
}

pub fn next_state(state: &State, input_str: &str) -> State {
    let mut new_state = state.clone();
    let curr_agent_idx = state.agent_idx;
    let curr_agent = &state.agents[curr_agent_idx];
    let curr_node_idx = curr_agent.node_idx;

    match from_str::<Input>(input_str) {
        Err(_) => {
            new_state.agents[curr_agent_idx].error_message = Some("could not parse input json".to_string());
        }
        Ok(input) => {
            // each action either succeeds or fails with nothing about the agent changed
            // an agent fights in at most one encounter, wherever it is
            let in_combat = state.nodes.iter()
                .flat_map(|node| &node.combat_encounters)
                .any(|CombatEncounter::Pve { agent_idxs, .. }| agent_idxs.contains(&curr_agent_idx));

            let result: Result<(), String> = match input.action {
                Some(Action::MoveTo(_)) | Some(Action::Harvest { .. }) | Some(Action::Inspect { .. }) | Some(Action::Engage { .. })
                    if curr_agent.open_menu.is_some() =>
                {
                    Err("you cannot do that while a menu is open".to_string())
                }
                Some(Action::MoveTo(_)) | Some(Action::Harvest { .. }) | Some(Action::Inspect { .. }) | Some(Action::Engage { .. })
                    if in_combat =>
                {
                    Err("you cannot do that while in combat".to_string())
                }
                Some(Action::MoveTo(node_name)) => {
                    match state.nodes.iter().position(|node| node.name == node_name) {
                        Some(target_node_idx) => {
                            new_state.agents[curr_agent_idx].node_idx = target_node_idx;
                            Ok(())
                        }
                        None => Err(format!("no node named {node_name}"))
                    }
                }
                Some(Action::Harvest { poi_idx, tool_idxs }) => 'harvest: {
                    let Some(poi) = state.nodes[curr_node_idx].pois.get(poi_idx) else {
                        break 'harvest Err(format!("no poi at index {poi_idx}"));
                    };
                    let PointOfInterest::ResourceDeposit { type_, exposed, stability, reserves, yield_, .. } = poi else {
                        break 'harvest Err("cannot harvest this poi".to_string());
                    };

                    // swings are simulated on a copy; the deposit itself never changes
                    let (mut exposed, mut stability, mut reserves) = (*exposed, *stability, *reserves);
                    let mut total_yield = 0;
                    let mut tool_uses = vec![0; curr_agent.inventory.len()];
                    let mut collapsed = false;

                    for tool_idx in tool_idxs {
                        let tool = match tool_idx {
                            None => None,
                            Some(idx) => match curr_agent.inventory.get(idx) {
                                Some(item_stack) => Some(&item_stack.item),
                                None => break 'harvest Err(format!("no item at inventory index {idx}"))
                            }
                        };
                        // digging tools take an exposed unit at a stability cost; the drill exposes one from reserves
                        let (stability_cost, durability) = match (type_, tool) {
                            (DepositType::Forage, None) => (1, None),
                            (DepositType::Rock, Some(Item::CrudePickaxe { durability })) => (5, Some(*durability)),
                            (DepositType::Rock, Some(Item::CopperPickaxe { durability })) => (3, Some(*durability)),
                            (DepositType::Rock, Some(Item::CopperDrill { durability })) => (0, Some(*durability)),
                            _ => break 'harvest Err("this tool has no use here".to_string())
                        };

                        if matches!(tool, Some(Item::CopperDrill { .. })) {
                            if reserves > 0 {
                                reserves -= 1;
                                exposed += 1;
                            }
                        } else if exposed > 0 {
                            exposed -= 1;
                            stability = stability.saturating_sub(stability_cost);
                            total_yield += 1;
                        }
                        if let Some(idx) = tool_idx {
                            tool_uses[idx] += 1;
                        }

                        // a collapse wins even on the swing that wears the tool out
                        if stability == 0 {
                            collapsed = true;
                            break;
                        }

                        // a tool that runs out of durability ends the harvest
                        if let (Some(idx), Some(durability)) = (tool_idx, durability) {
                            if tool_uses[idx] >= durability { break; }
                        }
                    }

                    let inventory = &mut new_state.agents[curr_agent_idx].inventory;
                    for (item_stack, uses) in inventory.iter_mut().zip(&tool_uses) {
                        if let Item::CrudePickaxe { durability }
                            | Item::CopperPickaxe { durability }
                            | Item::CopperDrill { durability } = &mut item_stack.item
                        {
                            *durability -= uses;
                        }
                    }
                    // a collapse buries everything the harvest dug up
                    if !collapsed {
                        match inventory.iter_mut().find(|item_stack| item_stack.item == *yield_) {
                            Some(item_stack) => item_stack.count += total_yield,
                            None => inventory.push(ItemStack { item: yield_.clone(), count: total_yield })
                        }
                    }
                    // a tool worn down to nothing breaks
                    inventory.retain(|item_stack| {
                        item_stack.count > 0 && !matches!(
                            item_stack.item,
                            Item::CrudePickaxe { durability: 0 } | Item::CopperPickaxe { durability: 0 } | Item::CopperDrill { durability: 0 }
                        )
                    });

                    if collapsed { Err("deposit collapsed".to_string()) } else { Ok(()) }
                }
                Some(Action::Inspect { poi_idx }) => {
                    match state.nodes[curr_node_idx].pois.get(poi_idx) {
                        None => Err(format!("no poi at index {poi_idx}")),
                        Some(PointOfInterest::Inspectable { menu, .. }) => {
                            new_state.agents[curr_agent_idx].open_menu = Some(menu.clone());
                            Ok(())
                        }
                        Some(_) => Err("nothing to inspect here".to_string())
                    }
                }
                Some(Action::Craft(recipe_idx)) => 'craft: {
                    let Some(Menu::CraftingMenu { recipes }) = &curr_agent.open_menu else {
                        break 'craft Err("crafting menu not open".to_string());
                    };
                    let Some(recipe) = recipes.get(recipe_idx) else {
                        break 'craft Err(format!("no recipe at index {recipe_idx}"));
                    };

                    let mut inventory = curr_agent.inventory.clone();
                    for ingredient in &recipe.ingredients {
                        match inventory.iter_mut().find(|item_stack| item_stack.item == ingredient.item) {
                            Some(item_stack) if item_stack.count >= ingredient.count => {
                                item_stack.count -= ingredient.count;
                            }
                            _ => break 'craft Err("not enough items to craft recipe".to_string())
                        }
                    }
                    // tools stay unstacked; every other item joins its existing stack
                    let existing_stack = match recipe.output.item {
                        Item::CrudePickaxe { .. } | Item::CopperPickaxe { .. } | Item::CopperDrill { .. } => None,
                        _ => inventory.iter_mut().find(|item_stack| item_stack.item == recipe.output.item)
                    };
                    match existing_stack {
                        Some(item_stack) => item_stack.count += recipe.output.count,
                        None => inventory.push(recipe.output.clone())
                    }
                    inventory.retain(|item_stack| item_stack.count > 0);

                    new_state.agents[curr_agent_idx].inventory = inventory;
                    Ok(())
                }
                Some(Action::Exit) => {
                    if curr_agent.open_menu.is_some() {
                        new_state.agents[curr_agent_idx].open_menu = None;
                        Ok(())
                    } else {
                        Err("nothing to exit".to_string())
                    }
                }
                Some(Action::Engage { poi_idx }) => 'engage: {
                    let Some(poi) = state.nodes[curr_node_idx].pois.get(poi_idx) else {
                        break 'engage Err(format!("no poi at index {poi_idx}"));
                    };
                    let PointOfInterest::EnemyGroup { enemies, .. } = poi else {
                        break 'engage Err("nothing to engage here".to_string());
                    };

                    // join the fight already running against this group, or start one against fresh clones
                    let encounters = &mut new_state.nodes[curr_node_idx].combat_encounters;
                    let existing = encounters.iter_mut()
                        .find(|CombatEncounter::Pve { enemy_group_poi_idx, .. }| *enemy_group_poi_idx == poi_idx);
                    match existing {
                        Some(CombatEncounter::Pve { agent_idxs, .. }) => agent_idxs.push(curr_agent_idx),
                        None => encounters.push(CombatEncounter::Pve {
                            enemy_group_poi_idx: poi_idx,
                            agent_idxs: vec![curr_agent_idx],
                            enemies: enemies.clone()
                        })
                    }
                    Ok(())
                }
                None => Ok(())
            };
            new_state.agents[curr_agent_idx].error_message = result.err();

            if let Some(content) = input.send_message {
                // the message reaches whoever shared the node at the start of the turn
                for (agent_idx, agent) in new_state.agents.iter_mut().enumerate() {
                    if state.agents[agent_idx].node_idx != curr_node_idx { continue; }
                    agent.node_messages_inbox.push(Message {
                        sender_agent_idx: curr_agent_idx,
                        content: content.clone()
                    });
                }
            }
        }
    }

    new_state.agent_idx += 1;
    if new_state.agent_idx == new_state.agents.len() {
        new_state.agent_idx = 0;
        new_state.turn += 1;
    }
    new_state
}
