use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::{env, error::Error};

pub fn establish_connection() -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn Error>> {
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
    let pool = Pool::builder().build(manager)?;

    Ok(pool)
}
