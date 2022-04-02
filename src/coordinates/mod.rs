//! Compare, calculate, and transform spacial coordinates

use measurements::{Angle, Distance};

/// Spherical coordinates expressed as (right ascension, declination)
#[derive(Debug)]
pub struct EquatorialCoord {
    /// The right ascension angle
    pub ra: Angle,
    /// The declination angle
    pub dec: Angle,
}

impl Default for EquatorialCoord {
    fn default() -> Self {
        Self {
            ra: Angle::from_radians(0.0),
            dec: Angle::from_radians(0.0),
        }
    }
}

/// Coordinates that represent a location on Earth.
#[derive(Debug)]
pub struct EarthLocation {
    /// The latitude coordinate
    pub lat: Angle,
    /// The longitude coordinate
    pub lon: Angle,
    /// The height of the location
    pub height: Distance,
}

impl Default for EarthLocation {
    fn default() -> Self {
        Self {
            lat: Angle::from_radians(0.0),
            lon: Angle::from_radians(0.0),
            height: Distance::from_meters(0.0),
        }
    }
}
