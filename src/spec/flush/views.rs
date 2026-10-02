use crate::state::state::{State, Agent, PointOfInterest, Recipe, CombatEncounter};
use crate::spec::common::{deposit_type_label, item_label};

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
                format!("{} ({}, exposed {}, stability {}, reserves {})", name, deposit_type_label(type_), exposed, stability, reserves)
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
            PointOfInterest::EnemyGroup { name, .. } => name,
            _ => panic!("encounter is not against an enemy group. this shouldn't be reachable")
        };
        let agent_names = agent_idxs.iter()
            .map(|agent_idx| state.agents[*agent_idx].name.clone())
            .collect::<Vec<String>>().join(", ");
        lines.push(format!("[{enemy_group_poi_idx}] {group_name}: {agent_names}"));
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
            .map(|(name, count)| format!("{count} {name}"))
            .collect::<Vec<String>>().join(", ");
        lines.push(format!("[{}] {} <- {}", recipe_idx, item_label(&recipe.output.item), ingredients));
    }
    lines.push("".to_string());

    lines
}
