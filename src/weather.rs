use dotenv::dotenv;
use reqwest;
use serde::Deserialize;

use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Weather {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    humidity: f64,
}

#[derive(Deserialize, Debug)]
pub struct WeatherDescription {
    main: String,
    description: String,
}

#[derive(Deserialize, Debug)]
pub struct Rain {
    one_hour: f64,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    speed: f64,
}
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    weather: Vec<WeatherDescription>,
    main: Weather,
    wind: Wind,
    rain: Option<Rain>,
}
pub async fn get_weather_for_location(
    lat: f64,
    long: f64,
) -> Result<WeatherResponse, Box<dyn Error>> {
    dotenv().ok();
    let api_key =
        std::env::var("OPEN_WEATHER_API_KEY").expect("Expected an API key in the environment");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={long}&appid={api_key}&units=metric",
        lat = lat.to_string(),
        long = long.to_string(),
        api_key = api_key
    );
    let response = reqwest::get(&url).await.expect("Failed to send request");
    if let Err(why) = response.error_for_status_ref() {
        Err(why)?
    } else {
        let weather: WeatherResponse = response.json().await.expect("Failed to parse response");

        return Ok(weather);
    }
}
