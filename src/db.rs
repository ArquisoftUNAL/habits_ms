use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::{env, error::Error};

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POSTGRES_POOL: PostgresPool = {
        dotenv().ok();

        let database_url: String = format!(
            "postgres://{}:{}@{}:{}/{}",
            env::var("POSTGRES_USER").unwrap(),
            env::var("POSTGRES_PASSWORD").unwrap(),
            env::var("POSTGRES_HOST").unwrap(),
            env::var("POSTGRES_PORT").unwrap(),
            env::var("POSTGRES_DB").unwrap()
        );

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).unwrap();

        pool
    };
}
