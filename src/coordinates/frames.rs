use hifitime::Epoch;
use thiserror::Error;

use crate::iers::{self, IersError};

use super::erfa;
use super::{EarthLocation, EquatorialCoord};

/// An enumeration of errors that can occur while converting coordinates from one frame to another.
#[derive(Debug, Error)]
pub enum AstroConversionError {
    /// Indicates an error occurred while obtaining the IERS data.
    #[error(transparent)]
    IersError(#[from] IersError),
}

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
    ///
    /// # Examples
    ///
    /// ```
    /// use astro_rs::coordinates::*;
    ///
    /// use hifitime::Epoch;
    /// use uom::si::angle::degree;
    /// use uom::si::f64::{Angle, Length};
    /// use uom::si::length::meter;
    ///
    /// let m33_eq_coords = Icrs {
    ///     coords: EquatorialCoord {
    ///         ra: Angle::new::<degree>(23.46206906218),
    ///         dec: Angle::new::<degree>(30.66017511198)
    ///     }
    /// };
    ///
    /// let bear_mountain = EarthLocation {
    ///     lat: Angle::new::<degree>(41.3),
    ///     lon: Angle::new::<degree>(-74.0),
    ///     height: Length::new::<meter>(390.0),
    /// };
    /// // 11pm EDT on 2012 July 12
    /// let date_time = Epoch::from_gregorian_utc_hms(2012, 07, 13, 03, 00, 00);
    ///
    /// let m33_horiz_coords = m33_eq_coords.to_alt_az(&date_time, &bear_mountain)?.coords;
    ///
    /// assert_eq!(
    ///     m33_horiz_coords,
    ///     EquatorialCoord {
    ///         ra: Angle::new::<degree>(47.30684898059173),
    ///         dec: Angle::new::<degree>(0.12850663686910466),
    ///     }
    /// );
    /// # Ok::<(), astro_rs::coordinates::AstroConversionError>(())
    /// ```
    pub fn to_alt_az(
        &self,
        date_time: &Epoch,
        location: &EarthLocation,
    ) -> Result<AltAz, AstroConversionError> {
        let (pm_x, pm_y) = iers::interpolate_polar_motion(date_time)?;

        let coords = erfa::erfa_transform_icrs_to_observed(self, location, date_time, pm_x, pm_y);

        Ok(coords)
    }
}

/// Coordinates with respect to the WGS84 ellipsoid.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AltAz {
    /// The coordinate value
    pub coords: EquatorialCoord,
}
