mod data_types;

use std::env;
use std::error::Error;
use mysql::{Pool, PooledConn};

pub async fn get_conn_string() -> Result<String, Box<dyn Error>> {
    Ok(env::var("MYSQL_CONNECTION_STRING")?)
}

/*pub async fn get_territories() -> Result<>*/

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