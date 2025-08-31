use crate::adsb_provider::AdsbAircraft;
use geoutils::{Distance, Location};
use serenity::all::Webhook;
use serenity::builder::{Builder, ExecuteWebhook};
use serenity::http::Http;
use std::fs;
use std::path::Path;

impl Config {
    pub fn from_path(p: impl AsRef<Path>) -> Self {
        toml::from_str(&fs::read_to_string(p).unwrap()).unwrap()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub site: Site,
    pub notification: NotificationConfig,
    pub weather: Weather,
    pub adsb: Adsb,
    pub conditions: Conditions,
}

#[derive(Debug, serde::Deserialize)]
pub struct Site {
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
}

impl Site {
    pub fn location(&self) -> Location {
        Location::new(self.lat, self.lon)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NotificationConfig {
    pub discord_webhook: Option<String>,
    #[serde(skip)]
    serenity_http: Option<Http>,
}

impl NotificationConfig {
    pub async fn notify(&mut self, ac: &AdsbAircraft) {
        if let Some(discord_webhook) = &self.discord_webhook {
            let webhook = Webhook::from_url(
                self.serenity_http
                    .get_or_insert(Http::new(discord_webhook.rsplit_once("/").unwrap().1)),
                discord_webhook,
            )
            .await
            .unwrap();
            let embed = ExecuteWebhook::new().content(format!(
                "{} is at {:.0} meters https://globe.adsbexchange.com/?icao={}",
                ac.desc,
                ac.altitude.meters(),
                ac.hex
            ));
            webhook
                .execute(
                    self.serenity_http.as_ref().expect("infallible"),
                    false,
                    embed,
                )
                .await
                .unwrap();
        }
    }
}

pub struct Notification {
    pub distance: Distance,
    pub location: Location,
    pub display_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weather {
    pub openweathermap_api_key: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Adsb {
    pub adsbfi: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct Conditions {
    /// Meters
    max_distance: f64,
    /// Meters
    max_altitude: f64,
}

impl Conditions {
    pub fn max_distance(&self) -> Distance {
        Distance::from_meters(self.max_distance)
    }

    pub fn max_altitude(&self) -> Distance {
        Distance::from_meters(self.max_altitude)
    }
}
