use uom::si::f64::Length;
use uom::si::length::astronomical_unit;

use super::pdp::era_pdp;
use super::pxp::era_pxp;
use super::SRS;

/// Apply light deflection by a solar-system body, as part of transforming
/// coordinate direction into natural direction.
///
///  * bm - mass of the gravitating body (solar masses, Note 2)
///  * p - direction from observer to source (unit vector)
///  * q - direction from body to source (unit vector)
///  * e - direction from body to observer (unit vector)
///  * em - distance from body to observer
///  * dlim - deflection limiter (Note 4)
///
/// Returned: p1 - observer to deflected source (unit vector)
///
/// Notes:
///
/// 1) The algorithm is based on Expr. (70) in Klioner (2003) and Expr. (7.63)
/// in the Explanatory Supplement (Urban & Seidelmann 2013), with some
/// rearrangement to minimize the effects of machine precision.
///
/// 2) The mass parameter bm can, as required, be adjusted in order to allow
/// for such effects as quadrupole field.
///
/// 3) The barycentric position of the deflecting body should ideally
/// correspond to the time of closest approach of the light ray to the body.
///
/// 4) The deflection limiter parameter dlim is phi^2/2, where phi is the
/// angular separation between source and body at which limiting is applied.  
/// As phi shrinks below the chosen threshold, the deflection is artificially
/// reduced, reaching zero for phi = 0.
///
/// 5) The returned vector p1 is not normalized, but the consequential
/// departure from unit magnitude is always negligible.
///
/// 6) To accumulate total light deflection taking into account the
/// contributions from several bodies, call the present function for each body
/// in succession, in decreasing order of distance from the observer.
///
/// 7) For efficiency, validation is omitted.  The supplied vectors must be of
/// unit magnitude, and the deflection limiter non-zero and positive.
pub fn era_ld(bm: f64, p: [f64; 3], q: [f64; 3], e: [f64; 3], em: Length, dlim: f64) -> [f64; 3] {
    let mut qpe = [0.0; 3];
    for i in 0..3 {
        qpe[i] = q[i] + e[i];
    }
    let qdqpe = era_pdp(q, qpe);

    /* 2 x G x bm / ( em x c^2 x ( q . (q + e) ) ). */
    let w = bm * SRS / em.get::<astronomical_unit>() / f64::max(qdqpe, dlim);

    /* p x (e x q). */
    let eq = era_pxp(e, q);
    let peq = era_pxp(p, eq);

    /* Apply the deflection. */
    let mut p1 = [0.0; 3];
    for i in 0..3 {
        p1[i] = p[i] + w * peq[i];
    }

    p1
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
