use crate::error::Error;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use std::env;
use std::time::Duration;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct DBManager {
    pub connection_write: Option<PostgresPool>,
    pub connection_read: Option<PostgresPool>,
}

pub fn create_pool_write() -> Result<PostgresPool, Error> {
    println!("Connecting to write database");
    dotenv().ok();

    let database_url_write: String = format!("{}", env::var("DATABASE_URL_WRITE").unwrap());

    // Create write database connections
    let manager_write = ConnectionManager::<PgConnection>::new(database_url_write);

    let pool_write = Pool::builder()
        .max_size(15)
        .connection_timeout(Duration::from_secs(10))
        .build(manager_write)
        .or_else(|err| {
            Err(Error::DBConnectionError2(ConnectionError::BadConnection(
                "No read connection".to_string(),
            )))
        });

    if pool_write.is_err() {
        return Err(pool_write.err().unwrap());
    }

    println!("Connected to WRITE database");

    Ok(pool_write.unwrap())
}

pub fn create_pool_read() -> Result<PostgresPool, Error> {
    println!("Connecting to read database");
    dotenv().ok();

    let database_url_read: String = format!("{}", env::var("DATABASE_URL_READ").unwrap());

    // Create read database connections
    let manager_read = ConnectionManager::<PgConnection>::new(database_url_read);

    let pool_read = Pool::builder()
        .max_size(15)
        .connection_timeout(Duration::from_secs(10))
        .build(manager_read)
        .or_else(|err| {
            Err(Error::DBConnectionError2(ConnectionError::BadConnection(
                "No read connection".to_string(),
            )))
        });

    if pool_read.is_err() {
        return Err(pool_read.err().unwrap());
    }

    println!("Connected to READ database");

    Ok(pool_read.unwrap())
}

impl DBManager {
    pub fn new(
        connection_write: Option<PostgresPool>,
        connection_read: Option<PostgresPool>,
    ) -> DBManager {
        DBManager {
            connection_write,
            connection_read,
        }
    }

    pub fn get_read_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        if self.connection_read.is_none() {
            if self.connection_write.is_none() {
                return Err(Error::DBConnectionError2(ConnectionError::BadConnection(
                    "No read connection".to_string(),
                )));
            }

            // Master is able to read and write, if necessary
            return self.get_write_connection();
        }

        let conn = self.connection_read.clone().unwrap().get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        Ok(conn.unwrap())
    }

    pub fn get_write_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        if self.connection_read.is_none() {
            return Err(Error::DBConnectionError2(ConnectionError::BadConnection(
                "No read connection".to_string(),
            )));
        }

        let conn = self.connection_write.clone().unwrap().get();

        if conn.is_err() {
            return Err(Error::DBConnectionError(conn.err().unwrap()));
        }

        Ok(conn.unwrap())
    }

    // Other approach: Establish connection on each request
    pub fn get_write_connection_establish() -> Result<PgConnection, Error> {
        dotenv().ok();

        let database_url_write: String = format!("{}", env::var("DATABASE_URL_WRITE").unwrap());

        let connection_write = PgConnection::establish(&database_url_write).or_else(|err| {
            Err(Error::DBConnectionError2(ConnectionError::BadConnection(
                "No read connection".to_string(),
            )))
        });

        if connection_write.is_err() {
            return Err(connection_write.err().unwrap());
        }

        Ok(connection_write.unwrap())
    }

    pub fn get_read_connection_establish() -> Result<PgConnection, Error> {
        dotenv().ok();

        let database_url_read: String = format!("{}", env::var("DATABASE_URL_READ").unwrap());

        let connection_read = PgConnection::establish(&database_url_read).or_else(|err| {
            Err(Error::DBConnectionError2(ConnectionError::BadConnection(
                "No read connection".to_string(),
            )))
        });

        if connection_read.is_err() {
            return Err(connection_read.err().unwrap());
        }

        Ok(connection_read.unwrap())
    }
}
