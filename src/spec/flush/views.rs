use crate::state::state::{State, Agent, PointOfInterest, DepositType, Recipe};
use crate::spec::common::item_label;

/// Where the agent stands, who else is there, and what it can work on.
pub fn node_lines(state: &State, agent: &Agent) -> Vec<String> {
    let node = &state.nodes[agent.node_idx];
    let mut lines = vec![format!("Node: {} ({})", node.name, node.biome), "".to_string()];

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
            PointOfInterest::Inspectable { name, .. } => name.clone()
        };
        lines.push(format!("[{}] {}", poi_idx, label));
    }
    lines.push("".to_string());

    lines
}

/// What the open crafting menu offers, shown in place of the node view.
pub fn crafting_menu_lines(recipes: &Vec<Recipe>) -> Vec<String> {
    let mut lines = vec!["Crafting menu: craft or exit".to_string(), "".to_string()];

    lines.push("Recipes:".to_string());
    for (recipe_idx, recipe) in recipes.iter().enumerate() {
        let ingredients = recipe.ingredients.iter()
            .map(|ingredient| format!("{} {}", ingredient.count, item_label(&ingredient.item)))
            .collect::<Vec<String>>()
            .join(", ");
        lines.push(format!("[{}] {} <- {}", recipe_idx, item_label(&recipe.output.item), ingredients));
    }
    lines.push("".to_string());

    lines
}
