//! Compare, calculate, and transform spacial coordinates

mod erfa;
mod frames;
mod lookup;
mod lookup_config;

use rust_decimal::Decimal;
use uom::si::angle::radian;
use uom::si::f64::{Angle, Length};
use uom::ConstZero;

pub use frames::*;
pub use lookup::*;
pub use lookup_config::*;

/// Equitorial coordinates expressed as (right ascension, declination)
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EquatorialCoord {
    /// The right ascension angle
    pub ra: Angle,
    /// The declination angle
    pub dec: Angle,
}

impl EquatorialCoord {
    /// Constructs an EquitorialCoord. The given right ascension and declination angles will be normalized to [0.0, 2π)
    pub fn new(ra: Angle, dec: Angle) -> Self {
        Self {
            ra: Self::normalize(ra),
            dec: Self::normalize(dec),
        }
    }

    fn normalize(a: Angle) -> Angle {
        if a < Angle::ZERO {
            (a % Angle::FULL_TURN) + Angle::FULL_TURN
        } else {
            a % Angle::FULL_TURN
        }
    }

    /// Creates a new EquitorialCoord with the angle values rounded to the specified decimal place.
    pub fn round(&self, dp: u32) -> Self {
        let ra = Decimal::from_f64_retain(self.ra.value)
            .unwrap()
            .round_dp(dp)
            .try_into()
            .unwrap();
        let dec = Decimal::from_f64_retain(self.dec.value)
            .unwrap()
            .round_dp(dp)
            .try_into()
            .unwrap();
        Self {
            ra: Angle::new::<radian>(ra),
            dec: Angle::new::<radian>(dec),
        }
    }
}

/// Coordinates that represent a location on Earth.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EarthLocation {
    /// The latitude coordinate
    pub lat: Angle,
    /// The longitude coordinate
    pub lon: Angle,
    /// The height of the location
    pub height: Length,
}
