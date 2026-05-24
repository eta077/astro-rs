use uom::si::angle::second;
use uom::si::f64::Angle;

use super::fad03::era_fad03;
use super::fae03::era_fae03;
use super::faf03::era_faf03;
use super::fal03::era_fal03;
use super::falp03::era_falp03;
use super::faom03::era_faom03;
use super::fapa03::era_fapa03;
use super::fave03::era_fave03;

struct Term {
    nfa: [i8; 8],
    s: f64,
    c: f64,
}

impl Term {
    fn new(nfa: [i8; 8], s: f64, c: f64) -> Self {
        Self { nfa, s, c }
    }
}

fn get_sp() -> [f64; 6] {
    [
        94.00_E-6,
        3808.65_E-6,
        -122.68_E-6,
        -72574.11_E-6,
        27.98_E-6,
        15.62_E-6,
    ]
}

fn get_s0() -> [Term; 33] {
    [
        /* 1-10 */
        Term::new([0, 0, 0, 0, 1, 0, 0, 0], -2640.73_E-6, 0.39_E-6),
        Term::new([0, 0, 0, 0, 2, 0, 0, 0], -63.53_E-6, 0.02_E-6),
        Term::new([0, 0, 2, -2, 3, 0, 0, 0], -11.75_E-6, -0.01_E-6),
        Term::new([0, 0, 2, -2, 1, 0, 0, 0], -11.21_E-6, -0.01_E-6),
        Term::new([0, 0, 2, -2, 2, 0, 0, 0], 4.57_E-6, 0.00_E-6),
        Term::new([0, 0, 2, 0, 3, 0, 0, 0], -2.02_E-6, 0.00_E-6),
        Term::new([0, 0, 2, 0, 1, 0, 0, 0], -1.98_E-6, 0.00_E-6),
        Term::new([0, 0, 0, 0, 3, 0, 0, 0], 1.72_E-6, 0.00_E-6),
        Term::new([0, 1, 0, 0, 1, 0, 0, 0], 1.41_E-6, 0.01_E-6),
        Term::new([0, 1, 0, 0, -1, 0, 0, 0], 1.26_E-6, 0.01_E-6),
        /* 11-20 */
        Term::new([1, 0, 0, 0, -1, 0, 0, 0], 0.63_E-6, 0.00_E-6),
        Term::new([1, 0, 0, 0, 1, 0, 0, 0], 0.63_E-6, 0.00_E-6),
        Term::new([0, 1, 2, -2, 3, 0, 0, 0], -0.46_E-6, 0.00_E-6),
        Term::new([0, 1, 2, -2, 1, 0, 0, 0], -0.45_E-6, 0.00_E-6),
        Term::new([0, 0, 4, -4, 4, 0, 0, 0], -0.36_E-6, 0.00_E-6),
        Term::new([0, 0, 1, -1, 1, -8, 12, 0], 0.24_E-6, 0.12_E-6),
        Term::new([0, 0, 2, 0, 0, 0, 0, 0], -0.32_E-6, 0.00_E-6),
        Term::new([0, 0, 2, 0, 2, 0, 0, 0], -0.28_E-6, 0.00_E-6),
        Term::new([1, 0, 2, 0, 3, 0, 0, 0], -0.27_E-6, 0.00_E-6),
        Term::new([1, 0, 2, 0, 1, 0, 0, 0], -0.26_E-6, 0.00_E-6),
        /* 21-30 */
        Term::new([0, 0, 2, -2, 0, 0, 0, 0], 0.21_E-6, 0.00_E-6),
        Term::new([0, 1, -2, 2, -3, 0, 0, 0], -0.19_E-6, 0.00_E-6),
        Term::new([0, 1, -2, 2, -1, 0, 0, 0], -0.18_E-6, 0.00_E-6),
        Term::new([0, 0, 0, 0, 0, 8, -13, -1], 0.10_E-6, -0.05_E-6),
        Term::new([0, 0, 0, 2, 0, 0, 0, 0], -0.15_E-6, 0.00_E-6),
        Term::new([2, 0, -2, 0, -1, 0, 0, 0], 0.14_E-6, 0.00_E-6),
        Term::new([0, 1, 2, -2, 2, 0, 0, 0], 0.14_E-6, 0.00_E-6),
        Term::new([1, 0, 0, -2, 1, 0, 0, 0], -0.14_E-6, 0.00_E-6),
        Term::new([1, 0, 0, -2, -1, 0, 0, 0], -0.14_E-6, 0.00_E-6),
        Term::new([0, 0, 4, -2, 4, 0, 0, 0], -0.13_E-6, 0.00_E-6),
        /* 31-33 */
        Term::new([0, 0, 2, -2, 4, 0, 0, 0], 0.11_E-6, 0.00_E-6),
        Term::new([1, 0, -2, 0, -3, 0, 0, 0], -0.11_E-6, 0.00_E-6),
        Term::new([1, 0, -2, 0, -1, 0, 0, 0], -0.11_E-6, 0.00_E-6),
    ]
}

