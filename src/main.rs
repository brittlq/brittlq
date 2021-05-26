use brittlq::{set_tracer, trace_init};
use chrono::prelude::*;
use std::collections::VecDeque;
use std::process::Command;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use utils::{chatbot, get_user_config, pop, remove, Queue};
use uuid::Uuid;

mod utils;

/* THE BIG TODO
 * Split the tasks up:
 * 1. a task that is waiting on an mpsc::Receiver for info on how to change the queues
 *      * Must receive:
 *          [] User identity
 *              - Server: Twitch oauth token
 *              - chatbot: channel name
 *          [] Response oneshot channel
 * 2. API/Server task
 * 3. IRC chatbot task
 *
 * API and IRC tasks will both need to communicate with the Receiver task managing global state
 * API and IRC will also need to communicate between each other
 *
 * ┌────────────┐                  ┌────────────┐
 * │            │   API request    │            │ API-invoked command
 * │   client   │─────────────────>│   server   │<─────────────────────┐
 * │            │                  │            │                      │
 * └────────────┘                  └────────────┘                      v
 *                                        │                       ┌─────────┐
 *                                        │ chat message          │  state  │
 *                                        v                       └─────────┘
 *                                ┌───────────────┐                    ^
 *                                │               │                    │
 *                                │  IRC client   │<───────────────────┘
 *                                │               │ Chat-invoked command
 *                                └───────────────┘
 *                                        ^
 *                                        │  chat
 *                                        v
 *                                 ┌───────────────┐
 *                               ┌─┴─────────────┐ │
 *                               │               │ │
 *                               │  IRC channel  │ │
 *                               │               ├─┘
 *                               └───────────────┘
 */

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up tracing system
    let subscriber = trace_init();
    set_tracer(&subscriber);

    let (state_tx, mut state_rx) = tokio::sync::mpsc::channel(32);
    let (chat_tx, mut chat_rx) = tokio::sync::mpsc::channel(4);
    let bot_state_tx = state_tx.clone();

    let state_task = tokio::spawn(async move {
        use crate::utils::{find, StateCommand::*, UserEntry};
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
                            time_joined: Local::now(),
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
        Ok(()) as anyhow::Result<()>
    });

    let server_task = tokio::spawn(async move {
        let server = warp::serve(utils::server::endpoints::queue(state_tx, chat_tx));
        server.run(([127, 0, 0, 1], 8080)).await;
        Ok(()) as anyhow::Result<()>
    });

    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(&["/C", "start http://localhost:8080"])
            .output();
        if let Err(_) = output {
            tracing::error!("Could not launch browser");
        }
    }

    let mut auth = String::new();
    if let Some(chatbot::Commands::Token(token)) = chat_rx.recv().await {
        auth = format!("oauth:{}", token.access_token);
    }

    let mut bot = chatbot::Bot::new(get_user_config(&auth), chat_rx)
        .await
        .unwrap();

    let bot_task = tokio::spawn(async move {
        utils::chatbot::build_bot(&mut bot);
        bot.run(bot_state_tx).await
    });

    tokio::select! {
        _ = bot_task => {
            tracing::debug!("Bot task exited.");
            Ok(()) as anyhow::Result<()>
        }
        _ = server_task => {
            tracing::debug!("Server task exited.");
            Ok(()) as anyhow::Result<()>
        }
        _ = state_task => {
            tracing::debug!("State task exited.");
            Ok(()) as anyhow::Result<()>
        }
    }
}
