//! Compare, calculate, and transform spacial coordinates

mod frames;
mod lookup;

use measurements::{Angle, Distance};

pub use frames::*;
pub use lookup::*;

/// Horizontal coordinates expressed as (altitude, azimuth)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HorizontalCoord {
    /// The altitude angle
    pub alt: Angle,
    /// The azimuth angle
    pub az: Angle,
}

impl Default for HorizontalCoord {
    fn default() -> Self {
        HorizontalCoord {
            alt: Angle::from_radians(0.0),
            az: Angle::from_radians(0.0),
        }
    }
}

/// Equitorial coordinates expressed as (right ascension, declination)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
