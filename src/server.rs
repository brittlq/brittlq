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
    count: u16,
}

impl Default for NextQueryArg {
    fn default() -> Self {
        Self { count: 4 }
    }
}

pub mod handlers {
    use super::{dispatch, NextQueryArg};
    use crate::{
        StateCommand, StateTx,
    };
    use axum::{
        extract::{Path, Query},
        http::StatusCode,
        Extension,
    };
    use axum::{response::IntoResponse, Json};
    use std::io;
    use tokio::sync::oneshot;

    pub async fn delete_user(
        Path(user): Path<String>,
        Extension(tx): Extension<StateTx>,
    ) -> impl IntoResponse {
        let (resp_tx, resp_rx) = oneshot::channel();
        let removed_users = dispatch(tx, resp_rx, StateCommand::RemoveUser { user, tx: resp_tx })
            .await
            .unwrap();
        Json(removed_users)
    }

    // `()` gives an empty response
    pub async fn empty() {}

    pub async fn get_queue(Extension(tx): Extension<StateTx>) -> impl IntoResponse {
        let (resp_tx, resp_rx) = oneshot::channel();
        let queue_status = dispatch(tx, resp_rx, StateCommand::GetQueue(resp_tx))
            .await
            .unwrap();
        Json(queue_status)
    }

    pub async fn toggle_queue(
        Extension(tx): Extension<StateTx>,
    ) -> impl IntoResponse {
        let (resp_tx, resp_rx) = oneshot::channel();
        let queue_status = dispatch(tx, resp_rx, StateCommand::ToggleQueue(resp_tx))
            .await
            .unwrap();
        Json(queue_status)
    }

    pub async fn pop_queue(
        args: Option<Query<NextQueryArg>>,
        Extension(tx): Extension<StateTx>,
    ) -> impl IntoResponse {
        let Query(args) = args.unwrap_or_default();
        let (resp_tx, resp_rx) = oneshot::channel();
        tracing::debug!("Popping: {}", args.count);
        let popped_entries = dispatch(
            tx,
            resp_rx,
            StateCommand::PopQueue {
                count: args.count,
                tx: resp_tx,
            },
        )
        .await
        .unwrap();
        Json(popped_entries)
    }

    pub async fn handle_error(_err: io::Error) -> impl IntoResponse {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
    }
}
