// https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a

mod controllers;
mod db;
mod models;
mod queries;
mod routes;
mod schema;
mod utils;

#[macro_use]
extern crate lazy_static;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
//#[tokio::main(flavor = "current_thread")]
async fn main() {
    let routes = routes::get_routes();
    print!("Preparing server to listen on port 3030");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
