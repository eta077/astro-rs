use hifitime::Epoch;
use thiserror::Error;

use super::{EarthLocation, EquatorialCoord};

/// An enumeration of errors that can occur while converting coordinates from one frame to another.
#[derive(Debug, Error)]
pub enum AstroConversionError {}

/// Coordinates in the International Celestial Reference System.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Icrs {
    /// The coordinate value
    pub coords: EquatorialCoord,
}

impl Icrs {
    /// Creates a new Icrs with the coordinate values rounded to the specified decimal place.
    pub fn round(&self, dp: u32) -> Self {
        Self {
            coords: self.coords.round(dp),
        }
    }

    /// Converts coordinates from ICRS to observed AltAz coordinates.
    pub fn to_alt_az(
        &self,
        _date_time: &Epoch,
        _location: &EarthLocation,
    ) -> Result<AltAz, AstroConversionError> {
        Ok(AltAz::default())
    }
}

/// Coordinates with respect to the WGS84 ellipsoid.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AltAz {
    /// The coordinate value
    pub coords: EquatorialCoord,
}
