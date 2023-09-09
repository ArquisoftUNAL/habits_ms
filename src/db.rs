use crate::error::Error;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;
use std::time::Duration;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct DBManager {
    pub connection: PostgresPool,
}

pub fn create_pool() -> Result<PostgresPool, Error> {
    println!("Connecting to database");
    dotenv().ok();

    let database_url: String = format!("{}", env::var("DATABASE_URL").unwrap());

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(15)
        .connection_timeout(Duration::from_secs(10))
        // .error_handler(Box::new(|err: R2D2Error, conn| {
        //     println!("Error on connection: {}", err);
        //     // For other errors, return an error
        //     Err(Error::DBConnectionError(err))
        // }))
        .build(manager)
        .or_else(|err| Err(Error::DBConnectionError(err)));

    if pool.is_err() {
        return Err(pool.err().unwrap());
    }

    println!("Connected to database");

    Ok(pool.unwrap())
}

impl DBManager {
    pub fn new(connection: PostgresPool) -> DBManager {
        DBManager {
            connection: connection,
        }
    }
}
