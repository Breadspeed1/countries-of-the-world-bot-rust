use std::env;
use std::error::Error;
use std::future::Future;
use mysql::PooledConn;
use serenity::client::{Context, EventHandler};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::Interaction;
use crate::commands::ping_command::PingCommandHandler;
use crate::commands::test_command::TestCommandHandler;
use crate::database::Database;

mod ping_command;
mod test_command;

pub async fn get_commands() -> Vec<Box<dyn CommandHandler + Send + Sync>> {
    vec![
        Box::new(PingCommandHandler::temp()),
        Box::new(TestCommandHandler::temp())
    ]
}

async fn build_commands(ctx: &Context) {
    let guild = GuildId(env::var("GUILD_ID").expect("Error getting guild id from environment").parse::<u64>().expect("invalid guild id in environment"));
    println!("building commands...");

    for command in get_commands().await {
        println!("building command {}", command.get_command_name());
        command.set_command(&guild, ctx).await;
    }

    println!("Finished setting up commands");
}

pub struct Handler {
    pub database: Database
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Ready with id: {}", ready.session_id);
        build_commands(&ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let name = command.data.name.as_str();
            let commands = get_commands().await;

            for i in 0..commands.len() {
                if commands[i].get_command_name() == name {
                    commands[i].handle(&command, &ctx).await.unwrap();
                }
            }
        }
    }
}

#[async_trait]
pub trait CommandHandler {
    async fn handle(&self, interaction: &ApplicationCommandInteraction, ctx: &Context, db_conn: &PooledConn) -> Result<(), Box<dyn Error>>;
    fn get_command_description(&self) -> String;
    fn get_command_name(&self) -> String;
    async fn set_command(&self, guild: &GuildId, ctx: &Context) {
        guild.set_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name(self.get_command_name()).description(self.get_command_description())
            })
        }).await.unwrap();
    }
}