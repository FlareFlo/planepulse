use crate::adsb_provider::AdsbProvider;
use crate::adsb_provider::adsbfi::AdsbFi;
use crate::config::{Conditions, Config, Notification, NotificationConfig, Site};
use crate::weather_provider::WeatherProvider;
use crate::weather_provider::openweathermap::OpenWeathermap;
use geoutils::Distance;
use std::env;
use std::process::exit;
use tracing::error;
use tracing::info;
use tracing::warn;
use tracing_subscriber::FmtSubscriber;

mod adsb_provider;
mod config;
mod weather_provider;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::builder().finish())
        .expect("tracing setup failed");

    ctrlc::set_handler(move || {
        error!("Got shutdown signal");
        exit(1);
    })
    .unwrap();

    let config = Config::from_path("configs/ohw.toml");
    let provider = AdsbFi::new(&config);
    let weather = OpenWeathermap::new(&config);

    main_loop(config, provider, weather).await;
}

async fn main_loop(
    Config {site, conditions, mut notification, ..}: Config,
    mut provider: impl AdsbProvider,
    _weathermap: impl WeatherProvider,
) {
    loop {
        let aircraft = provider.get_nearby(&conditions, &site.location()).await;
        info!(
            "Posting {} aircraft",
            aircraft.len()
        );
        for candidate in aircraft {
            notification.notify(&candidate).await;
            warn!("Posted {}", candidate.hex);
        }
    }
}
