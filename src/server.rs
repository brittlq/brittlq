use oneshot::error::RecvError;
use serde::Deserialize;
use tokio::sync::oneshot;

use crate::{StateCommand, StateRx, StateTx};

async fn dispatch<T>(tx: StateTx, rx: StateRx<T>, command: StateCommand) -> Result<T, RecvError> {
    tx.send(command).await.unwrap();
    rx.await
}

#[derive(Debug, Deserialize)]
pub struct NextQueryArg {
    count: Option<u16>,
}

mod handlers {
    use super::{dispatch, NextQueryArg};
    use crate::{
        chatbot::{self, Commands},
        StateCommand, StateTx, Token,
    };
    use std::convert::Infallible;
    use tokio::sync::oneshot;

    pub async fn delete_user(user: String, tx: StateTx) -> Result<impl warp::Reply, Infallible> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let removed_users = dispatch(tx, resp_rx, StateCommand::RemoveUser { user, tx: resp_tx })
            .await
            .unwrap();
        Ok(warp::reply::json(&removed_users))
    }

    pub async fn get_queue(tx: StateTx) -> Result<impl warp::Reply, Infallible> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let queue_status = dispatch(tx, resp_rx, StateCommand::GetQueue(resp_tx))
            .await
            .unwrap();
        Ok(warp::reply::json(&queue_status))
    }

    pub async fn toggle_queue(
        tx: StateTx,
        chatbot_tx: chatbot::Tx,
    ) -> Result<impl warp::Reply, Infallible> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let queue_status = dispatch(tx, resp_rx, StateCommand::ToggleQueue(resp_tx))
            .await
            .unwrap();
        chatbot_tx
            .send(Commands::SendMessage(format!(
                "The queue is now {}.",
                if queue_status { "open" } else { "closed" }
            )))
            .await
            .unwrap();
        Ok(warp::reply::json(&queue_status))
    }

    pub async fn pop_queue(
        args: NextQueryArg,
        tx: StateTx,
        chatbot_tx: chatbot::Tx,
    ) -> Result<impl warp::Reply, Infallible> {
        let (resp_tx, resp_rx) = oneshot::channel();
        tracing::debug!("Popping: {}", args.count.unwrap_or(4));
        let popped_entries = dispatch(
            tx,
            resp_rx,
            StateCommand::PopQueue {
                count: args.count.unwrap_or(4),
                tx: resp_tx,
            },
        )
        .await
        .unwrap();
        if let Some(popped) = &popped_entries {
            let temp_users = popped
                .iter()
                .map(|u| u.nickname.clone())
                .collect::<Vec<String>>();
            let names_message = temp_users.join(", @");
            chatbot_tx
                .send(Commands::SendMessage(format!(
                    "Up next: @{}. You can reach BK in game with the following message: @brittleknee Hi.",
                    names_message
                )))
                .await
                .unwrap();
        }
        Ok(warp::reply::json(&popped_entries))
    }

    pub async fn send_token(token: Token, tx: chatbot::Tx) -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(
            &tx.send(chatbot::Commands::Token(token)).await.unwrap(),
        ))
    }
}

pub mod endpoints {
    use super::{handlers, NextQueryArg, StateTx};
    use crate::chatbot;

    use warp::Filter;

    pub fn queue(
        tx: StateTx,
        chatbot_tx: chatbot::Tx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        queue_get(tx.clone())
            .or(queue_pop(tx.clone(), chatbot_tx.clone()))
            .or(queue_toggle(tx.clone(), chatbot_tx.clone()))
            .or(token(chatbot_tx))
            .or(user_delete(tx))
            .or(health())
            .or(warp::fs::dir("./www/dist/"))
            .with(warp::trace(
                |info| tracing::info_span!("API request", method = %info.method(), path = %info.path(), id = %uuid::Uuid::new_v4().to_hyphenated()),
            ))
    }

    fn with_tx<T>(
        tx: tokio::sync::mpsc::Sender<T>,
    ) -> impl Filter<Extract = (tokio::sync::mpsc::Sender<T>,), Error = std::convert::Infallible> + Clone
    where
        T: Send + Sync,
    {
        warp::any().map(move || tx.clone())
    }

    // DELETE /queue/:name
    pub fn user_delete(
        tx: StateTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("queue" / String)
            .and(warp::delete())
            .and(with_tx(tx))
            .and_then(handlers::delete_user)
            .with(warp::trace::named("user"))
    }

    // GET /queue
    pub fn queue_get(
        tx: StateTx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("queue")
            .and(warp::get())
            .and(with_tx(tx))
            .and_then(handlers::get_queue)
            .with(warp::trace::named("queue"))
    }

    // GET /queue/toggle
    pub fn queue_toggle(
        tx: StateTx,
        chatbot_tx: chatbot::Tx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("queue" / "toggle")
            .and(warp::get())
            .and(with_tx(tx))
            .and(with_tx(chatbot_tx))
            .and_then(handlers::toggle_queue)
            .with(warp::trace::named("toggle"))
    }
    // GET /queue/pop?:u16
    pub fn queue_pop(
        tx: StateTx,
        chatbot_tx: chatbot::Tx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("queue" / "pop")
            .and(warp::get())
            .and(warp::query::<NextQueryArg>())
            .and(with_tx(tx))
            .and(with_tx(chatbot_tx))
            .and_then(handlers::pop_queue)
            .with(warp::trace::named("pop"))
    }

    // GET /health
    pub fn health() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("health").and(warp::get()).map(warp::reply)
    }

    // TODO This gets removed once the backend is running seperately. ATM we are using the implict auth flow, which is best for client side authentication.
    // Once this is no longer running on the client, we'll need to use an approach that utilizes client secrets instead.
    pub fn token(
        tx: chatbot::Tx,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("queue" / "token")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_tx(tx))
            .and_then(handlers::send_token)
            .with(warp::trace::named("token"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        init_state,
        {chatbot, server::endpoints, StateCommand},
    };

    struct Txs {
        state_tx: tokio::sync::mpsc::Sender<StateCommand>,
        state_rx: tokio::sync::mpsc::Receiver<StateCommand>,
        chat_tx: chatbot::Tx,
        _chat_rx: chatbot::Rx,
    }

    fn init_tx() -> Txs {
        let (state_tx, state_rx) = tokio::sync::mpsc::channel(1);
        let (chat_tx, _chat_rx) = tokio::sync::mpsc::channel(1);
        Txs {
            state_tx,
            state_rx,
            chat_tx,
            _chat_rx,
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn get_health_check_200() {
        let health_filter = endpoints::health();

        let value = warp::test::request()
            .path("/health")
            .reply(&health_filter)
            .await;

        assert_eq!(value.status(), 200);
        assert_eq!(value.body(), "");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn post_health_check_405() {
        let health_filter = endpoints::health();

        let value = warp::test::request()
            .path("/health")
            .method("POST")
            .reply(&health_filter)
            .await;

        assert_eq!(value.status(), 405);
    }
}
