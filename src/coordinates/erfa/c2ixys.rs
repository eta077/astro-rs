use uom::si::angle::radian;
use uom::si::f64::Angle;
use uom::ConstZero;

use super::ir::era_ir;
use super::ry::era_ry;
use super::rz::era_rz;

/// Form the celestial to intermediate-frame-of-date matrix given the CIP X,Y
/// and the CIO locator s.
///
/// * x,y - Celestial Intermediate Pole (Note 1)
/// * s - the CIO locator s (Note 2)
///
/// Notes:
///
/// 1) The Celestial Intermediate Pole coordinates are the x,y components of
/// the unit vector in the Geocentric Celestial Reference System.
///
/// 2) The CIO locator s positions the Celestial Intermediate Origin on the
/// equator of the CIP.
///
/// 3) The matrix rc2i is the first stage in the transformation from celestial
/// to terrestrial coordinates:
/// ```text
///        [TRS] = RPOM * R_3(ERA) * rc2i * [CRS]
///              = RC2T * [CRS]                  
/// ```
/// where `[CRS]` is a vector in the Geocentric Celestial Reference System and
/// `[TRS]` is a vector in the International Terrestrial Reference System (see
/// IERS Conventions 2003), ERA is the Earth Rotation Angle and RPOM is the
/// polar motion matrix.
pub fn era_c2ixys(x: Angle, y: Angle, s: Angle) -> [[Angle; 3]; 3] {
    /* Obtain the spherical angles E and d. */
    let r2: Angle = (x * x + y * y).into();
    let e = if r2 > Angle::ZERO {
        Angle::atan2(y, x)
    } else {
        Angle::ZERO
    };
    let d = (r2 / (Angle::new::<radian>(1.0) - r2)).sqrt().atan();

    /* Form the matrix. */
    let mut rc2i = era_ir();
    era_rz(e, &mut rc2i);
    era_ry(d, &mut rc2i);
    era_rz(-(e + s), &mut rc2i);

    rc2i
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
