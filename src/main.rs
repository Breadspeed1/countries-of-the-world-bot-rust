mod commands;
mod database;
mod data_types;

use std::env;
use std::error::Error;
use std::sync::Arc;
use mysql::serde_json::error::Category::Data;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use crate::database::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("starting...");

    dotenv::dotenv().expect("failed to load environment");
    let token = env::var("DISCORD_TOKEN").expect("could not find token in environment");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(commands::Handler { database: Arc::new(Database::new(
            database::get_conn_string().await?,
            database::get_history_storage_path().await?
        ).await?) })
        .await
        .expect("error creating client");

    if let Err(why) = client.start().await {
        println!("error starting client: {:?}", why);
    }

    Ok(())
}
