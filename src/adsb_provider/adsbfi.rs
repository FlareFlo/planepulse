use crate::adsb_provider::{AdsbAircraft, AdsbProvider};
use crate::config::{Conditions, Config};
use geoutils::{Distance, Location};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::{Instant, sleep_until};

pub struct AdsbFi {
    last_fetch: Instant,
    client: reqwest::Client,
}

impl AdsbProvider for AdsbFi {
    async fn get_nearby(&mut self, conditions: &Conditions, location: &Location) -> Vec<AdsbAircraft> {
        // Cooldown of 30s, we play it safe
        sleep_until(self.last_fetch + Duration::from_secs_f32(35.0)).await;
        self.last_fetch = Instant::now();

        let req = self.client.get("https://opendata.adsb.fi/api/v2/snapshot");
        let res = req.send().await.unwrap();
        let root: ResponseRoot = res.json().await.unwrap();
        root.ac
            .into_iter()
            .filter_map(|r| {
                let lat = r.lat?;
                let lon = r.lon?;
                let alt = r.alt_baro?;
                Some(AdsbAircraft {
                    // Feet to meters
                    altitude: Distance::from_meters(alt / 3.281),
                    location: Location::new(lat, lon),
                    desc: r.desc,
                    flight: r.flight.trim().to_owned(),
                    hex: r.hex.trim().to_owned(),
                })
            })
            .filter(|aircraft|aircraft.is_candidate(conditions, location))
            .collect()
    }

    fn new(_: &Config) -> Self {
        Self {
            // This sucks but eh
            last_fetch: Instant::now() - Duration::from_secs(10000),
            client: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ResponseRoot {
    ac: Vec<ApiResponseAircraft>,
}

#[derive(Debug, Deserialize)]
struct ApiResponseAircraft {
    hex: String,
    #[serde(default)]
    desc: String,
    #[serde(default, deserialize_with = "deser_altitude")]
    alt_baro: Option<f64>,
    #[serde(default)]
    squawk: String,
    lat: Option<f64>,
    lon: Option<f64>,
    #[serde(default)]
    flight: String,
}

fn deser_altitude<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let int = f64::deserialize(deserializer);
    Ok(Some(int.unwrap_or(0.0)))
}
