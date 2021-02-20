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
            let position = match find(args.msg.sender, &args.queue.queue) {
                Some(index) => {
                    index + 1
                }
                None => {
                    let s = UserEntry{ nickname: args.msg.sender.to_owned(), time_joined: Local::now(), id: Uuid::new_v4(), disabled: false };
                    args.queue.queue.push_back(s);
                    args.queue.queue.len()
                }
            };
            let response = format!("{}, you're #{} in the queue", args.msg.sender, position);
            log::debug!("{}", response);
            args.writer.send_privmsg(&args.msg.target, &response).unwrap();
        })
        .with_command("!next", handlers::peek)
        .with_command("!place", |args: Args| {
            match find(args.msg.sender, &args.queue.queue) {
                Some(index) => {
                    let leading_groups = index / 4;
                    let wait_time = leading_groups * 5;
                    
                    let response = match leading_groups {
                        0 => format!("{} is #{}. You're on deck! There is 1 group ahead of you, wait time is approximately {}-{} minutes", args.msg.sender, index + 1, wait_time, wait_time + 5),
                        _ => format!("{} is #{}. There are {} groups ahead of you, wait time is approximately {} minutes", args.msg.sender, index + 1, leading_groups, wait_time),
                    };
                    args.writer.send_privmsg(args.msg.target, &response).unwrap();
                }
                None => {
                    args.writer.send_privmsg(args.msg.target, &format!("{} is not in the queue", args.msg.sender)).unwrap();
                }
            }
        })
        .with_command("!leave", |args: Args| {
            if remove(args.msg.sender, &mut args.queue.queue).is_some() {
                args.writer.send_privmsg(args.msg.target, &format!("You've been removed from the queue, {}.", args.msg.sender)).unwrap();
            }
        })
}
