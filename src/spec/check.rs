use crate::spec::spec::{check_next_state, check_flush_state};
use crate::state::state::{State, Agent, Node, BodyGrid, BodyCell, Item, ItemStack, PointOfInterest, DepositType, Menu, Recipe};
use crate::impl_::next_state::{flush_state, next_state};
use crate::spec::common::tool_durability;
use std::{
    iter::zip,
    collections::HashSet
};

fn mock_states() -> Vec<State> {
    let contents = ["hi", "", "line1\nline2", "with \"quotes\""];
    let mut base_states = vec![State::new()];

    // (exposed, stability, reserves): nothing exposed, collapses on the first swing,
    // runs out of exposure before stability, collapses mid-harvest, the real copper vein,
    // only reachable by drilling (and drilled dry), and nothing left to drill
    let deposit_kinds = [(0, 3, 8), (2, 1, 8), (2, 4, 8), (4, 2, 8), (2, 10, 8), (0, 16, 3), (1, 12, 0)];
    let workbench_menu = Menu::CraftingMenu {
        recipes: vec![
            Recipe {
                output: ItemStack { item: Item::CrudePickaxe { durability: 20 }, count: 1 },
                ingredients: vec![
                    ItemStack { item: Item::Stick, count: 10 },
                    ItemStack { item: Item::SmoothPebble, count: 10 }
                ]
            },
            // a non-pickaxe output, which joins an existing stack
            Recipe {
                output: ItemStack { item: Item::Resin, count: 1 },
                ingredients: vec![ItemStack { item: Item::Stick, count: 2 }]
            },
            Recipe {
                output: ItemStack { item: Item::CopperPickaxe { durability: 40 }, count: 1 },
                ingredients: vec![
                    ItemStack { item: Item::CopperOre, count: 5 },
                    ItemStack { item: Item::Stick, count: 5 }
                ]
            },
            Recipe {
                output: ItemStack { item: Item::CopperDrill { durability: 20 }, count: 1 },
                ingredients: vec![
                    ItemStack { item: Item::CopperOre, count: 8 },
                    ItemStack { item: Item::Resin, count: 3 },
                    ItemStack { item: Item::Stick, count: 5 }
                ]
            }
        ]
    };
    let deposit = |name: &str, type_: DepositType, exposed: usize, stability: usize, reserves: usize, yield_: Item| {
        PointOfInterest::ResourceDeposit { name: name.to_string(), type_, exposed, stability, reserves, yield_ }
    };

    for num_agents in 1..=3 {
        for num_nodes in 1..=2 {
            for shift in 0..num_nodes {
                for agent_idx in 0..num_agents {
                    // 0: nothing, 1: enough to craft, 2: holds a pickaxe, 3: partial materials,
                    // 4: enough to craft while already holding identical pickaxes and resin,
                    // 5: copper tools, 6: enough to craft both copper tools, 7: one tool of each tier
                    for inventory_kind in 0..8 {
                    // the acting agent is either in the node view or standing at the workbench menu
                    for (exposed, stability, reserves) in deposit_kinds {
                    for open_menu in [None, Some(workbench_menu.clone())] {
                    for has_error in [false, true] {
                        let agents = (0..num_agents).map(|a| Agent {
                            name: format!("Agent{a}"),
                            node_idx: (a + shift) % num_nodes,
                            node_messages_inbox: (0..a % 3).map(|m| crate::state::state::Message {
                                sender_agent_idx: (a + m) % num_agents,
                                content: contents[(a + m) % contents.len()].to_string()
                            }).collect(),
                            error_message: if has_error && a == agent_idx {
                                Some("could not parse input json".to_string())
                            } else {
                                None
                            },
                            body_grid: {
                                let width = 1 + a % 3;
                                let height = 1 + (a + shift) % 2;
                                BodyGrid {
                                    width,
                                    height,
                                    cells: (0..width * height).map(|cell_idx| match (a + cell_idx) % 3 {
                                        0 => BodyCell::CoreCell { health: 5 + a },
                                        1 => BodyCell::CoreCell { health: 0 },
                                        _ => BodyCell::EmptyCell
                                    }).collect()
                                }
                            },
                            inventory: match inventory_kind {
                                0 => vec![],
                                1 => vec![
                                    ItemStack { item: Item::Stick, count: 10 + a },
                                    ItemStack { item: Item::SmoothPebble, count: 10 }
                                ],
                                // two pickaxes: picking the right tool_idx is the point of indexing
                                2 => vec![
                                    ItemStack { item: Item::CrudePickaxe { durability: 1 }, count: 1 },
                                    ItemStack { item: Item::Resin, count: 3 },
                                    ItemStack { item: Item::CrudePickaxe { durability: 20 }, count: 1 }
                                ],
                                3 => vec![
                                    ItemStack { item: Item::Stick, count: 10 },
                                    ItemStack { item: Item::SmoothPebble, count: 9 }
                                ],
                                4 => vec![
                                    ItemStack { item: Item::Stick, count: 10 },
                                    ItemStack { item: Item::SmoothPebble, count: 10 },
                                    ItemStack { item: Item::Resin, count: 3 },
                                    ItemStack { item: Item::CrudePickaxe { durability: 20 }, count: 1 },
                                    ItemStack { item: Item::CrudePickaxe { durability: 20 }, count: 1 }
                                ],
                                // same layout as 2 at indices 0-2, with a fresh and a worn drill after
                                5 => vec![
                                    ItemStack { item: Item::CopperPickaxe { durability: 1 }, count: 1 },
                                    ItemStack { item: Item::Resin, count: 3 },
                                    ItemStack { item: Item::CopperPickaxe { durability: 40 }, count: 1 },
                                    ItemStack { item: Item::CopperDrill { durability: 20 }, count: 1 },
                                    ItemStack { item: Item::CopperDrill { durability: 1 }, count: 1 }
                                ],
                                6 => vec![
                                    ItemStack { item: Item::Stick, count: 10 },
                                    ItemStack { item: Item::CopperOre, count: 13 },
                                    ItemStack { item: Item::Resin, count: 3 }
                                ],
                                _ => vec![
                                    ItemStack { item: Item::CrudePickaxe { durability: 20 }, count: 1 },
                                    ItemStack { item: Item::CopperPickaxe { durability: 40 }, count: 1 },
                                    ItemStack { item: Item::CopperDrill { durability: 20 }, count: 1 }
                                ]
                            },
                            // only the acting agent opens a menu, and only where a workbench stands
                            open_menu: if a == agent_idx && (a + shift) % num_nodes == 0 {
                                open_menu.clone()
                            } else {
                                None
                            }
                        }).collect();
                        base_states.push(State {
                            turn: num_agents + shift,
                            agent_idx,
                            agents,
                            // node 0 holds every poi kind, node 1 holds none
                            nodes: (0..num_nodes).map(|n| Node {
                                name: format!("Node{n}"),
                                biome: "Amberwood thicket".to_string(),
                                pois: if n == 0 {
                                    vec![
                                        deposit("Thornbush", DepositType::Forage, exposed, stability, reserves, Item::Stick),
                                        deposit("Amber Bole", DepositType::Forage, exposed, stability, reserves, Item::Resin),
                                        deposit("Smooth Pebble", DepositType::Forage, exposed, stability, reserves, Item::SmoothPebble),
                                        deposit("Copper Ore Vein", DepositType::Rock, exposed, stability, reserves, Item::CopperOre),
                                        PointOfInterest::Inspectable {
                                            name: "Ruined Workbench".to_string(),
                                            menu: workbench_menu.clone()
                                        }
                                    ]
                                } else {
                                    vec![]
                                }
                            }).collect()
                        });
                    }
                    }
                    }
                    }
                }
            }
        }
    }

    // check() zips states with inputs, so repeat each state 64 times;
    // mock_input_strs() cycles its 64 inputs, pairing every state with every input
    base_states.into_iter()
        .flat_map(|state| std::iter::repeat_n(state, 64))
        .collect()
}

