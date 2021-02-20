use crate::utils::{find, handlers, remove, Queue, UserEntry};
use chrono::prelude::*;
use futures::prelude::*;
use serde::{Deserialize, Serialize};
mod irc {
    pub use irc::client::prelude::*;
}
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct Message<'a> {
    pub target: &'a str,
    pub sender: &'a str,
    pub message: &'a str,
}

pub struct Args<'a> {
    pub msg: Message<'a>,
    pub writer: &'a irc::Sender,
    pub queue: &'a mut Queue,
}

pub trait Handler: Send + Sync {
    fn handle(&mut self, args: Args<'_>);
}

impl<F> Handler for F
where
    F: FnMut(Args<'_>),
    F: Send + Sync,
{
    fn handle(&mut self, args: Args<'_>) {
        (self)(args)
    }
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
                write!(f, "{} is #{}. {}, wait time is approximately {}-{} minutes", self.user_nickname, index + 1, leading_groups_preamble, wait_time, wait_time + 5)
            }
            None => {
                write!(f, "{} is not in the queue", self.user_nickname)
            }
        }
       
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Bot {
    #[serde(skip)]
    commands: HashMap<String, Box<dyn Handler>>,
    pub queue: Arc<Mutex<Queue>>,
}

impl Bot {
    // add this command to the bot
    pub fn with_command(mut self, name: impl Into<String>, cmd: impl Handler + 'static) -> Self {
        self.commands.insert(name.into(), Box::new(cmd));
        self
    }

    // run the bot until its done
    pub async fn run(&mut self, user_config: irc::Config) -> anyhow::Result<()> {
        let mut client = irc::Client::from_config(user_config).await?;

        client.identify()?;

        log::debug!("starting main loop");
        let mut stream = client.stream()?;
        let sender = client.sender();

        while let Some(message) = stream.next().await.transpose()? {
            log::debug!("{}", message);
            if let irc::Command::PRIVMSG(ref _target, ref msg) = message.command {
                // see if its a command and do stuff with it
                if let Some(cmd) = Self::parse_command(msg) {
                    if let Some(command) = self.commands.get_mut(cmd) {
                        log::trace!("dispatching to: {}", cmd.escape_debug());

                        let args = Args {
                            msg: Message {
                                target: message.response_target().unwrap(),
                                sender: message.source_nickname().unwrap(),
                                message: msg,
                            },
                            writer: &sender,
                            queue: &mut self.queue.lock().unwrap(),
                        };

                        command.handle(args);
                    }
                }
            }
        }

        log::trace!("end of main loop");
        Ok(())
    }

    pub fn parse_command(input: &str) -> Option<&str> {
        if !input.starts_with('!') {
            return None;
        }
        input.splitn(2, ' ').next()
    }
}

pub fn build_bot() -> Bot {
    Bot::default()
        .with_command("!join", |args: Args| {
            if !args.queue.is_open {
                return;
            }
            
            let mut queue_pos = QueuePos {
                index: find(args.msg.sender, &args.queue.queue),
                user_nickname: args.msg.sender, 
                group_size: 4, 
                wait_per_group: 5
            };

            if queue_pos.index.is_none() {
                let s = UserEntry{ nickname: args.msg.sender.to_owned(), time_joined: Local::now(), id: Uuid::new_v4(), disabled: false };
                args.queue.queue.push_back(s);
                queue_pos.index = Some(args.queue.queue.len() - 1);
            };

            args.writer.send_privmsg(&args.msg.target, &format!("{}", queue_pos)).unwrap();
        })
        .with_command("!next", handlers::peek)
        .with_command("!place", |args: Args| {
            let queue_pos = QueuePos {
                index: find(args.msg.sender, &args.queue.queue),
                user_nickname: args.msg.sender, 
                group_size: 4, 
                wait_per_group: 5
            };
            args.writer.send_privmsg(args.msg.target, &format!("{}", queue_pos)).unwrap();
        })
        .with_command("!leave", |args: Args| {
            if remove(args.msg.sender, &mut args.queue.queue).is_some() {
                args.writer.send_privmsg(args.msg.target, &format!("{} has been removed from the queue.", args.msg.sender)).unwrap();
            }
        })
}
