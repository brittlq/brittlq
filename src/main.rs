use brittlq::{chatbot, get_user_config, register_subscriber, server::endpoints, subscriber_init};
use std::process::Command;

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
    let subscriber = subscriber_init();
    register_subscriber(subscriber);

    let (state_tx, state_rx) = tokio::sync::mpsc::channel(32);
    let (chat_tx, mut chat_rx) = tokio::sync::mpsc::channel(4);
    let bot_state_tx = state_tx.clone();

    let state_task = brittlq::init_state(state_rx);

    let server_task = tokio::spawn(async move {
        let server = warp::serve(endpoints::queue(state_tx, chat_tx));
        server.run(([0, 0, 0, 0], 8080)).await;
        Ok(()) as anyhow::Result<()>
    });

    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .args(&["/C", "start http://localhost:8080"])
            .output();
        if output.is_err() {
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
        chatbot::build_bot(&mut bot);
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
        e = state_task => {
            tracing::debug!("State task exited: {:?}", e);
            if let Err(error) = e.await? {
                tracing::error!("STATE TASK ERROR. {}", error);
            }
            Ok(()) as anyhow::Result<()>
        }
    }
}
