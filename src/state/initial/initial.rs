use crate::state::initial::nodes::initial_nodes;
use crate::state::nodeworld::{BodyCell, BodyGrid};
use crate::state::state::{State, Universe};
use crate::state::escaperoom::{DoorCell, EscapeRoomCell, Room, EscapeRoomAgent};
use crate::state::nodeworld::NodeworldAgent;
use crate::state::state::Agent;
use crate::state::initial::rooms::ROOM1;

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
                'E' => EscapeRoomCell::Exit,
                '1' | '2' | '3' | '4' => EscapeRoomCell::Spawn(char.to_digit(10).unwrap() as usize - 1),
                _ => EscapeRoomCell::Empty
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

fn initial_agent(name: &str, x: usize, y: usize) -> Agent {
    Agent {
        name: name.to_string(),
        universe: Universe::EscapeRoom,
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
            x: x,
            y: y,
            messages_inbox: vec![],
            finished: false
        }
    }
}

impl State {
    pub fn new() -> State {
        State { 
            turn: 0,
            agent_idx: 0,
            agents: vec![
                initial_agent("Agent1", 1, 1),
                initial_agent("Agent2", 3, 1),
            ],
            nodes: initial_nodes(),
            rooms: vec![
                room_from_str("Room1", ROOM1),
            ]
        }
    }
}