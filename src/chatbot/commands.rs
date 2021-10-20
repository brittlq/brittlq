use crate::chatbot::actions;

use super::actions::{BotAction, Command};
use std::collections::HashMap;

pub struct Channel {
    pub name: String,
    pub command_prefix: String,
    pub commands: HashMap<String, Vec<BotAction>>,
}

impl Channel {
    pub fn extract_command(&self, msg: &str) -> Option<&Vec<BotAction>> {
        if !msg.starts_with(&self.command_prefix) {
            return None;
        }
        let prefix_removed: &str = &msg[1..]; // TODO `! `, that is, prefix followed by a space, shouldn't be checked for command at all
        if prefix_removed.is_empty() {
            return None;
        }
        tracing::info!("Extracted {} from message: {}", &prefix_removed, msg);
        self.commands.get(prefix_removed)
    }

    pub fn edit_commands(&mut self, command: Command) -> Option<()> {
        // Should probably return a Result
        match command.action {
            // TODO improvements on the size of the stored Command by using `or_insert_with_key`
            // TODO utilize a real error type instead of an Option
            actions::CommandActions::Add => {
                if self.commands.contains_key(&command.name) {
                    // TODO Return an error that the command already exist
                    return None;
                }
                tracing::info!("Added command for {}: {}", self.name, &command.name);
                self.commands.insert(command.name, command.script);
                Some(())
            }
            actions::CommandActions::Edit => {
                tracing::info!("Edited command for {}: {}", self.name, &command.name);
                self.commands.insert(command.name, command.script);
                Some(())
            }
            actions::CommandActions::Remove => {
                self.commands.remove(&command.name);
                tracing::info!("Removed command for {}: {}", self.name, command.name);
                Some(())
            }
        }
    }
}
