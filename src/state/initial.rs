use crate::state::state::{Agent, BodyCell, BodyGrid, DepositType, Enemy, Item, ItemStack, Menu, Node, PointOfInterest, Recipe, State};

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
                        PointOfInterest::ResourceDeposit { 
                            name: "Thornbush".to_string(),
                            type_: DepositType::Forage,
                            exposed: 2,
                            stability: 4,
                            reserves: 20,
                            yield_: Item::Stick
                        },
                        PointOfInterest::ResourceDeposit {
                            name: "Amber Bole".to_string(),
                            type_: DepositType::Forage,
                            exposed: 2,
                            stability: 3,
                            reserves: 15,
                            yield_: Item::Resin
                        },
                        PointOfInterest::ResourceDeposit {
                            name: "Smooth Pebble".to_string(),
                            type_: DepositType::Forage,
                            exposed: 4,
                            stability: 6,
                            reserves: 30,
                            yield_: Item::SmoothPebble
                        },
                        PointOfInterest::Inspectable {
                            name: "Ruined Workbench".to_string(),
                            menu: Menu::CraftingMenu { recipes: vec![Recipe {
                                output: ItemStack { item: Item::CrudePickaxe { 
                                    durability: 20
                                }, count: 1 },
                                ingredients: vec![
                                    ItemStack { item: Item::Stick, count: 10 },
                                    ItemStack { item: Item::SmoothPebble, count: 10 },
                                ],
                            },
                            Recipe {
                                output: ItemStack { item: Item::CopperPickaxe {
                                    durability: 40
                                }, count: 1 },
                                ingredients: vec![
                                    ItemStack { item: Item::CopperOre, count: 5 },
                                    ItemStack { item: Item::Stick, count: 5 }
                                ]
                            },
                            Recipe {
                                output: ItemStack { item: Item::CopperDrill {
                                    durability: 20
                                }, count: 1 },
                                ingredients: vec![
                                    ItemStack { item: Item::CopperOre, count: 8 },
                                    ItemStack { item: Item::Resin, count: 3 },
                                    ItemStack { item: Item::Stick, count: 5 }
                                ]
                            }]}
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
                            yield_: Item::CopperOre
                        },
                        PointOfInterest::EnemyGroup {
                            name: "Husk Pack".to_string(),
                            enemies: vec![amber_husk(), amber_husk()]
                        }
                    ],
                    combat_encounters: vec![]
                }
            ]
        }
    }
}