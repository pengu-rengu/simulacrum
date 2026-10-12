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
use std::io::Write;

fn agent_file_path(agent_name: &str) -> PathBuf {
    let mut path = PathBuf::from("agents");
    path.push(agent_name);
    path.set_extension("txt");
    path
}

fn agent_log_file_path(agent_name: &str) -> PathBuf {
    let mut path = PathBuf::from("agent_logs");
    path.push(agent_name);
    path.set_extension("txt");
    path
}

fn write_log(agent_name: &str, log: &str) -> Result<(), std::io::Error> {
    let path = agent_log_file_path(agent_name);

    let mut file = File::options()
        .append(true)
        .open(path)?;

    writeln!(file, "{}\n\n", log)?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    if args().any(|arg| arg == "check") {
        check();
        return Ok(());
    }

    let mut state= State::new();

    for agent in &state.agents {
        let agent_path = agent_file_path(&agent.name);
        File::create(agent_path)?;
        
        let log_path = agent_log_file_path(&agent.name);
        File::create(log_path)?;
    }

    loop {
        let (flushed_state, output) = flush_state(&state);

        let agent_name = &flushed_state.agents[flushed_state.agent_idx].name;
        write_log(agent_name, &output)?;

        let path = agent_file_path(agent_name);
        write(&path, &output)?;

        let one_second = Duration::from_secs(1);
        let input = loop {
            sleep(one_second);
            let file_contents = read_to_string(&path)?;
            if file_contents != output {
                break file_contents;
            }
        };

        write_log(agent_name, &input)?;

        state = next_state(&flushed_state, &input);
    }
}
