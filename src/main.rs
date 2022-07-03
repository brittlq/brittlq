use axum::{
    routing::{get, get_service},
    Extension, Router,
};
use brittlq::server::handlers;
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "brittlq_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (state_tx, state_rx) = tokio::sync::mpsc::channel(32);

    let state_task = brittlq::init_state(state_rx);

    let app = Router::new()
        .route(
            "/queue",
            get(handlers::get_queue).delete(handlers::delete_user),
        )
        .route("/queue/pop", get(handlers::pop_queue))
        .route("/queue/toggle", get(handlers::toggle_queue))
        .route("/health", get(handlers::empty))
        .fallback(get_service(ServeDir::new("./www/dist")).handle_error(handlers::handle_error))
        .layer(Extension(state_tx))
        .layer(TraceLayer::new_for_http());

    let server_task = tokio::spawn(async move {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
        Ok(()) as anyhow::Result<()>
    });

    tokio::select! {
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
