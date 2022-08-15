use std::error::Error;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::async_trait;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use crate::commands::CommandHandler;

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
    async fn handle(&self, interaction: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), Box<dyn Error>> {
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