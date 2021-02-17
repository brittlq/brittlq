use irc::client::prelude::*;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use warp::{http::StatusCode, Filter};

mod utils;
use utils::{get_user_config, pop, remove};

#[derive(Serialize)]
struct ToggleResponse {
    is_open: bool,
}

#[derive(Debug, Deserialize)]
struct Num {
    num: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct Token {
    access_token: String,
    scope: String,
    token_type: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new() /*.with_level(log::LevelFilter::Debug)*/
        .init()
        .unwrap();

    let mut bot = utils::bot::build_bot();

    // Clone the Arc to get a new pointer which will be moved into the handlers
    let bot_queue = bot.queue.clone();
    let open = warp::path("status").map(move || warp::reply::json(&*bot_queue.lock().unwrap()));

    let bot_queue = bot.queue.clone();
    let queue =
        warp::path("queue").map(move || warp::reply::json(&bot_queue.lock().unwrap().queue));

    let bot_queue = bot.queue.clone();
    let remove =
        warp::path("remove")
            .and(warp::path::param())
            .map(
                move |name: String| match remove(&name, &mut bot_queue.lock().unwrap().queue) {
                    Some(_) => warp::reply::with_status("", StatusCode::OK),
                    None => warp::reply::with_status("", StatusCode::NOT_FOUND),
                },
            );

    let bot_queue = bot.queue.clone();
    let next = warp::path("next")
        .and(warp::query::<Num>())
        .map(move |x: Num| warp::reply::json(&pop(x.num, &mut bot_queue.lock().unwrap().queue)));
    // TODO there's a better mechanism out there than this. We'll find it one day
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let token = warp::path("token")
        .and(warp::body::json())
        .map(move |x: Token| warp::reply::json(&tx.send(x.access_token).unwrap()));

    let bot_queue = bot.queue.clone();
    let posts = warp::post().and(warp::path("toggle").map(move || {
        let mut lock = bot_queue.lock().unwrap();
        lock.is_open = !lock.is_open;
        let j = ToggleResponse {
            is_open: lock.is_open,
        };
        warp::reply::json(&j)
    }));

    let routes = warp::get()
        .and(open)
        .or(queue)
        .or(remove)
        .or(next)
        .or(token)
        .or(posts)
        .or(warp::fs::dir("./www/dist/"));
    tokio::spawn(async move {
        let server = warp::serve(routes);
        server.run(([127, 0, 0, 1], 8080)).await;
    });

    let token = format!("oauth:{}", rx.recv().await.unwrap());
    bot.run(get_user_config(&token)).await
}
