use std::env;
use std::error::Error;
use std::sync::{Arc};
use mysql::{params, Pool, PooledConn};
use mysql::prelude::Queryable;
use crate::data_types::{Territory, User};

pub async fn get_conn_string() -> Result<String, Box<dyn Error>> {
    Ok(env::var("MYSQL_CONNECTION_STRING")?)
}

pub struct Database {
    connection_factory: Arc<SqlConnectionFactory>
}

impl Database {
    pub async fn new(connection_string: String) -> Result<Database, Box<dyn Error>> {
        Ok(Database {
            connection_factory: Arc::new(SqlConnectionFactory::new(connection_string.as_str()).await?)
        })
    }

    pub async fn get_conn(&self) -> Result<PooledConn, Box<dyn Error>> {
        Ok(self.connection_factory.get_connection().await?)
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

    pub async fn get_users_from_id(&self, id: u64) -> Result<Vec<User>, Box<dyn Error>> {
        let mut conn = self.get_conn().await?;
        let statement = conn.prep("SELECT * FROM users WHERE id = :id")?;

        Ok(conn.exec_map(
            &statement,
            params! {"id" => id},
            |(user_id, money, distance_traveled)| { User { user_id, money, distance_traveled } }
        )?)
    }

    pub async fn set_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        let mut conn = self.get_conn().await?;
        let statement = conn.prep(
            "UPDATE users SET (user_id, money, distance_traveled) VALUES(:user_id, :money, :distance_traveled) WHERE user_id = (user_id) VALUES(:user_id)"
        )?;

        conn.exec_drop(
            &statement,
            params! {"user_id" => user.user_id, "money" => user.money, "distance_traveled" => user.distance_traveled }
        )?;

        Ok(())
    }

    pub async fn get_territories(&self) -> Result<Vec<Territory>, Box<dyn Error>> {
        let mut conn = self.get_conn().await?;

        let territories = conn.
            query_map(
                "SELECT * FROM territories",
                |(territory_id, owner_id, color, name)| {
                    Territory { territory_id, owner_id, color, name  }
                }
            )?;

        Ok(territories)
    }

    pub async fn get_territory_from_id(&self, id: u64) -> Result<Vec<Territory>, Box<dyn Error>> {
        let mut conn = self.get_conn().await?;
        let statement = conn.prep("SELECT * FROM users WHERE id = :id")?;

        Ok(conn.exec_map(
            &statement,
            params! {"id" => id},
            |(territory_id, owner_id, color, name)| { Territory { territory_id, owner_id, color, name  } }
        )?)
    }

    pub async fn set_territory(&self, territory: Territory) -> Result<(), Box<dyn Error>> {
        let mut conn = self.get_conn().await?;
        let statement = conn.prep(
            "UPDATE territories SET (territory_id, owner_id, color, name) VALUES(:territory_id, :owner_id, :color, :name) WHERE territory_id = (territory_id) VALUES(:user_id)"
        )?;

        conn.exec_drop(
            &statement,
            params! {"user_id" => user.user_id, "money" => user.money, "distance_traveled" => user.distance_traveled }
        )?;

        Ok(())
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            connection_factory: self.connection_factory.clone()
        }
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