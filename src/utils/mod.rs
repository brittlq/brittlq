pub mod bot;
pub mod handlers;

use chrono::prelude::*;
use irc::client::prelude::*;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

use std::collections::{HashMap, VecDeque};

fn serialize_datetime<S>(date_time: &DateTime<Local>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    const TIME_FMT: &str = "%H:%M:%S";
    s.serialize_str(&date_time.format(TIME_FMT).to_string())
}
#[derive(Deserialize, Serialize)]
pub struct UserEntry {
    pub nickname: String,
    #[serde(serialize_with = "serialize_datetime")]
    time_joined: DateTime<Local>,
    id: Uuid,
    disabled: bool,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Queue {
    pub queue: VecDeque<UserEntry>,
    pub is_open: bool,
}

pub fn pop(num: u16, user_queue: &mut VecDeque<UserEntry>) -> Option<Vec<UserEntry>> {
    let mut popped_entries = Vec::new();
    for _ in 0..num {
        if let Some(x) = user_queue.pop_front() {
            popped_entries.push(x);
        }
    }
    if popped_entries.is_empty() {
        None
    } else {
        Some(popped_entries)
    }
}
pub fn get_user_config(token: &str) -> Config {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("Settings").required(false))
        .unwrap()
        .merge(config::Environment::with_prefix("TWITCH"))
        .unwrap();
    let config = settings.try_into::<HashMap<String, String>>().unwrap();

    let name = match config.get("name") {
        Some(n) => n,
        None => {
            panic!("Expected `name` in Settings.toml or TWITCH_NAME environment variable");
        }
    };
    let channel = match config.get("channel") {
        Some(n) => {
            if n.starts_with('#') {
                n.clone()
            } else {
                format!("#{}", n)
            }
        }
        None => {
            panic!("Expected `channel` in Settings.toml or TWITCH_CHANNEL environment variable");
        }
    };

    Config {
        nickname: Some(name.clone()),
        password: Some(token.to_owned()),
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6697),
        channels: vec![channel],
        ..Default::default()
    }
}

pub fn find(nickname: &str, user_queue: &VecDeque<UserEntry>) -> Option<usize> {
    user_queue
        .iter()
        .position(|entry: &UserEntry| entry.nickname == nickname)
}

pub fn remove(name: &str, user_queue: &mut VecDeque<UserEntry>) -> Option<()> {
    match find(name, user_queue) {
        Some(index) => user_queue.remove(index).map(|_| ()),
        None => None,
    }
}
