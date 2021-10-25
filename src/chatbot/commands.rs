use chrono::Duration;

use crate::chatbot::actions;

use super::actions::{EditorAction, ScriptAction};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ExecutionError {
    InsufficientAccessLevel {
        required: AccessLevel,
        actual: AccessLevel,
    },
    Cooldown(Duration),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum AccessLevel {
    User,
    Moderator,
    Broadcaster,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub enabled: bool,
    pub access_level: AccessLevel,
    pub script: Vec<ScriptAction>,
    pub cooldown: Duration,
    pub last_execution: Option<chrono::DateTime<chrono::Utc>>,
}

impl Command {
    pub async fn execute(
        &mut self,
        target: &str,
        sender: &::irc::client::Sender,
    ) -> Result<(), ExecutionError> {
        let le = self
            .last_execution
            .unwrap_or(chrono::Utc::now() - self.cooldown);
        let time_elapsed = chrono::Utc::now() - le;
        if time_elapsed < self.cooldown {
            return Err(ExecutionError::Cooldown(self.cooldown - time_elapsed));
        }
        self.last_execution = Some(chrono::Utc::now());
        for action in &self.script {
            match action {
                ScriptAction::Say(msg) => {
                    if let Err(e) = sender.send_privmsg(target, msg) {
                        tracing::error!(
                            "Failed to send message to {}\n\tError:{}\n\tMessage:{}",
                            target,
                            e,
                            msg
                        );
                    }
                }
                ScriptAction::Wait(duration) => {
                    tokio::time::sleep(duration.to_std().unwrap()) // Safe to unwrap because we check for positive values at the time the command is created
                        .await;
                }
            }
        }
        Ok(())
    }
}

pub struct Channel {
    pub name: String,
    pub command_prefix: String,
    pub commands: HashMap<String, Command>,
}

impl Channel {
    pub fn extract_command(&mut self, msg: &str) -> Option<&mut Command> {
        if !msg.starts_with(&self.command_prefix) {
            return None;
        }
        let prefix_removed: &str = &msg[1..]; // TODO `! `, a prefix followed by a space, shouldn't be checked for command at all
        if prefix_removed.is_empty() {
            return None;
        }
        tracing::info!("Extracted {} from message: {}", &prefix_removed, msg);
        self.commands.get_mut(prefix_removed)
    }

    pub fn edit_commands(&mut self, editor_action: EditorAction) -> Option<()> {
        // Should probably return a Result
        match editor_action.action {
            // TODO improvements on the size of the stored Command by using `or_insert_with_key`
            // TODO utilize a real error type instead of an Option
            actions::ActionTag::Add => {
                if self.commands.contains_key(&editor_action.name) {
                    // TODO Return an error that the command already exist
                    return None;
                }
                tracing::info!("Added command for {}: {}", self.name, &editor_action.name);
                self.commands
                    .insert(editor_action.name, editor_action.command.unwrap()); // It is safe to unwrap `command` because a None for Add and Edit action tags cannot make it to this point
                Some(())
            }
            actions::ActionTag::Edit => {
                tracing::info!("Edited command for {}: {}", self.name, &editor_action.name);
                self.commands
                    .insert(editor_action.name, editor_action.command.unwrap());
                Some(())
            }
            actions::ActionTag::Remove => {
                self.commands.remove(&editor_action.name);
                tracing::info!("Removed command for {}: {}", self.name, editor_action.name);
                Some(())
            }
        }
    }
}
