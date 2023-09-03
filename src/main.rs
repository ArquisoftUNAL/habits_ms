// https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a

use std::convert::Infallible;
use warp::{Filter, Rejection};

mod controllers;
mod db;
mod models;
mod routes;
mod schema;
mod utils;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let routes = routes::get_routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
