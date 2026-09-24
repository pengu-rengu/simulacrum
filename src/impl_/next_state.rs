use crate::state::state::{State, Input, Action, Message, Item, ItemStack, PointOfInterest, Menu, workbench_recipes};
use serde_json::from_str;

pub fn flush_state(state: &State) -> (State, String) {
    let mut new_state = state.clone();
    let agent_idx = new_state.agent_idx;
    let agent = &state.agents[agent_idx];
    let node = &state.nodes[agent.node_idx];

    let mut lines = vec![];

    match agent.open_menu {
        Some(Menu::Workbench) => {
            lines.push("Workbench: craft or exit".to_string());
            lines.push("".to_string());

            lines.push("Recipes:".to_string());
            for (recipe_idx, recipe) in workbench_recipes().iter().enumerate() {
                let ingredients = recipe.ingredients.iter()
                    .map(|ingredient| format!("{} {:?}", ingredient.count, ingredient.item))
                    .collect::<Vec<String>>()
                    .join(", ");
                lines.push(format!("[{}] {:?} <- {}", recipe_idx, recipe.output.item, ingredients));
            }
            lines.push("".to_string());
        }
        None => {
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
                lines.push(format!("[{}] {:?}", poi_idx, poi));
            }
            lines.push("".to_string());
        }
    }

    lines.push("Inventory:".to_string());
    for item_stack in &agent.inventory {
        lines.push(format!("{} {:?}", item_stack.count, item_stack.item));
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

    // every item the action gives or takes from the acting agent, applied once at the end
    let mut inventory_deltas: Vec<(Item, i64)> = vec![];

    match from_str::<Input>(input_str) {
        Err(_) => {
            new_state.agents[curr_agent_idx].error_message = Some("could not parse input json".to_string());
        }
        Ok(input) => {
            new_state.agents[curr_agent_idx].error_message = None;

            match input.action {
                Some(Action::MoveTo(_)) | Some(Action::Interact(_)) if menu_open => {
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
                Some(Action::Interact(poi_idx)) => {
                    match state.nodes[curr_node_idx].pois.get(poi_idx) {
                        None => {
                            new_state.agents[curr_agent_idx].error_message =
                                Some(format!("no poi at index {poi_idx}"));
                        }
                        Some(PointOfInterest::Thornbush) => inventory_deltas.push((Item::Stick, 1)),
                        Some(PointOfInterest::AmberBole) => inventory_deltas.push((Item::Resin, 1)),
                        Some(PointOfInterest::SmoothPebble) => inventory_deltas.push((Item::SmoothPebble, 1)),
                        Some(PointOfInterest::CopperOreVein) => {
                            let has_pickaxe = state.agents[curr_agent_idx].inventory.iter()
                                .any(|item_stack| item_stack.item == Item::CrudePickaxe && item_stack.count > 0);
                            if has_pickaxe {
                                inventory_deltas.push((Item::CopperOre, 1));
                            } else {
                                new_state.agents[curr_agent_idx].error_message = Some("need a crude pickaxe to mine copper ore".to_string());
                            }
                        }
                        Some(PointOfInterest::RuinedWorkbench) => {
                            new_state.agents[curr_agent_idx].open_menu = Some(Menu::Workbench);
                        }
                    }
                }
                Some(Action::Craft(recipe_idx)) => {
                    let recipes = workbench_recipes();
                    if !menu_open {
                        new_state.agents[curr_agent_idx].error_message =
                            Some("you are not at a workbench".to_string());
                    } else if let Some(recipe) = recipes.get(recipe_idx) {
                        let affordable = recipe.ingredients.iter().all(|ingredient| {
                            let held = state.agents[curr_agent_idx].inventory.iter()
                                .find(|item_stack| item_stack.item == ingredient.item)
                                .map(|item_stack| item_stack.count)
                                .unwrap_or(0);
                            held >= ingredient.count
                        });
                        if affordable {
                            for ingredient in &recipe.ingredients {
                                inventory_deltas.push((ingredient.item.clone(), -(ingredient.count as i64)));
                            }
                            inventory_deltas.push((recipe.output.item.clone(), recipe.output.count as i64));
                        } else {
                            new_state.agents[curr_agent_idx].error_message =
                                Some(format!("not enough items to craft {:?}", recipe.output.item));
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

    let inventory = &mut new_state.agents[curr_agent_idx].inventory;
    for (item, delta) in inventory_deltas {
        match inventory.iter_mut().find(|item_stack| item_stack.item == item) {
            Some(item_stack) if delta < 0 => item_stack.count -= (-delta) as usize,
            Some(item_stack) => item_stack.count += delta as usize,
            None if delta > 0 => inventory.push(ItemStack { item, count: delta as usize }),
            None => {}
        }
    }
    inventory.retain(|item_stack| item_stack.count > 0);

    new_state.agent_idx += 1;
    if new_state.agent_idx == new_state.agents.len() {
        new_state.agent_idx = 0;
        new_state.turn += 1;
    }
    new_state
}
