# rusty-weather

Simple weather cli written in [rust](https://www.rust-lang.org/)

## Setup

Make sure you have [rust](https://www.rust-lang.org/) installed on your machine by following the [getting started guide](https://www.rust-lang.org/learn/get-started)

Get an API key from [OpenWeather](https://openweathermap.org/)

## Instructions

* Clone this repository `git clone git@github.com:thomaschaplin/rusty-weather.git`
* Change directory `cd rusty-weather`
* Build the application `cargo build`
* Run the application `cargo run <city> <country_code> <api_key>`

#### Example output:

```
City: London
Country Code: GB
Humidity: 87%
Weather Description: light rain
Wind Speed: 16.56kmh
Wind Direction: South
Temperature: 15.49°C
Minimum Temperature: 15.00°C
Maximum Temperature: 16.11°C
Feels Like: 13.31°C
Sunrise: 2020-09-30 05:59:58 UTC
Sunset: 2020-09-30 17:40:41 UTC
```
