use std::iter::zip;

use crate::state::state::{CombatEncounter, PointOfInterest, State};
use crate::spec::common::{action_blocked, check_encounters_unchanged, check_error_and_unchanged};

pub fn check_engage(old_state: &State, new_state: &State, poi_idx: usize) {
    let acting_agent_idx = old_state.agent_idx;
    let acting_agent = &old_state.agents[acting_agent_idx];
    let node_idx = acting_agent.nodeworld.node_idx;

    let fail = |expected: &str| {
        check_error_and_unchanged(old_state, new_state, expected);
        check_encounters_unchanged(old_state, new_state);
    };

    if let Some(error) = action_blocked(old_state) {
        fail(error);
        return;
    }

    let Some(poi) = old_state.nodes[node_idx].pois.get(poi_idx) else {
        fail(&format!("no poi at index {poi_idx}"));
        return;
    };

    let PointOfInterest::EnemyGroup { name: _, enemies } = poi else {
        fail("nothing to engage here");
        return;
    };

    let mut expected_encounters = old_state.nodes[node_idx].combat_encounters.clone();
    let existing_encounter = expected_encounters.iter_mut().find(|encounter| {
        matches!(encounter, CombatEncounter::Pve { enemy_group_poi_idx, .. } if *enemy_group_poi_idx == poi_idx)
    });
    match existing_encounter {
        Some(CombatEncounter::Pve { agent_idxs, .. }) => agent_idxs.push(acting_agent_idx),
        None => expected_encounters.push(CombatEncounter::Pve {
            enemy_group_poi_idx: poi_idx,
            agent_idxs: vec![acting_agent_idx],
            enemies: enemies.clone()
        })
    }

    for (i, (old_node, new_node)) in zip(&old_state.nodes, &new_state.nodes).enumerate() {
        if i == node_idx {
            assert_eq!(new_node.combat_encounters, expected_encounters);
        } else {
            assert_eq!(new_node.combat_encounters, old_node.combat_encounters);
        }
    }

    for (i, (old_agent, new_agent)) in zip(&old_state.agents, &new_state.agents).enumerate() {
        if i == acting_agent_idx {
            assert_eq!(new_agent.nodeworld.error_message, None);
        } else {
            assert_eq!(new_agent.nodeworld.error_message, old_agent.nodeworld.error_message);
        }
        assert_eq!(new_agent.nodeworld.inventory, old_agent.nodeworld.inventory);
        assert_eq!(new_agent.nodeworld.open_menu, old_agent.nodeworld.open_menu);
        assert_eq!(new_agent.nodeworld.node_idx, old_agent.nodeworld.node_idx);
    }
}
