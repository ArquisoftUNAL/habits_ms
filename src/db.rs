use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POSTGRES_POOL: PostgresPool = {
        println!("Connecting to database");
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

        let pool = Pool::builder().build(manager);

        if pool.is_err() {
            let error = pool.err().unwrap();
            panic!("Error connecting to database: {}", error);
        }

        println!("Connected to database");

        pool.unwrap()
    };
}
