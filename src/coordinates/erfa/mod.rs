//! Utilities translated from ERFA (Essential Routines for Fundamental
//! Astronomy). ERFA is a C library containing key algorithms for astronomy,
//! and is based on the SOFA library <http://www.iausofa.org/> published by
//! the International Astronomical Union (IAU). Note that this code has a
//! separate LICENSE, included in and alongside these files.

mod ab;
mod anp;
mod anpm;
mod apco;
mod apcs;
mod atioq;
mod c2ixys;
mod c2s;
mod eform;
mod epv00;
mod era00;
mod fad03;
mod fae03;
mod faf03;
mod fal03;
mod falp03;
mod faom03;
mod fapa03;
mod fave03;
mod fw2m;
mod gd2gc;
mod gd2gce;
mod ir;
mod ld;
mod nut00a;
mod nut06a;
mod pdp;
mod pfw06;
mod pm;
mod pn;
mod pnm06a;
mod pom00;
mod pvtob;
mod pxp;
mod rx;
mod rxp;
mod rxpv;
mod ry;
mod rz;
mod s06;
mod s2c;
mod sp00;
mod sxp;
mod tr;
mod trxp;
mod trxpv;

use hifitime::Epoch;
use uom::si::f64::{Angle, Length, Velocity};
use uom::ConstZero;

use self::ab::era_ab;
use self::apco::era_apco;
use self::atioq::era_atioq;
use self::c2s::era_c2s;
use self::epv00::era_epv00;
use self::era00::era_era00;
use self::ld::era_ld;
use self::pnm06a::era_pnm06a;
use self::rxp::era_rxp;
use self::s06::era_s06;
use self::s2c::era_s2c;
use self::sp00::era_sp00;

use super::EquatorialCoord;
use super::Icrs;
use super::{AltAz, EarthLocation};

/// Reference epoch (J2000.0), Julian Date
const DJ00: f64 = 2451545.0;
/// Days per Julian year
const DJY: f64 = 365.25;
/// Days per Julian century
const DJC: f64 = 36525.0;
/// Units of 0.1 microarcsecond to radians
const U2R: f64 = 4.848_136_811_095_36_E-6 / 1_E7;
/// Schwarzschild radius of the Sun (au) = 2 * 1.32712440041e20 / (2.99792458e8)^2 / 1.49597870700e11
const SRS: f64 = 1.97412574336e-8;
/// Arcseconds in a full circle
const TURNAS: f64 = 1296000.0;
/// Astronomical unit (m, IAU 2012)
const DAU: f64 = 149597870.7e3;
/// Seconds per day.
const DAYSEC: f64 = 86400.0;
/// au/d to m/s
const AUDMS: f64 = DAU / DAYSEC;
/// 2Pi
const D2PI: f64 = 2.0 * std::f64::consts::PI;
/// Earth rotation rate in radians per UT1 second
const OM: f64 = 1.0027378119113545 * D2PI / 86400.0;
/// Speed of light (m/s)
const CMPS: f64 = 299792458.0;

/// Star-independent astrometry parameters.
#[derive(Debug, Default)]
pub struct EraAstrom {
    /// PM time interval (SSB, Julian years)
    pub pmt: f64,
    /// SSB to observer
    pub eb: [Length; 3],
    /// Sun to observer (unit vector)
    pub eh: [f64; 3],
    /// distance from Sun to observer
    pub em: Length,
    /// barycentric observer velocity
    pub v: [Velocity; 3],
    /// sqrt(1-|v|^2): reciprocal of Lorenz factor
    pub bm1: f64,
    /// bias-precession-nutation matrix
    pub bpn: [[Angle; 3]; 3],
    /// longitude + s' + dERA(DUT)
    pub along: Angle,
    /// geodetic latitude
    pub phi: Angle,
    /// polar motion xp wrt local meridian
    pub xpl: Angle,
    /// polar motion yp wrt local meridian
    pub ypl: Angle,
    /// sine of geodetic latitude
    pub sphi: Angle,
    /// cosine of geodetic latitude
    pub cphi: Angle,
    /// magnitude of diurnal aberration vector
    pub diurab: f64,
    /// "local" Earth rotation angle
    pub eral: Angle,
    /// refraction constant A
    pub refa: Angle,
    /// refraction constant B
    pub refb: Angle,
}

/// An object's position and velocity.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PositionVelocityVector {
    pub position: [Length; 3],
    pub velocity: [Velocity; 3],
}

/// An enumeration of supported reference ellipsoids.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum ReferenceEllipsoid {
    WGS84,
    GRS80,
    WGS72,
}

