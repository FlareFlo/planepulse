use crate::config::Config;
use crate::weather_provider::{Weather, WeatherProvider};
use geoutils::Location;
use reqwest::Client;

pub struct OpenWeathermap {
    key: String,
    location: Location,
    client: Client,
}

impl WeatherProvider for OpenWeathermap {
    async fn get_weather(&mut self) -> Weather {
        let req = self.client.get(format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={key}",
            key = self.key,
            lat = self.location.latitude(),
            lon = self.location.longitude(),
        ));
        let res = req.send().await.unwrap();
        let root: serde_json::Value = res.json().await.unwrap();

        Weather {
            visibility: root["visibility"].as_f64().unwrap(),
            clouds: root["clouds"]["all"].as_f64().unwrap(),
        }
    }

    fn new(c: &Config) -> Self {
        Self {
            key: c.weather.openweathermap_api_key.clone().unwrap(),
            location: c.site.location(),
            client: Default::default(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Root {
    #[serde(rename = "clouds.all")]
    clouds: f64,
    visibility: f64,
}
