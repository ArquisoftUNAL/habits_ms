// https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a

use std::convert::Infallible;
use warp::{Filter, Rejection};

mod models;
mod db;
// mod error;
mod schema;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
