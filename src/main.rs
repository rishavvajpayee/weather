use std::io;
use std::io::Write;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    println!("Weather API Find Weather Forcast");
    print!("Enter the place you want to find the Weather of : ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed");

    let mut headers = reqwest::header::HeaderMap::new();
    let rapid_api_key = env::var("RAPID_API_KEY").expect("ERROR : in reading key");
    let rapid_api_host = env::var("RAPID_API_HOST").expect("ERROR : in reading host");
    headers.insert("X-RapidAPI-Key", rapid_api_key.parse().expect("incorrect format for key"));
    headers.insert("X-RapidAPI-Host",rapid_api_host.parse().expect("incorrect format for host"));

    let data = "";

    let client = reqwest::Client::new();
    let url = format!("https://weatherapi-com.p.rapidapi.com/forecast.json?q={}&days=1", input.trim());
    let request = client.get(&url).headers(headers).body(data);

    let response = request.send().await?;
    let body = response.json::<serde_json::Value>().await?;
    println!("Current Temperature in {} - {} C", input, body["current"]["temp_c"]);
    Ok(())
}

