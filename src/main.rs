use serde::Deserialize;
use std::env;
use std::io;
#[tokio::main]
async fn main() {
    //let api_key = "8abf1f4db6bd2304d1d4c56a437814b8";
    let weath = get_weather(&init()).await;
    println!("Temperature: {}°C", weath.current.temperature);
    println!("Weather: {}", weath.current.weather_descriptions[0]);
    println!("Cloudcover: {}%", weath.current.cloudcover);

 

   /*  let response = reqwest::get(&url).await.unwrap().json::<serde_json::Value>().await.unwrap();
    let temperature = response["current"]["temperature"].as_f64().unwrap();
    let description = response["current"]["weather_descriptions"][0].as_str().unwrap();
    println!("The temperature in {} is {}°C and the weather is {}", city, temperature, description); */

}

// init function to get the weather
fn init() -> String {
    println!("Enter the city name: ");
    let mut buffer = String::new();
    println!("");
    
    io::stdin().read_line(&mut buffer).unwrap();
    
    let api_key = std::env::var("API_KEY").expect("API_KEY not found");

    let city = buffer.trim().to_string();
    let url = format!("http://api.weatherstack.com/current?access_key={}&query={}", api_key, &city);

    url
}
async fn get_weather(url: &str) -> Weather {


    let response = reqwest::get(url).await.unwrap().json::<Weather>().await.unwrap();
/*     let temperature = response.current.temperature;
    let description = &response.current.weather_descriptions[0];
    let cloudcover = response.current.cloudcover; */
    
    return response;
}


#[derive(Deserialize)]
struct Weather {
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature: f64,
    weather_descriptions: Vec<String>,
    cloudcover: f64,

}