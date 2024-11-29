use crate::geocoding::get_geolocation_from_city;
use crate::weather::get_current_weather_for_location;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
pub async fn run(options: &[ResolvedOption<'_>]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(city),
        ..
    }) = options.first()
    {
        let geolocation = get_geolocation_from_city(city.to_string()).await;
        if let Err(why) = geolocation {
            return format!("Failed to get geolocation for city: {why}");
        }
        let geolocation = geolocation.unwrap();
        let current_weather = get_current_weather_for_location(geolocation.0, geolocation.1).await;
        if let Err(why) = current_weather {
            return format!("Failed to get weather for city: {why}");
        }

        let current_weather = current_weather.unwrap();
        return format!("The weather for {} is {:?}", city, current_weather);
    } else {
        return "City does not exist".to_string();
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("current-weather")
        .description("Get the current weather for a location")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "city",
                "The city to get the weather for",
            )
            .description("The city to get the weather for")
            .required(true),
        )
}
