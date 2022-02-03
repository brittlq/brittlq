use brittlq::{chatbot, config::get_user_config, register_subscriber, server, subscriber_init};
use std::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    // Set up tracing system
    let subscriber = subscriber_init();
    register_subscriber(subscriber);

    let (state_tx, state_rx) = tokio::sync::mpsc::channel(32);
    let (chat_tx, mut chat_rx) = tokio::sync::mpsc::channel(4);
    let bot_state_tx = state_tx.clone();

    let state_task = brittlq::init_state(state_rx);

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    let sql_pool = sqlx::PgPool::connect(&database_url).await?;

    let server_task = tokio::spawn(async move {
        let server = warp::serve(server::queue::api::routes(sql_pool));
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

    let runtime_config = get_user_config(&auth)?;
    let mut bot = chatbot::Bot::new(runtime_config, chat_rx).await.unwrap();

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
