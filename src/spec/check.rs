use crate::spec::spec::{check_next_state, check_flush_state};
use crate::state::state::{State, Agent, Node, BodyGrid, BodyCell};
use crate::impl_::next_state::{flush_state, next_state};
use std::{
    iter::zip,
    collections::HashSet,
};

fn mock_states() -> Vec<State> {
    let contents = ["hi", "", "line1\nline2", "with \"quotes\""];
    let mut base_states = vec![State::new()];

    for num_agents in 1..=3 {
        for num_nodes in 1..=2 {
            for shift in 0..num_nodes {
                for agent_idx in 0..num_agents {
                    for has_error in [false, true] {
                        let agents = (0..num_agents).map(|a| Agent {
                            name: format!("Agent{a}"),
                            node_idx: (a + shift) % num_nodes,
                            node_messages_inbox: (0..a % 3).map(|m| crate::state::state::Message {
                                sender_agent_idx: (a + m) % num_agents,
                                content: contents[(a + m) % contents.len()].to_string(),
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
                                        _ => BodyCell::EmptyCell,
                                    }).collect(),
                                }
                            },
                        }).collect();
                        base_states.push(State {
                            turn: num_agents + shift,
                            agent_idx,
                            agents,
                            nodes: (0..num_nodes).map(|n| Node {
                                name: format!("Node{n}"),
                                biome: "Amberwood thicket".to_string(),
                            }).collect(),
                        });
                    }
                }
            }
        }
    }

    // check() zips states with inputs, so repeat each state 16 times;
    // mock_input_strs() cycles its (<= 16) inputs, pairing every state with every input
    base_states.into_iter()
        .flat_map(|state| std::iter::repeat_n(state, 16))
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