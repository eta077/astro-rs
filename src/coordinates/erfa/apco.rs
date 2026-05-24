use uom::si::f64::{Angle, Length};
use uom::ConstZero;

use super::anpm::era_anpm;
use super::apcs::era_apcs;
use super::c2ixys::era_c2ixys;
use super::ir::era_ir;
use super::pvtob::era_pvtob;
use super::rx::era_rx;
use super::ry::era_ry;
use super::rz::era_rz;
use super::trxpv::era_trxpv;
use super::EraAstrom;
use super::PositionVelocityVector;

/// For a terrestrial observer, prepare star-independent astrometry parameters
/// for transformations between ICRS and observed coordinates.  The caller
/// supplies the Earth ephemeris, the Earth rotation information and the
/// refraction constants as well as the site coordinates.
///
/// * date1 - TDB as a 2-part...
/// * date2 - ...Julian Date (Note 1)
/// * ebpv - Earth barycentric PV (Note 2)
/// * ehp - Earth heliocentric P (Note 2)
/// * x,y - CIP X,Y (components of unit vector)
/// * s - the CIO locator s
/// * theta - Earth rotation angle
/// * elong - longitude (east +ve, Note 3)
/// * phi - latitude (geodetic, Note 3)
/// * hm - height above ellipsoid (geodetic, Note 3)
/// * xp,yp - polar motion coordinates (Note 4)
/// * sp - the TIO locator s' (Note 4)
/// * refa - refraction constant A (Note 5)
/// * refb - refraction constant B (Note 5)
///
/// Notes:
///
/// 1) The TDB date date1+date2 is a Julian Date, apportioned in any convenient
/// way between the two arguments.  For example, `JD(TDB)=2450123.7` could be
/// expressed in any of these ways, among others:
///
/// |   date1   |   date2   |                      |
/// |-----------|-----------|----------------------|
/// | 2450123.7 |      0.0  | (JD method)          |
/// | 2451545.0 |  -1421.3  | (J2000 method)       |
/// | 2400000.5 |  50123.2  | (MJD method)         |
/// | 2450123.5 |      0.2  | (date & time method) |
///
/// The JD method is the most natural and convenient to use in cases where the
/// loss of several decimal digits of resolution is acceptable.  The J2000
/// method is best matched to the way the argument is handled internally and
/// will deliver the optimum resolution.  The MJD method and the date & time
/// methods are both good compromises between resolution and convenience.  For
/// most applications of this function the choice will not be at all critical.
/// TT can be used instead of TDB without any significant impact on accuracy.
///
/// 2) The vectors eb, eh, and all the astrom vectors, are with respect to BCRS
/// axes.
///
/// 3) The geographical coordinates are with respect to the ERFA_WGS84
/// reference ellipsoid.  TAKE CARE WITH THE LONGITUDE SIGN CONVENTION:  the
/// longitude required by the present function is right-handed, i.e.
/// east-positive, in accordance with geographical convention. The adjusted
/// longitude stored in the astrom array takes into account the TIO locator and
/// polar motion.
///
/// 4) xp and yp are the coordinates of the Celestial Intermediate Pole with
/// respect to the International Terrestrial Reference System (see IERS
/// Conventions), measured along the meridians 0 and 90 deg west respectively.  
/// sp is the TIO locator s', which positions the Terrestrial Intermediate
/// Origin on the equator.  For many applications, xp, yp and (especially) sp
/// can be set to zero. Internally, the polar motion is stored in a form
/// rotated onto the local meridian.
///
/// 5) The refraction constants refa and refb are for use in a `dZ = A*tan(Z)
/// +B*tan^3(Z)` model, where Z is the observed (i.e. refracted) zenith
/// distance and dZ is the amount of refraction.
#[allow(clippy::too_many_arguments)]
pub fn era_apco(
    date1: f64,
    date2: f64,
    ebpv: PositionVelocityVector,
    ehp: [Length; 3],
    x: Angle,
    y: Angle,
    s: Angle,
    theta: Angle,
    elong: Angle,
    phi: Angle,
    hm: Length,
    xp: Angle,
    yp: Angle,
    sp: Angle,
    refa: Angle,
    refb: Angle,
) -> EraAstrom {
    let mut astrom = EraAstrom::default();

    /* Form the rotation matrix, CIRS to apparent [HA,Dec]. */
    let mut r = era_ir();
    era_rz(theta + sp, &mut r);
    era_ry(-xp, &mut r);
    era_rx(-yp, &mut r);
    era_rz(elong, &mut r);

    /* Solve for local Earth rotation angle. */
    let a = r[0][0];
    let b = r[0][1];
    let eral = if a != Angle::ZERO || b != Angle::ZERO {
        Angle::atan2(b, a)
    } else {
        Angle::ZERO
    };
    astrom.eral = eral;

    /* Solve for polar motion [X,Y] with respect to local meridian. */
    let a = r[0][0];
    let c = r[0][2];
    astrom.xpl = Angle::atan2(c, (a * a + b * b).sqrt().into());
    let a = r[1][2];
    let b = r[2][2];
    astrom.ypl = if a != Angle::ZERO || b != Angle::ZERO {
        -Angle::atan2(a, b)
    } else {
        Angle::ZERO
    };

    /* Adjusted longitude. */
    astrom.along = era_anpm(eral - theta);

    /* Functions of latitude. */
    astrom.phi = phi;
    astrom.sphi = phi.sin().into();
    astrom.cphi = phi.cos().into();

    /* Refraction constants. */
    astrom.refa = refa;
    astrom.refb = refb;

    /* Disable the (redundant) diurnal aberration step. */
    astrom.diurab = 0.0;

    /* CIO based BPN matrix. */
    let r = era_c2ixys(x, y, s);

    /* Observer's geocentric position and velocity (m, m/s, CIRS). */
    let pvc = era_pvtob(elong, phi, hm, xp, yp, sp, theta);

    /* Rotate into GCRS. */
    let pv = era_trxpv(r.map(|ar| ar.map(|a| a.value)), pvc);

    /* ICRS <-> GCRS parameters. */
    era_apcs(date1, date2, pv, ebpv, ehp, &mut astrom);

    /* Store the CIO based BPN matrix. */
    astrom.bpn = r;

    astrom
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
