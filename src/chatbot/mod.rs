use self::commands::Channel;
use crate::{StateTx, Token};
use actions::BotAction;
use futures::prelude::*;
mod irc {
    pub use irc::{client::prelude::*, error::*};
}
use std::collections::HashMap;

pub mod actions;
pub mod commands;

#[derive(Debug)]
pub enum Commands {
    SendMessage(String),
    Token(Token),
}

pub type Tx = tokio::sync::mpsc::Sender<Commands>;
pub type Rx = tokio::sync::mpsc::Receiver<Commands>;

#[derive(Clone)]
pub struct Message {
    pub target: String,
    pub sender: String,
    pub message: String,
}

struct QueuePos<'a> {
    index: Option<usize>,
    user_nickname: &'a str,
    group_size: usize,
    wait_per_group: usize, // TODO this should be a chrono::Duration
}

impl std::fmt::Display for QueuePos<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.index {
            Some(index) => {
                let leading_groups = index / self.group_size;
                let wait_time = leading_groups * self.wait_per_group;
                let leading_groups_preamble = match leading_groups {
                    0 => "You're on deck".to_owned(),
                    _ => format!("There are {} groups ahead of you", leading_groups),
                };
                write!(
                    f,
                    "{} is #{}. {}, wait time is approximately {}-{} minutes",
                    self.user_nickname,
                    index + 1,
                    leading_groups_preamble,
                    wait_time,
                    wait_time + 5
                )
            }
            None => {
                write!(f, "{} is not in the queue", self.user_nickname)
            }
        }
    }
}

async fn execute_actions(
    trigger: Message,
    sender: ::irc::client::Sender,
    actions: Vec<BotAction>, // Bleh, but the values need to be moved in rather than passed as a slice to avoid potential use-after-free
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        for action in actions {
            match action {
                BotAction::Say(msg) => {
                    tracing::debug!("Sending message:{}", &msg);
                    if let Err(e) = sender.send_privmsg(&trigger.target, &msg) {
                        tracing::error!(
                            "Failed to send message to {}\n\tError:{}\n\tMessage:{}",
                            trigger.target,
                            e,
                            msg
                        );
                    }
                }
                BotAction::Wait(duration) => {
                    tracing::debug!("Sleeping for {}", duration);
                    tokio::time::sleep(duration.to_std().unwrap()) // Safe to unwrap because we check for positive values at the time the command is created
                        .await;
                }
            }
        }
    })
}

pub struct Bot {
    channel: String,
    client: irc::Client,
    // commands: HashMap<String, Box<dyn Handler>>,
    channels: HashMap<String, Channel>,
    rx: Rx,
}

impl Bot {
    pub async fn new(user_config: irc::Config, rx: Rx) -> Result<Bot, irc::Error> {
        let channel: String = user_config.channels.iter().take(1).cloned().collect();
        let client = irc::Client::from_config(user_config).await?;
        client.identify()?;
        let mut channels = HashMap::new();
        channels.insert(
            channel.clone(),
            Channel {
                name: channel.clone(),
                command_prefix: "!".to_owned(),
                commands: HashMap::new(),
            },
        );
        Ok(Bot {
            channel,
            client,
            // commands: HashMap::new(),
            channels,
            rx,
        })
    }

    // run the bot until its done
    pub async fn run(&mut self, tx: StateTx) -> anyhow::Result<()> {
        tracing::debug!("starting main loop");
        let mut stream = self.client.stream()?;
        let sender = self.client.sender();

        loop {
            tokio::select! {
                // Oh wow that Option<Result<_>> nesting is pretty gnarly
                Some(Ok(message)) = stream.next() => {
                    tracing::debug!("{}", message);
                    match message.command {
                        irc::Command::PRIVMSG(ref target, ref msg) =>
                        {
                            if let Some(channel) = self.channels.get_mut(target) {
                                if let Some(actions) = channel.extract_command(msg) {
                                    let msg = msg.clone();
                                    let irc_response_data =
                                        Message {
                                            target: message.response_target().unwrap().to_string(),
                                            sender: message.source_nickname().unwrap().to_string(),
                                            message: msg,
                                        };

                                        let command_sender = sender.clone();
                                        execute_actions(irc_response_data, command_sender, actions.to_vec()).await;
                                } else if let Ok(command_editor) = actions::action_parser::command(&msg[1..]) {
                                    let name = command_editor.name.clone(); // TODO don't really need this clone here, we can be smart instead
                                    match channel.edit_commands(command_editor) {
                                        Some(_) => {
                                            // TODO I dont' think we should just be `unwrap`ing these
                                            sender.send_privmsg(message.response_target().unwrap(), format!("Command edit successful: {}", name));
                                        }
                                        None => {
                                            sender.send_privmsg(message.response_target().unwrap(), format!("Command edit failed: {}", name));
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            tracing::debug!("IRC commmand not supported");
                        }
                    }
                },
                Some(command) = self.rx.recv() => {
                    match command {
                        Commands::SendMessage(message) => {sender.send_privmsg(&self.channel, &message).unwrap();},
                        Commands::Token(_) => tracing::debug!("Already handled token"),
                    }
                }
                else => break,
            }
        }

        tracing::trace!("end of main loop");
        Ok(())
    }

    pub fn parse_command(input: &str) -> Option<&str> {
        if !input.starts_with('!') {
            return None;
        }
        input.splitn(2, ' ').next()
    }
}
