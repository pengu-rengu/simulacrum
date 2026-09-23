use crate::state::state::{State, Agent, Node};

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
                },
                Agent {
                    name: "Agent2".to_string(),
                    node_idx: 0,
                    node_messages_inbox: vec![],
                },
            ],
            nodes: vec![Node {
                name: "Node1".to_string()
            }],
        }
    }
}