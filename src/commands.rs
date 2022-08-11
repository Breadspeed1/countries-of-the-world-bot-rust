use std::env;
use std::error::Error;
use serenity::client::{Context, EventHandler};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId};
use serenity::model::prelude::interaction::Interaction;

mod ping_command;

async fn build_commands(ctx: &Context) -> Result<(), Box<dyn Error>> {
    let guild = GuildId(env::var("GUILD_ID").expect("Error getting guild id from environment").parse::<u64>().expect("invalid guild id in environment"));
    println!("building commands...");

    ping_command::set_command(&guild, &ctx).await?;

    println!("Finished setting up commands");

    Ok(())
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Ready with id: {}", ready.session_id);
        build_commands(&ctx).await.expect("Error building commands");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let name = command.data.name.as_str();
            match name {
                "ping" => ping_command::handle(&ctx, &command).await.expect("Error handling ping command"),
                _ => println!("{} command not implemented", name)
            }
        }
    }
}