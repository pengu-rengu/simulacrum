use crate::state::state::{State, Input, Action, Message, Item, ItemStack, PoiKind, Menu, POI_YIELDS, WORKBENCH_RECIPES};
use serde_json::from_str;

pub fn flush_state(state: &State) -> (State, String) {
    let mut new_state = state.clone();
    let agent_idx = new_state.agent_idx;
    let agent = &state.agents[agent_idx];

    // the spec cannot be imported from here, so only the label formatting is restated
    let item_label = |item: &Item| match item {
        Item::CrudePickaxe { durability } => format!("CrudePickaxe (durability {durability})"),
        _ => format!("{:?}", item),
    };

    let mut lines = vec![];

    match agent.open_menu {
        // an open menu replaces the node view, but the agent still holds items and still hears the room
        Some(Menu::Workbench) => {
            lines.push("Workbench: craft or exit".to_string());
            lines.push("".to_string());

            lines.push("Recipes:".to_string());
            for (recipe_idx, recipe) in WORKBENCH_RECIPES.iter().enumerate() {
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
                let label = match &poi.deposit {
                    None => format!("{:?}", poi.kind),
                    Some(deposit) => format!(
                        "{:?} (exposure {}, stability {}, reserves {})",
                        poi.kind, deposit.exposure, deposit.stability, deposit.reserves
                    ),
                };
                lines.push(format!("[{}] {}", poi_idx, label));
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
    let curr_node_idx = state.agents[curr_agent_idx].node_idx;
    let menu_open = state.agents[curr_agent_idx].open_menu.is_some();

    let item_label = |item: &Item| match item {
        Item::CrudePickaxe { durability } => format!("CrudePickaxe (durability {durability})"),
        _ => format!("{:?}", item),
    };

    match from_str::<Input>(input_str) {
        Err(_) => {
            new_state.agents[curr_agent_idx].error_message = Some("could not parse input json".to_string());
        }
        Ok(input) => {
            new_state.agents[curr_agent_idx].error_message = None;

            match input.action {
                Some(Action::MoveTo(_)) | Some(Action::Harvest { .. }) | Some(Action::Inspect { .. })
                    if menu_open =>
                {
                    new_state.agents[curr_agent_idx].error_message =
                        Some("you are at the ruined workbench: craft or exit".to_string());
                }
                Some(Action::MoveTo(node_name)) => {
                    match state.nodes.iter().position(|node| node.name == node_name) {
                        Some(target_node_idx) => new_state.agents[curr_agent_idx].node_idx = target_node_idx,
                        None => {
                            new_state.agents[curr_agent_idx].error_message =
                                Some(format!("no node named {node_name}"));
                        }
                    }
                }
                Some(Action::Harvest { poi_idx, tool_idx, uses }) => {
                    // resolved step by step: the first failure ends the turn with nothing else changed
                    let error_message = 'harvest: {
                        let Some(poi) = state.nodes[curr_node_idx].pois.get(poi_idx) else {
                            break 'harvest Some(format!("no poi at index {poi_idx}"));
                        };
                        let Some((_, harvested_item, needs_pickaxe)) =
                            POI_YIELDS.iter().find(|(kind, _, _)| *kind == poi.kind)
                        else {
                            break 'harvest Some("nothing to harvest here".to_string());
                        };
                        let deposit = poi.deposit.as_ref().unwrap();
                        if uses == 0 {
                            break 'harvest Some("uses must be at least 1".to_string());
                        }

                        let tool = match tool_idx {
                            None => None,
                            Some(tool_idx) => {
                                let Some(item_stack) = state.agents[curr_agent_idx].inventory.get(tool_idx)
                                else {
                                    break 'harvest Some(format!("no item at inventory index {tool_idx}"));
                                };
                                let is_pickaxe = matches!(item_stack.item, Item::CrudePickaxe { .. });
                                if !is_pickaxe || !*needs_pickaxe {
                                    break 'harvest Some(format!(
                                        "{} is no use here",
                                        item_label(&item_stack.item)
                                    ));
                                }
                                let Item::CrudePickaxe { durability } = item_stack.item else {
                                    unreachable!()
                                };
                                Some((tool_idx, durability))
                            }
                        };

                        if *needs_pickaxe && tool.is_none() {
                            break 'harvest Some("need a crude pickaxe to mine copper ore".to_string());
                        }
                        if let Some((_, durability)) = tool {
                            if durability < uses {
                                break 'harvest Some(format!(
                                    "{} has only {durability} swings left",
                                    item_label(&Item::CrudePickaxe { durability })
                                ));
                            }
                        }

                        // swinging wears the tool whether or not the deposit gives anything back
                        let inventory = &mut new_state.agents[curr_agent_idx].inventory;
                        if let Some((tool_idx, durability)) = tool {
                            let durability_left = durability - uses;
                            if durability_left == 0 {
                                inventory.remove(tool_idx);
                            } else {
                                inventory[tool_idx] = ItemStack {
                                    item: Item::CrudePickaxe { durability: durability_left },
                                    count: 1,
                                };
                            }
                        }

                        if uses > deposit.stability {
                            break 'harvest Some(format!(
                                "too many swings: this takes at most {}",
                                deposit.stability
                            ));
                        }

                        let harvested_count = uses.min(deposit.exposure);
                        if harvested_count > 0 {
                            match inventory.iter_mut().find(|item_stack| item_stack.item == *harvested_item) {
                                Some(item_stack) => item_stack.count += harvested_count,
                                None => inventory.push(ItemStack {
                                    item: harvested_item.clone(),
                                    count: harvested_count,
                                }),
                            }
                        }
                        None
                    };
                    new_state.agents[curr_agent_idx].error_message = error_message;
                }
                Some(Action::Inspect { poi_idx }) => {
                    match state.nodes[curr_node_idx].pois.get(poi_idx) {
                        None => {
                            new_state.agents[curr_agent_idx].error_message =
                                Some(format!("no poi at index {poi_idx}"));
                        }
                        Some(poi) if poi.kind == PoiKind::RuinedWorkbench => {
                            new_state.agents[curr_agent_idx].open_menu = Some(Menu::Workbench);
                        }
                        Some(_) => {
                            new_state.agents[curr_agent_idx].error_message =
                                Some("nothing to inspect here".to_string());
                        }
                    }
                }
                Some(Action::Craft(recipe_idx)) => {
                    if !menu_open {
                        new_state.agents[curr_agent_idx].error_message =
                            Some("you are not at a workbench".to_string());
                    } else if let Some(recipe) = WORKBENCH_RECIPES.get(recipe_idx) {
                        let inventory = &mut new_state.agents[curr_agent_idx].inventory;
                        let affordable = recipe.ingredients.iter().all(|ingredient| {
                            inventory.iter()
                                .find(|item_stack| item_stack.item == ingredient.item)
                                .map(|item_stack| item_stack.count)
                                .unwrap_or(0) >= ingredient.count
                        });
                        if affordable {
                            for ingredient in recipe.ingredients {
                                let item_stack_idx = inventory.iter()
                                    .position(|item_stack| item_stack.item == ingredient.item)
                                    .unwrap();
                                inventory[item_stack_idx].count -= ingredient.count;
                                if inventory[item_stack_idx].count == 0 {
                                    inventory.remove(item_stack_idx);
                                }
                            }
                            inventory.push(recipe.output.clone());
                        } else {
                            new_state.agents[curr_agent_idx].error_message = Some(format!(
                                "not enough items to craft {}",
                                item_label(&recipe.output.item)
                            ));
                        }
                    } else {
                        new_state.agents[curr_agent_idx].error_message =
                            Some(format!("no recipe at index {recipe_idx}"));
                    }
                }
                Some(Action::Exit) => {
                    if menu_open {
                        new_state.agents[curr_agent_idx].open_menu = None;
                    } else {
                        new_state.agents[curr_agent_idx].error_message = Some("nothing to exit".to_string());
                    }
                }
                None => {}
            }

            if let Some(content) = input.send_message {
                // the message reaches whoever shared the node at the start of the turn
                for (agent_idx, agent) in new_state.agents.iter_mut().enumerate() {
                    if state.agents[agent_idx].node_idx != curr_node_idx { continue; }
                    agent.node_messages_inbox.push(Message {
                        sender_agent_idx: curr_agent_idx,
                        content: content.clone(),
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
