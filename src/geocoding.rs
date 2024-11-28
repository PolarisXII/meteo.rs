use dotenv::dotenv;
use reqwest;
use serde::Deserialize;
use std::{error::Error, str};

#[derive(Deserialize, Debug)]
struct Geolocation {
    lat: f64,
    lon: f64,
}
pub async fn get_geolocation_from_city(city: String) -> Result<(f64, f64), Box<dyn Error>> {
    dotenv().ok();
    let api_key =
        std::env::var("OPEN_WEATHER_API_KEY").expect("Expected an API key in the environment");
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={city}&limit=1&appid={api_key}",
        city = city,
        api_key = api_key
    );

    let response = reqwest::get(&url).await.expect("Failed to send request");
    if let Err(why) = response.error_for_status_ref() {
        Err(why)?;
    }

    let geolocation: Vec<Geolocation> = response.json().await.expect("Failed to parse response");
    println!("{:?}", geolocation);
    return Ok((geolocation[0].lat, geolocation[0].lon));
}
