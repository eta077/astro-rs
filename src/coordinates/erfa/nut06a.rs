use uom::si::f64::Angle;

use super::nut00a::era_nut00a;

/// IAU 2000A nutation with adjustments to match the IAU 2006 precession.
///
/// * t - Interval between fundamental date J2000.0 and given date (JC).
///
/// Returned: dpsi,deps - nutation, luni-solar + planetary (Note 1)
///
/// Notes:
///
/// 1) The nutation components in longitude and obliquity are with respect to
/// the mean equinox and ecliptic of date, IAU 2006 precession model (Hilton et
/// al. 2006, Capitaine et al. 2005).
///
/// 2) The function first computes the IAU 2000A nutation, then applies
/// adjustments for (i) the consequences of the change in obliquity from the
/// IAU 1980 ecliptic to the IAU 2006 ecliptic and (ii) the secular variation
/// in the Earth's dynamical form factor J2.
///
/// 3) The present function provides classical nutation, complementing the IAU
/// 2000 frame bias and IAU 2006 precession.  It delivers a pole which is at
/// current epochs accurate to a few tens of microarcseconds, apart from the
/// free core nutation.
pub fn era_nut06a(t: f64) -> (Angle, Angle) {
    /* Factor correcting for secular variation of J2. */
    let fj2 = -2.7774_E-6 * t;

    /* Obtain IAU 2000A nutation. */
    let (dp, de) = era_nut00a(t);

    /* Apply P03 adjustments (Wallace & Capitaine, 2006, Eqs.5). */
    let dpsi = dp + dp * (0.4697_E-6 + fj2);
    let deps = de + de * fj2;

    (dpsi, deps)
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
