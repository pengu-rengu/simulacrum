use crate::state::state::{PointOfInterest, State};
use crate::state::agent::Agent;

pub fn check_agent(state: &State, agent: &Agent) {
    let nodeworld_agent = &agent.nodeworld;
    assert!(nodeworld_agent.node_idx < state.nodes.len());
    assert_eq!(nodeworld_agent.body_grid.cells.len(), nodeworld_agent.body_grid.width * nodeworld_agent.body_grid.height);
    for message in &nodeworld_agent.node_messages_inbox {
        assert!(message.sender_agent_idx < state.agents.len());
    }
    for message in &agent.escape_room.messages_inbox {
        assert!(message.sender_agent_idx < state.agents.len());
    }

    // a menu is only open where the thing it belongs to stands
    if let Some(open_menu) = &nodeworld_agent.open_menu {
        assert!(state.nodes[nodeworld_agent.node_idx].pois.iter().any(|poi| {
            matches!(poi, PointOfInterest::Inspectable { menu, .. } if menu == open_menu)
        }));
    }
}