/// Converts coordinates from ICRS to observed AltAz coordinates.
pub fn erfa_transform_icrs_to_observed(
    icrs: &Icrs,
    location: &EarthLocation,
    date_time: &Epoch,
    pm_x: Angle,
    pm_y: Angle,
) -> AltAz {
    let jd = date_time.to_jde_tt_days();
    let jd1_tt = jd.ceil();
    let jd2_tt = (jd1_tt - jd) * -1.0;
    let t = ((jd1_tt - DJ00) + jd2_tt) / DJC;
    let sp = era_sp00(t);

    let rpnb = era_pnm06a(t);

    let x = rpnb[2][0];
    let y = rpnb[2][1];
    let s = era_s06(t, x, y);

    let era = era_era00(date_time);

    let (earth_pv_helio, earth_pv) = calculate_earth_position_velocity(date_time);

    let astrom = era_apco(
        jd1_tt,
        jd2_tt,
        earth_pv,
        earth_pv_helio.position,
        x,
        y,
        s,
        era,
        location.lon,
        location.lat,
        location.height,
        pm_x,
        pm_y,
        sp,
        Angle::ZERO,
        Angle::ZERO,
    );

    let topocentric_cirs = atciqz(icrs, &astrom);

    let (lon, zen, _, _, _) = era_atioq(topocentric_cirs, &astrom);
    let lat = Angle::HALF_TURN / 2.0 - zen;

    AltAz {
        coords: EquatorialCoord { ra: lon, dec: lat },
    }
}

fn calculate_earth_position_velocity(
    date_time: &Epoch,
) -> (PositionVelocityVector, PositionVelocityVector) {
    let jd = date_time.to_jde_tdb_days();
    let jd1_tdb = jd.ceil();
    let jd2_tdb = (jd1_tdb - jd) * -1.0;
    let t = ((jd1_tdb - DJ00) + jd2_tdb) / DJY;

    era_epv00(t)
}

/// Performs transformations between two coordinate systems. This function has
/// been modified from the ERFA `atciqz` function by increasing the threshold
/// for artificially reducing light deflection from 9 arcseconds to 90
/// arcseconds, and accounting for the difference between the object-Sun vector
/// and observer-Sun vector.
fn atciqz(icrs: &Icrs, astrom: &EraAstrom) -> EquatorialCoord {
    let pco = era_s2c(icrs.coords.ra, icrs.coords.dec);

    let pnat = era_ld(
        1.0,
        pco.map(|a| a.value),
        pco.map(|a| a.value),
        astrom.eh,
        astrom.em,
        1e-6,
    );

    let ppr = era_ab(pnat, astrom.v, astrom.em, astrom.bm1);

    let pi = era_rxp(astrom.bpn.map(|v| v.map(|a| a.value)), ppr);

    era_c2s(pi)
}

/*----------------------------------------------------------------------
**
**
**  Copyright (C) 2013-2021, NumFOCUS Foundation.
**  All rights reserved.
**
**  This library is derived, with permission, from the International
**  Astronomical Union's "Standards of Fundamental Astronomy" library,
**  available from http://www.iausofa.org.
**
**  The ERFA version is intended to retain identical functionality to
**  the SOFA library, but made distinct through different function and
**  file names, as set out in the SOFA license conditions.  The SOFA
**  original has a role as a reference standard for the IAU and IERS,
**  and consequently redistribution is permitted only in its unaltered
**  state.  The ERFA version is not subject to this restriction and
**  therefore can be included in distributions which do not support the
**  concept of "read only" software.
**
**  Although the intent is to replicate the SOFA API (other than
**  replacement of prefix names) and results (with the exception of
**  bugs;  any that are discovered will be fixed), SOFA is not
**  responsible for any errors found in this version of the library.
**
**  If you wish to acknowledge the SOFA heritage, please acknowledge
**  that you are using a library derived from SOFA, rather than SOFA
**  itself.
**
**
**  TERMS AND CONDITIONS
**
**  Redistribution and use in source and binary forms, with or without
**  modification, are permitted provided that the following conditions
**  are met:
**
**  1 Redistributions of source code must retain the above copyright
**    notice, this list of conditions and the following disclaimer.
**
**  2 Redistributions in binary form must reproduce the above copyright
**    notice, this list of conditions and the following disclaimer in
**    the documentation and/or other materials provided with the
**    distribution.
**
**  3 Neither the name of the Standards Of Fundamental Astronomy Board,
**    the International Astronomical Union nor the names of its
**    contributors may be used to endorse or promote products derived
**    from this software without specific prior written permission.
**
**  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
**  "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
**  LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
**  FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE
**  COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
**  INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
**  BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
**  LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
**  CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
**  LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
**  ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
**  POSSIBILITY OF SUCH DAMAGE.
**
*/
