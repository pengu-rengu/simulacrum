use crate::state::initial::nodes::initial_nodes;
use crate::state::nodeworld::{BodyCell, BodyGrid};
use crate::state::state::{State, Universe};
use crate::state::escaperoom::{DoorCell, EscapeRoomCell, Room, EscapeRoomAgent};
use crate::state::nodeworld::NodeworldAgent;
use crate::state::state::Agent;

fn initial_body_grid() -> BodyGrid {
    BodyGrid {
        width: 1,
        height: 1,
        cells: vec![BodyCell::CoreCell { health: 10 }],
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
                    universe: Universe::Nodeworld,
                    error_message: None,
                    nodeworld: NodeworldAgent {
                        node_idx: 0,
                        node_messages_inbox: vec![],
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
                    universe: Universe::Nodeworld,
                    error_message: None,
                    nodeworld: NodeworldAgent {
                        node_idx: 0,
                        node_messages_inbox: vec![],
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
            nodes: initial_nodes(),
            rooms: vec![]
        }
    }
}