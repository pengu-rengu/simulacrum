use crate::state::state::{State, Agent};
use crate::state::state::WORKBENCH_RECIPES;
use crate::spec::common::{item_label, poi_label};

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
        lines.push(format!("[{}] {}", poi_idx, poi_label(poi)));
    }
    lines.push("".to_string());

    lines
}

/// What the workbench offers, shown in place of the node view.
pub fn workbench_lines() -> Vec<String> {
    let mut lines = vec!["Workbench: craft or exit".to_string(), "".to_string()];

    lines.push("Recipes:".to_string());
    for (recipe_idx, recipe) in WORKBENCH_RECIPES.iter().enumerate() {
        let ingredients = recipe.ingredients.iter()
            .map(|ingredient| format!("{} {}", ingredient.count, item_label(&ingredient.item)))
            .collect::<Vec<String>>()
            .join(", ");
        lines.push(format!("[{}] {} <- {}", recipe_idx, item_label(&recipe.output.item), ingredients));
    }
    lines.push("".to_string());

    lines
}
