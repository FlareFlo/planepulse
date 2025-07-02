use std::env;
use crate::adsb_provider::adsbfi::AdsbFi;
use crate::adsb_provider::AdsbProvider;
use crate::config::Config;
use crate::weather_provider::openweathermap::OpenWeathermap;
use crate::weather_provider::WeatherProvider;

mod adsb_provider;
mod weather_provider;
mod config;

#[tokio::main]
async fn main() {
    let config = Config::from_path("configs/ohw.toml");
    //let mut prov = AdsbFi::new(&config);
    //dbg!(&prov.get_nearby().await);
    let mut w = OpenWeathermap::new(&config);
    dbg!(w.get_weather().await);
}
