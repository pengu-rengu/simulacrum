use crate::state::state::{ToolAttribute, BodyCell, BodyGrid, DepositType, Enemy, Item, ItemStack, Menu, Node, PointOfInterest, Recipe, State};
use crate::state::escaperoom::{DoorCell, EscapeRoomCell, Room};
use crate::state::agent::{Agent, EscapeRoomAgent, NodeworldAgent};

fn initial_body_grid() -> BodyGrid {
    BodyGrid {
        width: 1,
        height: 1,
        cells: vec![BodyCell::CoreCell { health: 10 }],
    }
}

/// Core cells in a plus shape.
fn amber_husk() -> Enemy {
    let core = || BodyCell::CoreCell { health: 5 };
    Enemy {
        name: "Amber Husk".to_string(),
        body_grid: BodyGrid {
            width: 3,
            height: 3,
            cells: vec![
                BodyCell::EmptyCell, core(), BodyCell::EmptyCell,
                core(), core(), core(),
                BodyCell::EmptyCell, core(), BodyCell::EmptyCell
            ]
        }
    }
}

fn room_from_str(name: &str, s: &str) -> Room {
    let mut cells = vec![];
    let mut width = 0;
    let mut height = 0;
    for line in s.lines().skip(1) {
        width = line.len();
        height += 1;
        for char in line.chars() {
            cells.push(match char {
                ' ' => EscapeRoomCell::Empty,
                'x' => EscapeRoomCell::Wall,
                'A' | 'B' | 'C' | 'D' |
                'a' | 'b' | 'c' | 'd' => EscapeRoomCell::Door(DoorCell {
                    id: char.to_uppercase().to_string(),
                    open: char.is_lowercase()
                }),
                _ => panic!("Invalid character in string: {}", char)
            });
        }
    }
    Room {
        name: name.to_string(),
        width: width,
        height: height,
        cells: cells
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
                    nodeworld: NodeworldAgent {
                        node_idx: 0,
                        node_messages_inbox: vec![],
                        error_message: None,
                        body_grid: initial_body_grid(),
                        inventory: vec![],
                        open_menu: None
                    },
                    escape_room: EscapeRoomAgent {
                        room_idx: 0,
                        x: 0,
                        y: 0,
                        messages_inbox: vec![]
                    }
                },
                Agent {
                    name: "Agent2".to_string(),
                    nodeworld: NodeworldAgent {
                        node_idx: 0,
                        node_messages_inbox: vec![],
                        error_message: None,
                        body_grid: initial_body_grid(),
                        inventory: vec![],
                        open_menu: None
                    },
                    escape_room: EscapeRoomAgent {
                        room_idx: 0,
                        x: 0,
                        y: 0,
                        messages_inbox: vec![]
                    }
                },
            ],
            nodes: vec![
                Node {
                    name: "Sapwell Hollow".to_string(),
                    biome: "Amberwood thicket".to_string(),
                    pois: vec![
                        PointOfInterest::ResourceDeposit { 
                            name: "Thornbush".to_string(),
                            type_: DepositType::Forage,
                            exposed: 2,
                            stability: 4,
                            reserves: 20,
                            yield_: Item::Material { name: "Stick".to_string() }
                        },
                        PointOfInterest::ResourceDeposit {
                            name: "Amber Bole".to_string(),
                            type_: DepositType::Forage,
                            exposed: 2,
                            stability: 3,
                            reserves: 15,
                            yield_: Item::Material { name: "Resin".to_string() }
                        },
                        PointOfInterest::ResourceDeposit {
                            name: "Smooth Pebble".to_string(),
                            type_: DepositType::Forage,
                            exposed: 4,
                            stability: 6,
                            reserves: 30,
                            yield_: Item::Material { name: "Smooth Pebble".to_string()}
                        },
                        PointOfInterest::Inspectable {
                            name: "Ruined Workbench".to_string(),
                            menu: Menu::CraftingMenu { recipes: vec![
                                Recipe {
                                    output: ItemStack { 
                                        item: Item::Tool { 
                                            name: "Crude Pickaxe".to_string(),
                                            deposit_type: DepositType::Rock,
                                            attributes: vec![(ToolAttribute::Chipping, 1)],
                                            durability: 20
                                        }, 
                                        count: 1 
                                    },
                                    ingredients: vec![
                                        ("Stick".to_string(), 10),
                                        ("Smooth Pebble".to_string(), 10),
                                    ],
                                },
                                Recipe {
                                    output: ItemStack {
                                        item: Item::Tool {
                                            name: "Copper Pickaxe".to_string(),
                                            deposit_type: DepositType::Rock,
                                            attributes: vec![(ToolAttribute::Chipping, 1)],
                                            durability: 40
                                        }, 
                                        count: 1
                                    },
                                    ingredients: vec![
                                        ("Copper Ore".to_string(), 10),
                                        ("Stick".to_string(), 10)
                                    ]
                                },
                                Recipe {
                                    output: ItemStack {
                                        item: Item::Tool {
                                            name: "Copper Drill".to_string(),
                                            deposit_type: DepositType::Rock,
                                            attributes: vec![(ToolAttribute::Drilling, 1)],
                                            durability: 20
                                        },
                                        count: 1
                                    },
                                    ingredients: vec![
                                        ("Copper Ore".to_string(), 8),
                                        ("Resin".to_string(), 3),
                                        ("Stick".to_string(), 5)
                                    ]
                                }
                            ]}
                        },
                    ],
                    combat_encounters: vec![]
                },
                Node {
                    name: "Amberveins".to_string(),
                    biome: "Amberwood thicket".to_string(),
                    pois: vec![
                        PointOfInterest::ResourceDeposit {
                            name: "Copper Ore Vein".to_string(),
                            type_: DepositType::Rock,
                            exposed: 2,
                            stability: 10,
                            reserves: 8,
                            yield_: Item::Material { name: "Copper Ore".to_string() }
                        },
                        PointOfInterest::EnemyGroup {
                            name: "Husk Pack".to_string(),
                            enemies: vec![amber_husk(), amber_husk()]
                        }
                    ],
                    combat_encounters: vec![]
                }
            ],
            rooms: vec![
                room_from_str("Room A", "
                    xxxxx
                    x   x
                    x   x
                    x   x
                    xxxxx
                ")
            ]
        }
    }
}