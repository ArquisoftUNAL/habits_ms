use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::fmt;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let DATABASE_URL : String = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("DATABASE_USER").unwrap(),
        env::var("DATABASE_PASSWORD").unwrap(),
        env::var("DATABASE_HOST").unwrap(),
        env::var("DATABASE_PORT").unwrap(),
        env::var("DATABASE_NAME").unwrap()
    );
    PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL))
}