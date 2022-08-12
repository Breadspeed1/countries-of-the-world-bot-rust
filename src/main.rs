mod commands;
mod database;

use std::env;
use std::error::Error;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use crate::database::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("starting...");

    dotenv::dotenv().expect("failed to load environment");
    let token = env::var("DISCORD_TOKEN").expect("could not find token in environment");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(commands::Handler { database: Database::new(env::var("SQL_CONNECTION_STRING")?.into()).await? })
        .await
        .expect("error creating client");

    if let Err(why) = client.start().await {
        println!("error starting client: {:?}", why);
    }

    Ok(())
}
