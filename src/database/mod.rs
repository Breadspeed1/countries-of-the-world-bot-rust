mod data_types;

use std::env;
use std::error::Error;
use mysql::{Pool, PooledConn};
use crate::database::data_types::Territory;

pub struct Database {
    connection_factory: SqlConnectionFactory
}

impl Database {
    pub async fn new(connection_string: String) -> Result<Database, Box<dyn Error>> {
        Ok(Database {
            connection_factory: SqlConnectionFactory::new(connection_string.as_str()).await?
        })
    }

    pub async fn get_territories(&self) -> Result<Vec<Territory>, Box<dyn Error>> {
        let conn: PooledConn = self.connection_factory.get_connection().await?;



        Ok(vec![])
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