use crate::state::state::State;
use crate::state::nodeworld::{CombatEncounter, PointOfInterest};
use std::collections::HashSet;

pub fn check_combat(state: &State) {
    let mut fighting_agent_idxs = HashSet::<usize>::new();
    for (node_idx, node) in state.nodes.iter().enumerate() {
        for poi in &node.pois {
            if let PointOfInterest::EnemyGroup { enemies, .. } = poi {
                for enemy in enemies {
                    assert_eq!(enemy.body_grid.cells.len(), enemy.body_grid.width * enemy.body_grid.height);
                }
            }
        }

        let mut engaged_poi_idxs = HashSet::<usize>::new();
        for CombatEncounter::Pve { enemy_group_poi_idx, agent_idxs, enemies: _ } in &node.combat_encounters {
            assert!(matches!(node.pois.get(*enemy_group_poi_idx), Some(PointOfInterest::EnemyGroup { .. })));
            assert!(engaged_poi_idxs.insert(*enemy_group_poi_idx));

            assert!(!agent_idxs.is_empty());
            for agent_idx in agent_idxs {
                assert!(*agent_idx < state.agents.len());
                assert_eq!(state.agents[*agent_idx].nodeworld.node_idx, node_idx);
                assert!(fighting_agent_idxs.insert(*agent_idx));
            }
        }
    }
}