/// Terms of order t^1
fn get_s1() -> [Term; 3] {
    [
        /* 1 - 3 */
        Term::new([0, 0, 0, 0, 2, 0, 0, 0], -0.07_E-6, 3.57_E-6),
        Term::new([0, 0, 0, 0, 1, 0, 0, 0], 1.73_E-6, -0.03_E-6),
        Term::new([0, 0, 2, -2, 3, 0, 0, 0], 0.00_E-6, 0.48_E-6),
    ]
}

/// Terms of order t^2
fn get_s2() -> [Term; 25] {
    [
        /* 1-10 */
        Term::new([0, 0, 0, 0, 1, 0, 0, 0], 743.52_E-6, -0.17_E-6),
        Term::new([0, 0, 2, -2, 2, 0, 0, 0], 56.91_E-6, 0.06_E-6),
        Term::new([0, 0, 2, 0, 2, 0, 0, 0], 9.84_E-6, -0.01_E-6),
        Term::new([0, 0, 0, 0, 2, 0, 0, 0], -8.85_E-6, 0.01_E-6),
        Term::new([0, 1, 0, 0, 0, 0, 0, 0], -6.38_E-6, -0.05_E-6),
        Term::new([1, 0, 0, 0, 0, 0, 0, 0], -3.07_E-6, 0.00_E-6),
        Term::new([0, 1, 2, -2, 2, 0, 0, 0], 2.23_E-6, 0.00_E-6),
        Term::new([0, 0, 2, 0, 1, 0, 0, 0], 1.67_E-6, 0.00_E-6),
        Term::new([1, 0, 2, 0, 2, 0, 0, 0], 1.30_E-6, 0.00_E-6),
        Term::new([0, 1, -2, 2, -2, 0, 0, 0], 0.93_E-6, 0.00_E-6),
        /* 11-20 */
        Term::new([1, 0, 0, -2, 0, 0, 0, 0], 0.68_E-6, 0.00_E-6),
        Term::new([0, 0, 2, -2, 1, 0, 0, 0], -0.55_E-6, 0.00_E-6),
        Term::new([1, 0, -2, 0, -2, 0, 0, 0], 0.53_E-6, 0.00_E-6),
        Term::new([0, 0, 0, 2, 0, 0, 0, 0], -0.27_E-6, 0.00_E-6),
        Term::new([1, 0, 0, 0, 1, 0, 0, 0], -0.27_E-6, 0.00_E-6),
        Term::new([1, 0, -2, -2, -2, 0, 0, 0], -0.26_E-6, 0.00_E-6),
        Term::new([1, 0, 0, 0, -1, 0, 0, 0], -0.25_E-6, 0.00_E-6),
        Term::new([1, 0, 2, 0, 1, 0, 0, 0], 0.22_E-6, 0.00_E-6),
        Term::new([2, 0, 0, -2, 0, 0, 0, 0], -0.21_E-6, 0.00_E-6),
        Term::new([2, 0, -2, 0, -1, 0, 0, 0], 0.20_E-6, 0.00_E-6),
        /* 21-25 */
        Term::new([0, 0, 2, 2, 2, 0, 0, 0], 0.17_E-6, 0.00_E-6),
        Term::new([2, 0, 2, 0, 2, 0, 0, 0], 0.13_E-6, 0.00_E-6),
        Term::new([2, 0, 0, 0, 0, 0, 0, 0], -0.13_E-6, 0.00_E-6),
        Term::new([1, 0, 2, -2, 2, 0, 0, 0], -0.12_E-6, 0.00_E-6),
        Term::new([0, 0, 2, 0, 0, 0, 0, 0], -0.11_E-6, 0.00_E-6),
    ]
}

