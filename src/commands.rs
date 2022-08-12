use std::env;
use std::error::Error;
use mysql::PooledConn;
use serenity::client::{Context, EventHandler};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::Interaction;
use crate::commands::ping_command::PingCommandHandler;
use crate::database::Database;

mod ping_command;

pub async fn get_commands() -> Result<Vec<Box<dyn CommandHandler>>, Box<dyn Error>> {

    Ok(vec![

    ])
}

async fn build_commands(ctx: &Context) -> Result<(), Box<dyn Error>> {
    let guild = GuildId(env::var("GUILD_ID").expect("Error getting guild id from environment").parse::<u64>().expect("invalid guild id in environment"));
    println!("building commands...");

    //ping_command::set_command(&guild, &ctx).await?;

    println!("Finished setting up commands");

    Ok(())
}

pub struct Handler {
    pub database: Database
}

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
                //"ping" => ping_command::handle(&ctx, &command).await.expect("Error handling ping command"),
                _ => println!("{} command not implemented", name)
            }
        }
    }
}

#[async_trait]
pub trait CommandHandler {
    async fn handle(&self, interaction: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Box<dyn Error>>;
    fn get_command_description() -> String;
    fn get_command_name() -> String;
    async fn set_command(&self, guild: &GuildId, ctx: &Context) -> Result<(), Box<dyn Error>> {
        guild.set_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name(Self::get_command_name()).description(Self::get_command_description())
            })
        }).await?;

        Ok(())
    }
}