use uom::si::angle::radian;
use uom::si::f64::Angle;
use uom::ConstZero;

use crate::coordinates::EquatorialCoord;

use super::anp::era_anp;
use super::c2s::era_c2s;
use super::s2c::era_s2c;
use super::EraAstrom;

///  Quick CIRS to observed place transformation.
///
///  Use of this function is appropriate when efficiency is important and where
/// many star positions are all to be transformed for one date.  The
/// star-independent astrometry parameters can be obtained by calling era_apio
/// or era_apco.
///
/// * cirs - CIRS right ascension and declination
/// * astrom - star-independent astrometry parameters
///
///  Returned:
///     aob    double*    observed azimuth (radians: N=0,E=90)
///     zob    double*    observed zenith distance (radians)
///     hob    double*    observed hour angle (radians)
///     dob    double*    observed declination (radians)
///     rob    double*    observed right ascension (CIO-based, radians)
///
/// Notes:
///
/// 1) This function returns zenith distance rather than altitude in order to
/// reflect the fact that no allowance is made for depression of the horizon.
///
/// 2) The accuracy of the result is limited by the corrections for refraction,
/// which use a simple A*tan(z) + B*tan^3(z) model.  Providing the
/// meteorological parameters are known accurately and there are no gross local
/// effects, the predicted observed coordinates should be within 0.05 arcsec
/// (optical) or 1 arcsec (radio) for a zenith distance of less than 70
/// degrees, better than 30 arcsec (optical or radio) at 85 degrees and better
/// than 20 arcmin (optical) or 30 arcmin (radio) at the horizon. Without
/// refraction, the complementary functions era_atioq and era_atoiq are
/// self-consistent to better than 1 microarcsecond all over the celestial
/// sphere.  With refraction included, consistency falls off at high zenith
/// distances, but is still better than 0.05 arcsec at 85 degrees.
///
/// 3) The CIRS RA,Dec is obtained from a star catalog mean place by allowing
/// for space motion, parallax, the Sun's gravitational lens effect, annual
/// aberration and precession-nutation.  For star positions in the ICRS, these
/// effects can be applied by means of the eraAtci13 (etc.) functions.  
/// Starting from classical "mean place" systems, additional transformations
/// will be needed first.
///
/// 5) "Observed" Az,El means the position that would be seen by a perfect
/// geodetically aligned theodolite.  This is obtained from the CIRS RA,Dec by
/// allowing for Earth orientation and diurnal aberration, rotating from
/// equator to horizon coordinates, and then adjusting for refraction.  The HA,
/// Dec is obtained by rotating back into equatorial coordinates, and is the
/// position that would be seen by a perfect equatorial with its polar axis
/// aligned to the Earth's axis of rotation.  Finally, the RA is obtained by
/// subtracting the HA from the local ERA.
///
/// 6) The star-independent CIRS-to-observed-place parameters in ASTROM may be
/// computed with era_apio or era_apco.  If nothing has changed significantly
/// except the time, era_aper may be used to perform the requisite adjustment
/// to the astrom structure.
pub fn era_atioq(cirs: EquatorialCoord, astrom: &EraAstrom) -> (Angle, Angle, Angle, Angle, Angle) {
    /* Minimum cos(alt) and sin(alt) for refraction purposes */
    let celmin = 1e-6;
    let selmin = Angle::new::<radian>(0.05);

    // CIRS RA,Dec to Cartesian -HA,Dec.
    let mut v = era_s2c(cirs.ra - astrom.eral, cirs.dec);
    let x = v[0];
    let y = v[1];
    let z = v[2];

    // Polar motion.
    let sx = astrom.xpl.sin();
    let cx = astrom.xpl.cos();
    let sy = astrom.ypl.sin();
    let cy = astrom.ypl.cos();
    let xhd = cx * x + sx * z;
    let yhd = sx * sy * x + cy * y - cx * sy * z;
    let zhd = -sx * cy * x + sy * y + cx * cy * z;

    // Diurnal aberration.
    let f = (1.0 - astrom.diurab) * yhd;
    let xhdt = f * xhd;
    let yhdt: Angle = (f * (yhd.value + astrom.diurab)).into();
    let zhdt = f * zhd;

    // Cartesian -HA,Dec to Cartesian Az,El (S=0,E=90).
    let xaet: Angle = (astrom.sphi * xhdt - astrom.cphi * zhdt).into();
    let yaet = yhdt;
    let zaet: Angle = (astrom.cphi * xhdt + astrom.sphi * zhdt).into();

    // Azimuth (N=0,E=90).
    let azobs = if xaet != Angle::ZERO || yaet != Angle::ZERO {
        Angle::atan2(yaet, -xaet)
    } else {
        Angle::ZERO
    };

    /* ---------- */
    /* Refraction */
    /* ---------- */

    // Cosine and sine of altitude, with precautions.
    let r = Angle::new::<radian>(
        (xaet.value * xaet.value + yaet.value * yaet.value)
            .sqrt()
            .max(celmin),
    );
    let z = if zaet > selmin { zaet } else { selmin };

    // A*tan(z)+B*tan^3(z) model, with Newton-Raphson correction.
    let tz: Angle = (r / z).into();
    let w: Angle = (astrom.refb * tz * tz).into();
    let denom_a: Angle = astrom.refa + 3.0 * w;
    let denom_b: Angle = (z * z).into();
    let denom: Angle = (denom_a / denom_b).into();
    let del: Angle = ((astrom.refa + w) * tz / (Angle::new::<radian>(1.0) + denom)).into();

    // Apply the change, giving observed vector.
    let cosdel = Angle::new::<radian>(1.0) - Angle::from(del * del / 2.0);
    let f = cosdel - Angle::from(del * z / r);
    let xaeo = xaet * f;
    let yaeo: Angle = (yaet * f).into();
    let zaeo = cosdel * zaet + del * r;

    // Observed ZD.
    let zdobs = Angle::new::<radian>(f64::atan2(
        f64::sqrt(xaeo.value * xaeo.value + yaeo.value * yaeo.value),
        zaeo.value,
    ));

    // Az/El vector to HA,Dec vector (both right-handed).
    v[0] = (astrom.sphi * xaeo + astrom.cphi * zaeo).into();
    v[1] = yaeo;
    v[2] = (-astrom.cphi * xaeo + astrom.sphi * zaeo).into();

    // To spherical -HA,Dec.
    let obs = era_c2s(v.map(|v| v.value));

    // Right ascension (with respect to CIO).
    let raobs = astrom.eral + obs.ra;

    (era_anp(azobs), zdobs, -obs.ra, obs.dec, era_anp(raobs))
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
