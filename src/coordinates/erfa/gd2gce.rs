use uom::si::f64::{Angle, Length};

/// Transform geodetic coordinates to geocentric for a reference ellipsoid of
/// specified form.
///
/// * a - equatorial radius
/// * f - flattening (Note 1)
/// * elong - longitude (east +ve)
/// * phi - latitude (geodetic)
/// * height - height above ellipsoid (geodetic)
///
/// Returned: xyz - geocentric vector
///
/// Notes:
///
/// 1) The flattening, f, is (for the Earth) a value around 0.00335, i.e.
/// around 1/298.
///
/// 2) No validation is performed on individual arguments.
///
/// 3) The inverse transformation is performed in the function era_gc2gde.
///
/// 4) The transformation for a standard ellipsoid (such as ERFA_WGS84) can
/// more conveniently be performed by calling era_gd2gc, which uses
/// [ReferenceEllipsoid](super::ReferenceEllipsoid) to identify the required a
/// and f values.
pub fn era_gd2gce(a: Length, f: f64, elong: Angle, phi: Angle, height: Length) -> [Length; 3] {
    let sp = phi.sin();
    let cp = phi.cos();
    let mut w = 1.0 - f;
    w = w * w;
    let d = cp * cp + w * sp * sp;
    // if ( d <= 0.0 ) return -1;
    let ac = a / d.sqrt();
    let r#as = w * ac;

    /* Geocentric vector. */
    let r = (ac + height) * cp;
    let xyz = [r * elong.cos(), r * elong.sin(), (r#as + height) * sp];

    xyz
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
