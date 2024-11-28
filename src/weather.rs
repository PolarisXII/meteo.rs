use dotenv::dotenv;
use reqwest;

use std::error::Error;

pub async fn get_weather_for_location(lat: f64, long: f64) -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key =
        std::env::var("OPEN_WEATHER_API_KEY").expect("Expected an API key in the environment");
    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={lat}&lon={long}&appid={api_key}",
        lat = lat.to_string(),
        long = long.to_string(),
        api_key = api_key
    );
    let response = reqwest::get(&url).await.expect("Failed to send request");
    if let Err(why) = response.error_for_status_ref() {
        Err(why)?;
    } else {
        println!("{:?}", response.text().await?);
    }

    Ok(())
}
