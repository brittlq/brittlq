struct QueueRow {
    id: i32,
    locked: bool,
    name: Option<String>,
    owner: String,
    content: Option<serde_json::Value>,
}

pub mod api {
    use super::QueueRow;
    use sqlx::PgPool;
    use warp::Filter;

    pub async fn get_by_name(
        owner: &str,
        name: &str,
        sql_pool: PgPool,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let Ok(queue) = sqlx::query_as!(
            QueueRow,
            "SELECT * FROM queue WHERE owner = $1 AND name = $2",
            owner,
            name
        )
        .fetch_one(&sql_pool)
        .await
        {
            Ok(warp::reply::json(&queue.content))
        } else {
            Err(warp::reject::not_found())
        }
    }

    pub async fn get_by_id(id: i32, sql_pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
        if let Ok(queue) = sqlx::query_as!(QueueRow, "SELECT * FROM queue WHERE id = $1", id)
            .fetch_one(&sql_pool)
            .await
        {
            Ok(warp::reply::json(&queue.content))
        } else {
            Err(warp::reject::not_found())
        }
    }

    pub async fn get_by_user(
        owner: String,
        sql_pool: PgPool,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if let Ok(queue) = sqlx::query_as!(QueueRow, "SELECT * FROM queue WHERE owner = $1", &owner)
            .fetch_one(&sql_pool)
            .await
        {
            Ok(warp::reply::json(&queue.content))
        } else {
            Err(warp::reject())
        }
    }
    #[derive(Debug)]
    struct UnknownError;
    impl warp::reject::Reject for UnknownError {}

    pub async fn post_new_queue(
        owner: String,
        sql_pool: PgPool,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        match sqlx::query!(
            "INSERT INTO queue (owner, locked) VALUES ($1, true)",
            &owner
        )
        .execute(&sql_pool)
        .await
        {
            Ok(_) => Ok(warp::reply()),
            Err(_) => Err(warp::reject::custom(UnknownError)), // TODO inspect the error and handle cases where the queue already exists
        }
    }

    mod routes {
        use sqlx::PgPool;
        use warp::Filter;

        use super::{get_by_user, post_new_queue};

        pub(crate) fn get_queues(
            sql_pool: PgPool,
        ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            warp::path!("queues" / String)
                .and(warp::get())
                .and(crate::server::with_pool(sql_pool))
                .and_then(get_by_user)
        }

        pub(crate) fn post_queues(
            sql_pool: PgPool,
        ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
            warp::path!("queues" / String)
                .and(warp::post())
                .and(crate::server::with_pool(sql_pool))
                .and_then(post_new_queue)
        }
    }

    pub fn routes(
        sql_pool: PgPool,
    ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        routes::get_queues(sql_pool.clone()).or(routes::post_queues(sql_pool))
    }
}
