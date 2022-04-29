use chrono::{DateTime, Utc};

use super::{EarthLocation, EquatorialCoord, HorizontalCoord};

/// Coordinates in the International Celestial Reference System.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Icrs {
    /// The coordinate value
    pub coords: EquatorialCoord,
}

impl Icrs {
    /// Converts coordinates from ICRS to AltAz
    pub fn as_alt_az(&self, _date_time: &DateTime<Utc>, _location: &EarthLocation) -> AltAz {
        AltAz::default()
    }
}

/// Coordinates
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AltAz {
    /// The coordinate value
    pub coords: HorizontalCoord,
}
