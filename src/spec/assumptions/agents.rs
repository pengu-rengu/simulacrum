use crate::state::state::{Agent, PointOfInterest, State};

pub fn check_agent(state: &State, agent: &Agent) {
    assert!(agent.node_idx < state.nodes.len());
    assert_eq!(agent.body_grid.cells.len(), agent.body_grid.width * agent.body_grid.height);
    for message in &agent.node_messages_inbox {
        assert!(message.sender_agent_idx < state.agents.len());
    }

    // a menu is only open where the thing it belongs to stands
    if let Some(open_menu) = &agent.open_menu {
        assert!(state.nodes[agent.node_idx].pois.iter().any(|poi| {
            matches!(poi, PointOfInterest::Inspectable { menu, .. } if menu == open_menu)
        }));
    }
}
