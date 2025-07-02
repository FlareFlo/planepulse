use crate::config::Config;

pub mod openweathermap;

pub trait WeatherProvider {
	async fn get_weather(&mut self) -> Weather;
	
	fn new(c: &Config) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Weather {
	/// In meters, max is 10km
	visibility: f64,
	/// Percentage of coverage
	clouds: f64,
}