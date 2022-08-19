use std::env;
use std::error::Error;
use mysql::{Pool, PooledConn};
use mysql::prelude::Queryable;
use crate::data_types::{User};

pub async fn get_conn_string() -> Result<String, Box<dyn Error>> {
    Ok(env::var("MYSQL_CONNECTION_STRING")?)
}

pub struct Database {
    connection_factory: SqlConnectionFactory
}

impl Database {
    pub async fn new(connection_string: String) -> Result<Database, Box<dyn Error>> {
        Ok(Database {
            connection_factory: SqlConnectionFactory::new(connection_string.as_str()).await?
        })
    }

    pub async fn from_conn(conn: &PooledConn) -> Result<Database, Box<dyn Error>> {

        Ok(Database {

        })
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn: PooledConn = self.connection_factory.get_connection().await?;

        let users = conn.
            query_map(
                "SELECT * FROM users",
                |(user_id, money, distance_traveled)| {
                    User { user_id, money, distance_traveled }
                }
            )?;

        Ok(users)
    }
}

struct SqlConnectionFactory {
    pool: Pool
}

impl SqlConnectionFactory {
    async fn new(url: &str) -> Result<SqlConnectionFactory, Box<dyn Error>> {
        Ok(SqlConnectionFactory {
            pool: Pool::new(url)?
        })
    }

    async fn from_env() -> Result<SqlConnectionFactory, Box<dyn Error>> {
        Ok(SqlConnectionFactory {
            pool: Pool::new(get_conn_string().await?.as_str())?
        })
    }

    async fn get_connection(&self) -> Result<PooledConn, Box<dyn Error>> {
        Ok(self.pool.get_conn()?)
    }
}