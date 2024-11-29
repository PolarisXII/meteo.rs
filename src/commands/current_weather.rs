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
        let greeting = format!("Here's the current weather for {}: \n", city);
        let description = format!(
            "The weather is currently: {} - {}\n",
            current_weather.weather[0].main, current_weather.weather[0].description,
        );
        let weather = format!(
            "Temperature: {}°C\nFeels like: {}°C\nMin: {}°C\nMax: {}°C\nHumidity: {}%\n",
            current_weather.main.temp,
            current_weather.main.feels_like,
            current_weather.main.temp_min,
            current_weather.main.temp_max,
            current_weather.main.humidity
        );
        let wind = format!("Wind speed: {}m/s\n", current_weather.wind.speed);
        let rain = if let Some(rain) = &current_weather.rain {
            format!(
                "Rain in the last hour: {}mm\n",
                rain.one_hour.unwrap_or(0.0)
            )
        } else {
            "No rain in the last hour\n".to_string()
        };

        return format!("{}{}{}{}{}", greeting, description, weather, wind, rain);
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
