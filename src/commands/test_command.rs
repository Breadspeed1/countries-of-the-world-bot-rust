use std::error::Error;
use std::sync::Arc;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::async_trait;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use crate::commands::{CommandHandler, CommandHandlerError};
use crate::database::data_types::User;
use crate::Database;

pub struct TestCommandHandler {
    database: Option<Arc<Database>>
}

impl TestCommandHandler {
    pub fn temp() -> TestCommandHandler {
        TestCommandHandler {
            database: None
        }
    }

    fn get_database(&self) -> Result<Arc<Database>, Box<dyn Error + Send + Sync>> {
        match &self.database {
            Some(x) => Ok(Arc::clone(x)),
            None => Err(Box::new(CommandHandlerError::new()))
        }
    }
}

#[async_trait]
impl CommandHandler for TestCommandHandler {
    fn from(&self, db: Arc<Database>) -> Box<dyn CommandHandler + Send + Sync> {
        Box::new(TestCommandHandler {
            database: Some(db)
        })
    }

    async fn handle(&mut self, interaction: &ApplicationCommandInteraction, ctx: &Context, db: Arc<Database>) -> Result<(), Box<dyn Error>> {
        self.database = Some(db);

        let mut m: String = String::from("_");
        let users: Vec<User> = self.get_database().unwrap().get_users().await?;

        for i in 0..users.len() {
            m.push_str(format!("id: {}\nmoney: {}\ndistance traveled: {}", users[i].user_id, users[i].money, users[i].distance_traveled).as_str());
        }

        interaction.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(m))
        }).await?;

        Ok(())
    }

    fn get_command_description(&self) -> String {
        String::from("test the bot 2")
    }

    fn get_command_name(&self) -> String {
        String::from("test")
    }
}