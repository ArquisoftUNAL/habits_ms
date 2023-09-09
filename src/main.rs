// https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a

// Disable camelcase warnings, due to postgres enum types being allways in lowercase
#![allow(non_camel_case_types)]

mod db;
mod error;
mod handlers;
mod models;
mod queries;
mod routes;
mod schema;
mod tests;
mod utils;
mod validators;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
//#[tokio::main(flavor = "current_thread")]
async fn main() {
    let pool = db::create_pool();

    if pool.is_err() {
        println!("Error creating pool: {:?}", pool.err());
        return;
    }

    let routes = routes::get_routes(pool.unwrap());
    print!("Preparing server to listen on port 3030");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
