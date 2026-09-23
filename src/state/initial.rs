use crate::state::state::{State, Agent, Node, BodyGrid, BodyCell, PointOfInterest};

fn initial_body_grid() -> BodyGrid {
    BodyGrid {
        width: 1,
        height: 1,
        cells: vec![BodyCell::CoreCell { health: 10 }],
    }
}

impl State {
    pub fn new() -> State {
        State { 
            turn: 0,
            agent_idx: 0,
            agents: vec![
                Agent {
                    name: "Agent1".to_string(),
                    node_idx: 0,
                    node_messages_inbox: vec![],
                    error_message: None,
                    body_grid: initial_body_grid(),
                    inventory: vec![],
                    open_menu: None,
                },
                Agent {
                    name: "Agent2".to_string(),
                    node_idx: 0,
                    node_messages_inbox: vec![],
                    error_message: None,
                    body_grid: initial_body_grid(),
                    inventory: vec![],
                    open_menu: None
                },
            ],
            nodes: vec![
                Node {
                    name: "Sapwell Hollow".to_string(),
                    biome: "Amberwood thicket".to_string(),
                    pois: vec![
                        PointOfInterest::Thornbush,
                        PointOfInterest::AmberBole,
                        PointOfInterest::SmoothPebble,
                        PointOfInterest::RuinedWorkbench,
                    ],
                },
                Node {
                    name: "Amberveins".to_string(),
                    biome: "Amberwood thicket".to_string(),
                    pois: vec![
                        PointOfInterest::CopperOreVein,
                        PointOfInterest::Thornbush,
                        PointOfInterest::SmoothPebble,
                    ],
                },
            ],
        }
    }
}