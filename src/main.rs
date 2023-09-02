// https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a

use std::convert::Infallible;
use warp::{Filter, Rejection};

mod controllers;
mod db;
mod models;
mod routes;
mod schema;

#[tokio::main]
async fn main() {
    let pool = db::establish_connection().expect("Failed to create pool");

    warp::serve(routes::get_routes(pool))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
