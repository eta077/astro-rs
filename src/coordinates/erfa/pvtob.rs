use uom::si::f64::{Angle, Length, Velocity};
use uom::si::length::meter;
use uom::si::velocity::meter_per_second;
use uom::ConstZero;

use super::gd2gc::era_gd2gc;
use super::pom00::era_pom00;
use super::trxp::era_trxp;
use super::OM;
use super::{PositionVelocityVector, ReferenceEllipsoid};

/// Position and velocity of a terrestrial observing station.
///
/// * elong - longitude (east +ve, Note 1)
/// * phi - latitude (geodetic, Note 1)
/// * hm - height above ref. ellipsoid (geodetic, m)
/// * xp,yp - coordinates of the pole (Note 2)
/// * sp - the TIO locator s' (Note 2)
/// * theta - Earth rotation angle
///
/// Returned: pv - position/velocity vector (CIRS)
///
/// Notes:
///
/// 1) The terrestrial coordinates are with respect to the ERFA_WGS84 reference
/// ellipsoid.
///
/// 2) xp and yp are the coordinates of the Celestial Intermediate Pole with
/// respect to the International Terrestrial Reference System (see IERS
/// Conventions), measured along the meridians 0 and 90 deg west respectively.  
/// sp is the TIO locator s', which positions the Terrestrial Intermediate
/// Origin on the equator.  For many applications, xp, yp and (especially) sp
/// can be set to zero.
///
/// 3) The velocity units are meters per UT1 second, not per SI second. This is
/// unlikely to have any practical consequences in the modern era.
///
/// 4) No validation is performed on the arguments.  Error cases that could
/// lead to arithmetic exceptions are trapped by the era_gd2gc function, and
/// the result set to zeros.
pub fn era_pvtob(
    elong: Angle,
    phi: Angle,
    hm: Length,
    xp: Angle,
    yp: Angle,
    sp: Angle,
    theta: Angle,
) -> PositionVelocityVector {
    /* Geodetic to geocentric transformation (ERFA_WGS84). */
    let xyzm = era_gd2gc(ReferenceEllipsoid::WGS84, elong, phi, hm);

    /* Polar motion and TIO position. */
    let rpm = era_pom00(xp, yp, sp);
    let xyz = era_trxp(rpm, xyzm.map(|l| l.value)).map(Length::new::<meter>);
    let x = xyz[0];
    let y = xyz[1];
    let z = xyz[2];

    /* Functions of ERA. */
    let s = theta.sin();
    let c = theta.cos();

    /* Position. */
    let position = [c * x - s * y, s * x + c * y, z];

    /* Velocity. */
    let mut velocity = [Velocity::ZERO; 3];
    velocity[0] = Velocity::new::<meter_per_second>(OM * (-s * x - c * y).value);
    velocity[1] = Velocity::new::<meter_per_second>(OM * (c * x - s * y).value);
    velocity[2] = Velocity::ZERO;

    PositionVelocityVector { position, velocity }
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
