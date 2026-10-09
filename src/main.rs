mod impl_;
mod spec;
mod state;

use state::state::State;
use std::fs::{File, write, read_to_string};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use std::env::args;
use impl_::next_state::{flush_state, next_state};
use spec::check::check;

fn agent_file_path(agent_name: &str) -> PathBuf {
    let mut path = PathBuf::from("agents");
    path.push(agent_name);
    path.set_extension("txt");
    path
}

fn main() -> Result<(), std::io::Error> {
    if args().any(|arg| arg == "check") {
        check();
        return Ok(());
    }

    let mut state= State::new();

    for agent in &state.agents {
        let path = agent_file_path(&agent.name);
        File::create(path)?;
    }

    loop {
        let (flushed_state, output) = flush_state(&state);
        
        let path = agent_file_path(&flushed_state.agents[flushed_state.agent_idx].name);
        write(&path, &output)?;

        let one_second = Duration::from_secs(1);
        let input =  loop {
            sleep(one_second);
            let input = read_to_string(&path)?;
            if input != output {
                break input;
            }
        };

        state = next_state(&flushed_state, &input);
    }
}
