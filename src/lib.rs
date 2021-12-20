use serde::{Deserialize, Serialize, Serializer};
use std::collections::VecDeque;
use time::OffsetDateTime;
use tokio::sync::oneshot;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use uuid::Uuid;

pub type StateTx = tokio::sync::mpsc::Sender<StateCommand>;
pub type StateRx<T> = tokio::sync::oneshot::Receiver<T>;

pub mod chatbot;
pub mod config;
pub mod server;

#[derive(Debug)]
pub enum StateCommand {
    AddUser {
        user: String,
        tx: oneshot::Sender<usize>,
    },
    GetQueue(oneshot::Sender<serde_json::Value>),
    GetQueueStatus(oneshot::Sender<bool>),
    FindUser {
        name: String,
        tx: oneshot::Sender<Option<usize>>,
    },
    PeekQueue {
        count: u16,
        tx: oneshot::Sender<Vec<UserEntry>>,
    },
    PopQueue {
        count: u16,
        tx: oneshot::Sender<Option<Vec<UserEntry>>>,
    },
    RemoveUser {
        user: String,
        tx: oneshot::Sender<Option<()>>,
    },
    ToggleQueue(oneshot::Sender<bool>),
}

fn serialize_datetime<S>(date_time: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(
        &date_time
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap(),
    )
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserEntry {
    pub nickname: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub time_joined: OffsetDateTime,
    pub id: Uuid,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Queue {
    pub queue: VecDeque<UserEntry>,
    pub is_open: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    scope: String,
    token_type: String,
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

pub fn subscriber_init() -> impl Subscriber {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("qbot".into(), std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn register_subscriber<T>(subscriber: T)
where
    T: Subscriber + Send + Sync,
{
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub async fn init_state(
    mut state_rx: tokio::sync::mpsc::Receiver<StateCommand>,
) -> tokio::task::JoinHandle<Result<(), anyhow::Error>> {
    tokio::spawn(async move {
        use crate::StateCommand::*;
        let mut state = Queue {
            queue: VecDeque::new(),
            is_open: false,
        };

        while let Some(command) = state_rx.recv().await {
            match command {
                AddUser { user, tx } => {
                    let pos = find(&user, &state.queue);

                    if let Some(index) = pos {
                        tx.send(index).unwrap();
                    } else {
                        state.queue.push_back(UserEntry {
                            nickname: user,
                            time_joined: OffsetDateTime::now_local().unwrap(),
                            id: Uuid::new_v4(),
                        });
                        tx.send(state.queue.len() - 1).unwrap();
                    }
                }
                GetQueue(tx) => {
                    tx.send(serde_json::to_value(&state).unwrap()).unwrap();
                }

                GetQueueStatus(tx) => {
                    tx.send(state.is_open).unwrap();
                }

                FindUser { name, tx } => {
                    tx.send(find(&name, &state.queue)).unwrap();
                }

                PeekQueue { count, tx } => {
                    let first_n: Vec<_> =
                        state.queue.iter().take(count as usize).cloned().collect();
                    tx.send(first_n).unwrap();
                }

                PopQueue { count, tx } => {
                    let popped_users = pop(count, &mut state.queue);
                    tx.send(popped_users).unwrap();
                }

                RemoveUser { user, tx } => {
                    tx.send(remove(&user, &mut state.queue)).unwrap();
                }

                ToggleQueue(tx) => {
                    state.is_open = !state.is_open;
                    tx.send(state.is_open).unwrap();
                }
            }
        }
        tracing::error!("Exiting the state tasks for some reason");
        Ok(()) as anyhow::Result<()>
    })
}
