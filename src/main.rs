use chrono::prelude::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[macro_use]
extern crate dotenv_codegen;

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i64,
    sunset: i64,
}

impl Forecast {
    async fn get(
        city: &String,
        country_code: &String,
        api_key: &String,
    ) -> Result<Self, ExitFailure> {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
            city, country_code, api_key
        );
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<Forecast>().await?;
        Ok(resp)
    }
}

async fn get_location_details_from_ip(option: String) -> Result<String, ExitFailure> {
    let url = format!("http://ipinfo.io/{}", option);
    let url = Url::parse(&*url)?;
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    Ok(body)
}

fn miles_per_second_to_kmh(inputspeed: f64) -> f64 {
    inputspeed * 3.6
}

fn degrees_to_compass(deg: i32) -> &'static str {
    match deg {
        00..=21 => return "North",
        22..=43 => return "North Northeast",
        44..=45 => return "North East",
        46..=66 => return "East Northeast",
        67..=111 => return "East",
        112..=133 => return "East Southeast",
        134..=135 => return "Southeast",
        136..=156 => return "South Southeast",
        157..=201 => return "South",
        202..=223 => return "South Southwest",
        224..=225 => return "Southwest",
        226..=246 => return "West Southwest",
        247..=291 => return "West",
        292..=313 => return "West Northwest",
        314..=315 => return "Northwest",
        316..=336 => return "North Northewest",
        337..=360 => return "North",
        _ => return "Error getting wind direction",
    }
}

fn kelvin_to_celcius(kel: f64) -> f64 {
    kel - 273.15
}

fn convert_timestamp_to_utc_date(timestamp: i64) -> chrono::DateTime<chrono::Utc> {
    let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime_again: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    datetime_again
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let country = get_location_details_from_ip(String::from("country")).await?;
    let city = get_location_details_from_ip(String::from("city")).await?;
    let api_key = dotenv!("API_KEY");
    let response = Forecast::get(
        &String::from(city),
        &String::from(country),
        &String::from(api_key),
    )
    .await?;
    println!(
        "City: {}\nCountry Code: {}\nHumidity: {}%\nWeather Description: {}\nWind Speed: {:.0}kmh\nWind Direction: {}\nTemperature: {:.0}°C\nMinimum Temperature: {:.0}°C\nMaximum Temperature: {:.0}°C\nFeels Like: {:.0}°C\nSunrise: {}\nSunset: {}",
        response.name,
        response.sys.country,
        response.main.humidity,
        response.weather.details.description,
        miles_per_second_to_kmh(response.wind.speed),
        degrees_to_compass(response.wind.deg),
        kelvin_to_celcius(response.main.temp),
        kelvin_to_celcius(response.main.temp_min),
        kelvin_to_celcius(response.main.temp_max),
        kelvin_to_celcius(response.main.feels_like),
        convert_timestamp_to_utc_date(response.sys.sunrise),
        convert_timestamp_to_utc_date(response.sys.sunset)
    );
    Ok(())
}
