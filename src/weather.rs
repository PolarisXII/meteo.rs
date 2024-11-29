use dotenv::dotenv;
use reqwest;
use serde::Deserialize;

use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub humidity: f64,
}

#[derive(Deserialize, Debug)]
pub struct WeatherDescription {
    pub main: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct Rain {
    pub one_hour: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
}
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<WeatherDescription>,
    pub main: Weather,
    pub wind: Wind,
    pub rain: Option<Rain>,
}
pub async fn get_current_weather_for_location(
    lat: f64,
    long: f64,
) -> Result<WeatherResponse, Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("OPEN_WEATHER_API_KEY")?;
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={long}&appid={api_key}&units=metric",
        lat = lat.to_string(),
        long = long.to_string(),
        api_key = api_key
    );
    let response = reqwest::get(&url).await?;
    if let Err(why) = response.error_for_status_ref() {
        Err(why)?
    } else {
        let weather: WeatherResponse = response.json().await?;

        return Ok(weather);
    }
}
