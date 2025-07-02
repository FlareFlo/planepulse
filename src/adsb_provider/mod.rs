pub mod adsbfi;

use geoutils::Location;
use crate::config::Config;

#[derive(Debug)]
pub struct AdsbAircraft {
	// Meters
	altitude: f64,
	location: Location,
	desc: String,
	flight: String,
	hex: String,
}

pub trait AdsbProvider {
	async fn get_nearby(&mut self) -> Vec<AdsbAircraft>;
	
	fn new(c: &Config) -> Self;
}


#[derive(Debug, Copy, Clone)]
pub enum Squawk {
	None,
	Hijacking,
	RadioFailure,
	Emergency,
	Normal(u16),
}

impl Squawk {
	pub fn from_int(c: u16) -> Self {
		match c { 
			0 => Squawk::None,
			7500 => Squawk::Hijacking,
			7600 => Squawk::RadioFailure,
			7700 => Squawk::Emergency,
			_ => Squawk::Normal(c),
		}
	}
	
	pub fn to_int(self) -> u16 {
		match self {
			Squawk::None => {0}
			Squawk::Hijacking => {7500}
			Squawk::RadioFailure => {7600}
			Squawk::Emergency => {7700}
			Squawk::Normal(i) => {i}
		}
	}
}