fn mock_input_strs() -> Vec<String> {
    let base_inputs = vec![
        "".to_string(),
        "not json".to_string(),
        "[]".to_string(),
        serde_json::json!({}).to_string(),
        serde_json::json!({ "send_message": null }).to_string(),
        serde_json::json!({ "send_message": "hello" }).to_string(),
        serde_json::json!({ "send_message": "" }).to_string(),
        serde_json::json!({ "send_message": "multi\nline" }).to_string(),
        serde_json::json!({ "send_message": "with \"quotes\"" }).to_string(),
        serde_json::json!({ "action": null }).to_string(),
        serde_json::json!({ "action": { "move_to": "Node0" } }).to_string(),
        serde_json::json!({ "action": { "move_to": "Node1" } }).to_string(),
        serde_json::json!({ "action": { "move_to": "Nowhere" } }).to_string(),
        serde_json::json!({ "action": { "move_to": "" } }).to_string(),
        // bare-handed harvests: one swing per null
        serde_json::json!({ "action": { "harvest": { "poi_idx": 0, "tool_idxs": [] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 0, "tool_idxs": [null] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 1, "tool_idxs": [null, null] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 2, "tool_idxs": [null, null, null, null] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 0, "tool_idxs": [null, null, null, null, null, null] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [null] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 4, "tool_idxs": [null] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 99, "tool_idxs": [null] } } }).to_string(),
        // with tools: index 0 is the worn pickaxe, 2 the fresh one, 1 is not a tool at all
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [0] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [0, 0] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [0, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [2, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [2, 2, 2, 2, 2, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [2, 0, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [] } } }).to_string(),
        // identical pickaxes in separate stacks
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [3, 4] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 1, "tool_idxs": [2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [1] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [99] } } }).to_string(),
        // a bad tool after a good swing, unless the deposit collapsed first
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [2, 99] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 0, "tool_idxs": [null, 2] } } }).to_string(),
        // drilling: index 3 is a fresh drill and 4 a worn one with copper tools,
        // while with one tool of each tier 0 is crude, 1 copper, 2 the drill
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [3] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [3, 3, 2, 2, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [3, 3, 3, 3, 2, 2, 2, 2, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [3, 2, 3, 2, 3, 2, 3, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [4, 4] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [4, 2, 2] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [3, 3, 3, 3, 3, 3, 3, 3, 3, 3] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 1, "tool_idxs": [3] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [0, 1] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [1, 0] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [2, 2, 1, 1, 1, 0] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idxs": [2, 0, 2, 0] } } }).to_string(),
        // inspect
        serde_json::json!({ "action": { "inspect": { "poi_idx": 4 } } }).to_string(),
        serde_json::json!({ "action": { "inspect": { "poi_idx": 0 } } }).to_string(),
        serde_json::json!({ "action": { "inspect": { "poi_idx": 99 } } }).to_string(),
        // malformed actions
        serde_json::json!({ "action": { "fly": 1 } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 0 } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": "zero", "tool_idxs": [null] } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 3, "tool_idx": 2, "uses": 1 } } }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 0, "tool_idxs": [-1] } } }).to_string(),
        // menu
        serde_json::json!({ "action": { "craft": 0 } }).to_string(),
        serde_json::json!({ "action": { "craft": 1 } }).to_string(),
        serde_json::json!({ "action": { "craft": 2 } }).to_string(),
        serde_json::json!({ "action": { "craft": 3 } }).to_string(),
        serde_json::json!({ "action": { "craft": 9 } }).to_string(),
        serde_json::json!({ "action": "exit" }).to_string(),
        // action plus message
        serde_json::json!({ "action": { "move_to": "Node1" }, "send_message": "heading out" }).to_string(),
        serde_json::json!({ "action": { "harvest": { "poi_idx": 0, "tool_idxs": [null] } }, "send_message": "grabbing a stick" }).to_string(),
        serde_json::json!({ "action": { "craft": 0 }, "send_message": "making a pickaxe" }).to_string()
    ];
    (0..mock_states().len())
        .map(|i| base_inputs[i % base_inputs.len()].clone())
        .collect()
}

fn assumptions(state: &State) {
    assert!(state.agents.len() > 0);
    assert!(state.agent_idx < state.agents.len());
    assert!(state.nodes.len() > 0);

    for agent in &state.agents {
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

        for item_stack in &agent.inventory {
            if let Some(durability) = tool_durability(&item_stack.item) {
                assert!(durability > 0);
                assert!(item_stack.count == 1);
            }
        }

        let items = agent.inventory.iter().map(|item_stack| &item_stack.item).collect::<Vec<&Item>>();
        // tools stay unstacked; every other item has one stack
        for item_stack in &agent.inventory {
            assert!(item_stack.count > 0);
            if tool_durability(&item_stack.item).is_none() {
                assert_eq!(items.iter().filter(|item| ***item == item_stack.item).count(), 1);
            }
        }
    }
    
    let agent_names  = state.agents.iter().map(|agent| &agent.name).collect::<HashSet<&String>>();
    assert_eq!(agent_names.len(), state.agents.len());

    let node_names = state.nodes.iter().map(|node| &node.name).collect::<HashSet<&String>>();
    assert_eq!(node_names.len(), state.nodes.len());
}

pub fn check() {
    for (state, input_str) in zip(mock_states(), mock_input_strs()) {
        assumptions(&state);

        let (flushed_state, output) = flush_state(&state);
        check_flush_state(&state, &flushed_state, &output);

        let new_state = next_state(&state, &input_str);
        check_next_state(&state, &new_state, &input_str);
    }
}