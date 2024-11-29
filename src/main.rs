use dotenv::dotenv;
use serenity::all::{
    Command, CreateInteractionResponse, CreateInteractionResponseMessage, Interaction,
};
use serenity::async_trait;

use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
mod commands;
mod geocoding;
mod weather;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Interaction::Command(interaction) = interaction {
            println!("Received command interaction: {interaction:#?}",);
            let content = match interaction.data.name.as_str() {
                "ping" => commands::ping::run(&interaction.data.options()),
                "current-weather" => {
                    commands::current_weather::run(&interaction.data.options()).await
                }
                _ => "Unknown command".to_string(),
            };

            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = interaction.create_response(&context.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, context: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = vec![
            commands::ping::register(),
            commands::current_weather::register(),
        ];

        let global_commands = Command::set_global_commands(&context.http, commands)
            .await
            .expect("Failed to register global commands");
        println!("Global commands registered: {global_commands:#?}");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("METEORS_BOT_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // weather api debugging
    let location_res = geocoding::get_geolocation_from_city("bondi junction".to_string()).await;
    let location = location_res.expect("Failed to get location from city");
    let weather_response = weather::get_current_weather_for_location(location.0, location.1)
        .await
        .expect("Failed to get weather for location");
    println!("{:?}", weather_response);

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
