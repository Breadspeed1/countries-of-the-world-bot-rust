use std::error::Error;
use std::sync::Arc;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::async_trait;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use crate::commands::CommandHandler;
use crate::Database;

pub struct PingCommandHandler {
}

impl PingCommandHandler {
    pub fn temp() -> PingCommandHandler {
        PingCommandHandler {
        }
    }
}

#[async_trait]
impl CommandHandler for PingCommandHandler {
    fn from(&self, db: Arc<Database>) -> Box<dyn CommandHandler + Send + Sync> {
        Box::new(PingCommandHandler {})
    }

    async fn handle(&mut self, interaction: &ApplicationCommandInteraction, ctx: &Context, db: Arc<Database>) -> Result<(), Box<dyn Error>> {
        interaction.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("hoe"))
        }).await?;

        Ok(())
    }

    fn get_command_description(&self) -> String {
        String::from("test the bot")
    }

    fn get_command_name(&self) -> String {
        String::from("ping")
    }
}