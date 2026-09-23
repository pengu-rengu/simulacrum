use crate::spec::spec::{check_next_state, check_flush_state};
use crate::state::state::{State, Agent, Node};
use crate::impl_::next_state::{flush_state, next_state};
use serde_json::json;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + (self.next() % (hi - lo + 1) as u64) as usize
    }
}

fn mock_message(rng: &mut Rng) -> String {
    let messages = ["hi", "", "line1\nline2", "quote \" here", "unicode ✓"];
    messages[rng.range(0, messages.len() - 1)].to_string()
}

fn mock_agent(i: usize, node_idx: usize, inbox_len: usize, rng: &mut Rng) -> Agent {
    Agent {
        name: format!("Agent{i}"),
        node_idx,
        node_messages_inbox: (0..inbox_len).map(|_| mock_message(rng)).collect(),
    }
}

fn mock_nodes(n: usize) -> Vec<Node> {
    (0..n).map(|i| Node { name: format!("Node{i}") }).collect()
}

fn mock_random_state(rng: &mut Rng) -> State {
    let num_agents = rng.range(1, 5);
    let num_nodes = rng.range(1, 4);
    let agents = (0..num_agents)
        .map(|i| {
            let node_idx = rng.range(0, num_nodes - 1);
            let inbox_len = rng.range(0, 3);
            mock_agent(i, node_idx, inbox_len, rng)
        })
        .collect();
    State {
        turn: rng.range(0, 10),
        agent_idx: rng.range(0, num_agents - 1),
        agents,
        nodes: mock_nodes(num_nodes),
    }
}

fn mock_states() -> Vec<State> {
    let mut rng = Rng(0x2545F4914F6CDD1D);
    let mut states = vec![State::new()];

    // Single agent with a non-empty inbox
    states.push(State {
        turn: 3,
        agent_idx: 0,
        agents: vec![mock_agent(0, 0, 2, &mut rng)],
        nodes: mock_nodes(1),
    });
    // All agents on one node, last agent's turn
    states.push(State {
        turn: 0,
        agent_idx: 3,
        agents: (0..4).map(|i| mock_agent(i, 0, i, &mut rng)).collect(),
        nodes: mock_nodes(1),
    });
    // Every agent on a different node
    states.push(State {
        turn: 1,
        agent_idx: 1,
        agents: (0..4).map(|i| mock_agent(i, i, 1, &mut rng)).collect(),
        nodes: mock_nodes(4),
    });

    for _ in 0..200 {
        states.push(mock_random_state(&mut rng));
    }
    states
}

fn mock_input_strs() -> Vec<String> {
    vec![
        "".to_string(),
        "not json".to_string(),
        "[]".to_string(),
        json!({}).to_string(),
        json!({ "send_message": null }).to_string(),
        json!({ "send_message": "hello" }).to_string(),
        json!({ "send_message": "" }).to_string(),
        json!({ "send_message": "multi\nline" }).to_string(),
        json!({ "send_message": "with \"quotes\"" }).to_string(),
    ]
}

pub fn check() {
    let inputs = mock_input_strs();
    for state in mock_states() {
        let (flushed_state, output) = flush_state(&state);
        check_flush_state(&state, &flushed_state, &output);

        for input_str in &inputs {
            let new_state = next_state(&state, input_str);
            check_next_state(&state, &new_state, input_str);

            let new_state = next_state(&flushed_state, input_str);
            check_next_state(&flushed_state, &new_state, input_str);
        }
    }
}
