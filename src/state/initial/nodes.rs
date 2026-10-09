use crate::state::nodeworld::{BodyCell, BodyGrid, DepositType, Enemy, Item, ItemStack, Menu, Node, PointOfInterest, Recipe, ToolAttribute};

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

pub fn initial_nodes() -> Vec<Node> {
    vec![
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
        ]
}