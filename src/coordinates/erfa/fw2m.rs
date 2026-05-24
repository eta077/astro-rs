use uom::si::f64::Angle;

use super::ir::era_ir;
use super::rx::era_rx;
use super::rz::era_rz;

/// Form rotation matrix given the Fukushima-Williams angles.
///
/// * gamb - F-W angle gamma_bar (radians)
/// * phib - F-W angle phi_bar (radians)
/// * psi - F-W angle psi (radians)
/// * eps - F-W angle epsilon
///
/// Notes:
///
/// 1) Naming the following points:
///
/// ```text
///           e = J2000.0 ecliptic pole,
///           p = GCRS pole,
///           E = ecliptic pole of date,
///     and   P = CIP,
/// ```
///
/// the four Fukushima-Williams angles are as follows:
///
/// ```text
///        gamb = gamma = epE
///        phib = phi = pE
///        psi = psi = pEP
///        eps = epsilon = EP
/// ```
///
/// 2) The matrix representing the combined effects of frame bias, precession
/// and nutation is: `NxPxB = R_1(-eps).R_3(-psi).R_1(phib).R_3(gamb)`
///
/// 3) The present function can construct three different matrices, depending
/// on which angles are supplied as the arguments gamb, phib, psi and eps:
///
///     * To obtain the nutation x precession x frame bias matrix, first
/// generate the four precession angles known conventionally as gamma_bar,
/// phi_bar, psi_bar and epsilon_A, then generate the nutation components Dpsi
/// and Depsilon and add them to psi_bar and epsilon_A, and finally call the
/// present function using those four angles as arguments.
///
///     * To obtain the precession x frame bias matrix, generate the four
/// precession angles and call the present function.
///
///     * To obtain the frame bias matrix, generate the four precession angles
/// for date J2000.0 and call the present function.
///
///     The nutation-only and precession-only matrices can if necessary be
/// obtained by combining these three appropriately.
pub fn era_fw2m(gamb: Angle, phib: Angle, psi: Angle, eps: Angle) -> [[Angle; 3]; 3] {
    /* Construct the matrix. */
    let mut r = era_ir();
    era_rz(gamb, &mut r);
    era_rx(phib, &mut r);
    era_rz(-psi, &mut r);
    era_rx(-eps, &mut r);

    r
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
