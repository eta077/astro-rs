use uom::si::angle::second;
use uom::si::f64::Angle;

/// Precession angles, IAU 2006 (Fukushima-Williams 4-angle formulation).
///
/// * t - Interval between fundamental date J2000.0 and given date (JC).
///
/// Returned:
///
/// * gamb - F-W angle gamma_bar
/// * phib - F-W angle phi_bar
/// * psib - F-W angle psi_bar
/// * epsa - F-W angle epsilon_A
///
/// Notes:
///
/// 1) Naming the following points:
///
/// ```text
///           e = J2000.0 ecliptic pole,
///           p = GCRS pole,
///           E = mean ecliptic pole of date,
///     and   P = mean pole of date,
/// ```
///
/// the four Fukushima-Williams angles are as follows:
///
/// ```text
///        gamb = gamma_bar = epE
///        phib = phi_bar = pE
///        psib = psi_bar = pEP
///        epsa = epsilon_A = EP
/// ```
///
/// 2) The matrix representing the combined effects of frame bias and
/// precession is: `PxB = R_1(-epsa).R_3(-psib).R_1(phib).R_3(gamb)`
///
/// 3) The matrix representing the combined effects of frame bias, precession
/// and nutation is simply:
/// `NxPxB = R_1(-epsa-dE).R_3(-psib-dP).R_1(phib).R_3(gamb)`
/// where dP and dE are the nutation components with respect to the ecliptic of
/// date.
pub fn era_pfw06(t: f64) -> (Angle, Angle, Angle, Angle) {
    // P03 bias+precession angles.
    let gamb = Angle::new::<second>(
        -0.052928
            + (10.556378
                + (0.4932044 + (-0.00031238 + (-0.000002788 + 0.0000000260 * t) * t) * t) * t)
                * t,
    );
    let phib = Angle::new::<second>(
        84381.412819
            + (-46.811016
                + (0.0511268 + (0.00053289 + (-0.000000440 + -0.0000000176 * t) * t) * t) * t)
                * t,
    );
    let psib = Angle::new::<second>(
        -0.041775
            + (5038.481484
                + (1.5584175 + (-0.00018522 + (-0.000026452 + -0.0000000148 * t) * t) * t) * t)
                * t,
    );
    let epsa = Angle::new::<second>(
        84381.406
            + (-46.836769
                + (-0.0001831 + (0.00200340 + (-0.000000576 + -0.0000000434 * t) * t) * t) * t)
                * t,
    );

    (gamb, phib, psib, epsa)
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
