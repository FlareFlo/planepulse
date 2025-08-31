use crate::adsb_provider::AdsbProvider;
use crate::adsb_provider::adsbfi::AdsbFi;
use crate::config::{Config, Notification, NotificationConfig, Site};
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
    let prov = AdsbFi::new(&config);
    let w = OpenWeathermap::new(&config);

    main_loop(prov, w, config.notification, config.site).await;
}

async fn main_loop(
    mut provider: impl AdsbProvider,
    mut _weathermap: impl WeatherProvider,
    mut notification: NotificationConfig,
    site: Site,
) {
    loop {
        let aircraft = provider.get_nearby().await;
        let total_count = aircraft.len();
        let candidates = aircraft
            .into_iter()
            .filter(|candidate| {
                candidate.is_candidate(
                    site.location(),
                    Distance::from_meters(5000.0),
                    Distance::from_meters(1524.0),
                )
            })
            .collect::<Vec<_>>();
        info!(
            "Got {} aircraft, {} of which are candidates",
            total_count,
            candidates.len()
        );
        for candidate in candidates {
            notification.notify(&candidate).await;
            warn!("Posted {}", candidate.hex);
        }
    }
}
