use crate::state::state::{State, Agent, Node, BodyGrid, BodyCell, PointOfInterest, PoiKind, Deposit};

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
                        PointOfInterest {
                            kind: PoiKind::Thornbush,
                            deposit: Some(Deposit { exposure: 3, stability: 4, reserves: 20 }),
                        },
                        PointOfInterest {
                            kind: PoiKind::AmberBole,
                            deposit: Some(Deposit { exposure: 2, stability: 3, reserves: 12 }),
                        },
                        PointOfInterest {
                            kind: PoiKind::SmoothPebble,
                            deposit: Some(Deposit { exposure: 4, stability: 6, reserves: 30 }),
                        },
                        PointOfInterest { kind: PoiKind::RuinedWorkbench, deposit: None },
                    ],
                },
                Node {
                    name: "Amberveins".to_string(),
                    biome: "Amberwood thicket".to_string(),
                    pois: vec![
                        PointOfInterest {
                            kind: PoiKind::CopperOreVein,
                            deposit: Some(Deposit { exposure: 2, stability: 5, reserves: 8 }),
                        },
                        PointOfInterest {
                            kind: PoiKind::Thornbush,
                            deposit: Some(Deposit { exposure: 1, stability: 2, reserves: 6 }),
                        },
                        PointOfInterest {
                            kind: PoiKind::SmoothPebble,
                            deposit: Some(Deposit { exposure: 4, stability: 6, reserves: 30 }),
                        },
                    ],
                },
            ],
        }
    }
}