/// Terms of order t^3
fn get_s3() -> [Term; 4] {
    [
        /* 1-4 */
        Term::new([0, 0, 0, 0, 1, 0, 0, 0], 0.30_E-6, -23.42_E-6),
        Term::new([0, 0, 2, -2, 2, 0, 0, 0], -0.03_E-6, -1.46_E-6),
        Term::new([0, 0, 2, 0, 2, 0, 0, 0], -0.01_E-6, -0.25_E-6),
        Term::new([0, 0, 0, 0, 2, 0, 0, 0], 0.00_E-6, 0.23_E-6),
    ]
}

/// Terms of order t^4
fn get_s4() -> Term {
    Term::new([0, 0, 0, 0, 1, 0, 0, 0], -0.26_E-6, -0.01_E-6)
}

/// The CIO locator s, positioning the Celestial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole, given the CIP's X,Y
/// coordinates.  Compatible with IAU 2006/2000A precession-nutation.
///
/// * t - Interval between fundamental epoch J2000.0 and current date (JC).
/// * x,y - CIP coordinates (Note 2)
///
/// Returned: the CIO locator s (Note 1)
///
/// Notes:
///
/// 1) The CIO locator s is the difference between the right ascensions of the
/// same point in two systems:  the two systems are the GCRS and the CIP,CIO,
/// and the point is the ascending node of the CIP equator.  The quantity s
/// remains below 0.1 arcsecond throughout 1900-2100.
///
/// 2) The series used to compute s is in fact for s+XY/2, where X and Y are
/// the x and y components of the CIP unit vector; this series is more compact
/// than a direct series for s would be.  This function requires X,Y to be
/// supplied by the caller, who is responsible for providing values that are
/// consistent with the supplied date.
///
/// 3) The model is consistent with the "P03" precession (Capitaine et al.
/// 2003), adopted by IAU 2006 Resolution 1, 2006, and the IAU 2000A nutation
/// (with P03 adjustments).
pub fn era_s06(t: f64, x: Angle, y: Angle) -> Angle {
    let fa = [
        // Mean anomaly of the Moon.
        era_fal03(t),
        // Mean anomaly of the Sun.
        era_falp03(t),
        // Mean longitude of the Moon minus that of the ascending node.
        era_faf03(t),
        // Mean elongation of the Moon from the Sun.
        era_fad03(t),
        // Mean longitude of the ascending node of the Moon.
        era_faom03(t),
        // Mean longitude of Venus.
        era_fave03(t),
        // Mean longitude of Earth.
        era_fae03(t),
        // General precession in longitude.
        era_fapa03(t),
    ];

    // Evaluate s.
    let sp = get_sp();
    let mut w0 = sp[0];
    let mut w1 = sp[1];
    let mut w2 = sp[2];
    let mut w3 = sp[3];
    let mut w4 = sp[4];
    let w5 = sp[5];

    for s0 in get_s0().iter().rev() {
        let mut a = 0.0;
        for (nfa, fa) in s0.nfa.iter().zip(fa) {
            a += *nfa as f64 * fa.value;
        }
        w0 += s0.s * a.sin() + s0.c * a.cos();
    }

    for s1 in get_s1().iter().rev() {
        let mut a = 0.0;
        for (nfa, fa) in s1.nfa.iter().zip(fa) {
            a += *nfa as f64 * fa.value;
        }
        w1 += s1.s * a.sin() + s1.c * a.cos();
    }

    for s2 in get_s2().iter().rev() {
        let mut a = 0.0;
        for (nfa, fa) in s2.nfa.iter().zip(fa) {
            a += *nfa as f64 * fa.value;
        }
        w2 += s2.s * a.sin() + s2.c * a.cos();
    }

    for s3 in get_s3().iter().rev() {
        let mut a = 0.0;
        for (nfa, fa) in s3.nfa.iter().zip(fa) {
            a += *nfa as f64 * fa.value;
        }
        w3 += s3.s * a.sin() + s3.c * a.cos();
    }

    let s4 = get_s4();
    let mut a = 0.0;
    for (nfa, fa) in s4.nfa.iter().zip(fa) {
        a += *nfa as f64 * fa.value;
    }
    w4 += s4.s * a.sin() + s4.c * a.cos();

    let xy: Angle = (x * y / 2.0).into();
    Angle::new::<second>(w0 + (w1 + (w2 + (w3 + (w4 + w5 * t) * t) * t) * t) * t) - xy
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
