use crate::state::state::State;
use crate::state::agent::Agent;
use crate::spec::common::item_label;

/// What the agent carries, indexed the way `harvest` expects a `tool_idx`.
pub fn inventory_lines(agent: &Agent) -> Vec<String> {
    let mut lines = vec!["Inventory:".to_string()];
    for item_stack in &agent.nodeworld.inventory {
        lines.push(format!("{} {}", item_stack.count, item_label(&item_stack.item)));
    }
    lines.push("".to_string());
    lines
}

/// What the agent heard since its last turn. Reaches the agent whether or not a menu is open.
pub fn message_lines(state: &State, agent: &Agent) -> Vec<String> {
    let mut lines = vec!["Messages:".to_string()];
    for message in &agent.nodeworld.node_messages_inbox {
        lines.push(format!("[{}] {}", state.agents[message.sender_agent_idx].name, message.content));
    }
    lines
}

/// How the last action failed, if it did.
pub fn error_lines(error_message: &Option<String>) -> Vec<String> {
    match error_message {
        Some(error_message) => vec!["".to_string(), format!("Error: {}", error_message)],
        None => vec![],
    }
}
