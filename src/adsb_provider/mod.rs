pub mod adsbfi;

use crate::config::{Conditions, Config};
use geoutils::{Distance, Location};

#[derive(Debug)]
pub struct AdsbAircraft {
    pub altitude: Distance,
    pub location: Location,
    pub desc: String,
    pub flight: String,
    pub hex: String,
}

impl AdsbAircraft {
    /// Checks if aircraft can/should be posted
    pub fn is_candidate(
        &self,
        conditions: &Conditions,
        location: &Location,
    ) -> bool {
        let d = location.haversine_distance_to(&self.location);
        if d.meters() > conditions.max_distance().meters() {
            return false;
        }

        if self.altitude.meters() > conditions.max_altitude().meters() {
            return false;
        }

        true
    }
}

pub trait AdsbProvider {
    async fn get_nearby(&mut self, conditions: &Conditions, location: &Location) -> Vec<AdsbAircraft>;

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
            Squawk::None => 0,
            Squawk::Hijacking => 7500,
            Squawk::RadioFailure => 7600,
            Squawk::Emergency => 7700,
            Squawk::Normal(i) => i,
        }
    }
}
