use crate::spec::spec::{assumptions, check_next_state, check_flush_state};
use crate::state::state::{BodyCell, BodyGrid, Message, Node, State, Universe};
use crate::impl_::next_state::{flush_state, next_state};
use crate::state::agent::{Agent, EscapeRoomAgent, NodeworldAgent};
use crate::state::escaperoom::{DoorCell, EscapeRoomCell, Room};
use std::iter::zip;

fn mock_states() -> Vec<State> {
    let body_grid = BodyGrid {
        width: 1,
        height: 1,
        cells: vec![BodyCell::CoreCell { health: 10 }]
    };
    let node = Node {
        name: "Node0".to_string(),
        biome: "Amberwood thicket".to_string(),
        pois: vec![],
        combat_encounters: vec![]
    };
    // closed A at the top edge, open A below it, so one interact toggles both
    let door = |open: bool| EscapeRoomCell::Door(DoorCell { id: "A".to_string(), open });
    let room = Room {
        name: "Cell".to_string(),
        width: 4,
        height: 3,
        cells: vec![
            door(false), EscapeRoomCell::Empty, EscapeRoomCell::Empty, EscapeRoomCell::Wall,
            EscapeRoomCell::Wall, door(true), EscapeRoomCell::Empty, EscapeRoomCell::Wall,
            EscapeRoomCell::Wall, EscapeRoomCell::Wall, EscapeRoomCell::Wall, EscapeRoomCell::Wall
        ]
    };
    let prior_message = Message {
        sender_agent_idx: 0,
        content: "already here".to_string()
    };
    let agent = |name: &str, x: usize, y: usize, error_message: Option<String>, inbox: Vec<Message>| Agent {
        name: name.to_string(),
        error_message,
        universe: Universe::EscapeRoom,
        nodeworld: NodeworldAgent {
            node_idx: 0,
            node_messages_inbox: vec![],
            body_grid: body_grid.clone(),
            inventory: vec![],
            open_menu: None
        },
        escape_room: EscapeRoomAgent {
            room_idx: 0,
            x,
            y,
            messages_inbox: inbox
        }
    };
    let state = |agent_idx: usize, agents: Vec<Agent>| State {
        turn: 0,
        agent_idx,
        agents,
        nodes: vec![node.clone()],
        rooms: vec![room.clone()]
    };

    // check() zips states with inputs, so repeat each state once per input
    let input_count = 15;
    let base_states = vec![
        state(0, vec![
            agent("Agent0", 1, 0, None, vec![]),
            agent("Agent1", 2, 1, None, vec![])
        ]),
        state(0, vec![
            agent("Agent0", 1, 1, None, vec![]),
            agent("Agent1", 2, 1, Some("kept".to_string()), vec![prior_message.clone()])
        ]),
        state(0, vec![
            agent("Agent0", 0, 0, None, vec![]),
            agent("Agent1", 2, 1, None, vec![])
        ]),
        state(1, vec![
            agent("Agent0", 1, 0, None, vec![]),
            agent("Agent1", 2, 1, Some("prior".to_string()), vec![prior_message])
        ])
    ];
    base_states.into_iter()
        .flat_map(|state| std::iter::repeat_n(state, input_count))
        .collect()
}

fn mock_input_strs() -> Vec<String> {
    let base_inputs = vec![
        serde_json::json!({ "EscapeRoomInput": { "actions": [], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Move": { "direction": "Right" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Move": { "direction": "Left" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Move": { "direction": "Up" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Move": { "direction": "Down" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Interact": { "direction": "Right" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Interact": { "direction": "Left" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Interact": { "direction": "Up" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Interact": { "direction": "Down" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Interact": { "direction": "Left" } }, { "Move": { "direction": "Left" } }], "send_message": null } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [{ "Move": { "direction": "Right" } }], "send_message": "hello" } }).to_string(),
        serde_json::json!({ "EscapeRoomInput": { "actions": [], "send_message": "multi\nline" } }).to_string(),
        "".to_string(),
        "not json".to_string(),
        serde_json::json!({ "NodeworldInput": { "action": null, "send_message": null } }).to_string()
    ];
    assert_eq!(base_inputs.len(), 15);
    (0..mock_states().len())
        .map(|i| base_inputs[i % base_inputs.len()].clone())
        .collect()
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
