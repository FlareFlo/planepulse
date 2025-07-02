use std::fs;
use std::path::Path;
use geoutils::Location;

impl Config {
	pub fn from_path(p: impl AsRef<Path>) -> Self {
		toml::from_str(&fs::read_to_string(p).unwrap()).unwrap()
	}
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
	pub site: Site,
	pub notification: Notification,
	pub weather: Weather,
	pub adsb: Adsb,
}

#[derive(Debug, serde::Deserialize)]
pub struct Site {
	lat: f64,
	lon: f64,
	pub elevation: f64,
}

impl Site {
	pub fn location(&self) -> Location {
		Location::new(self.lat, self.lon)
	}
}

#[derive(Debug, serde::Deserialize)]
pub struct Notification {
	pub discord_webhook: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weather {
	pub openweathermap_api_key: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Adsb {
	pub adsbfi: bool
}