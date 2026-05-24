use uom::si::f64::Angle;

/// Rotate an r-matrix about the y-axis.
///
/// Notes:
///
/// 1) Calling this function with positive theta incorporates in the supplied
/// r-matrix r an additional rotation, about the y-axis, anticlockwise as seen
/// looking towards the origin from positive y.
///
/// 2) The additional rotation can be represented by this matrix:
///
/// ```text
///         (  + cos(theta)     0      - sin(theta)  )
///         (                                        )
///         (       0           1           0        )
///         (                                        )
///         (  + sin(theta)     0      + cos(theta)  )
/// ```
pub fn era_ry(theta: Angle, r: &mut [[Angle; 3]; 3]) {
    let s = theta.sin();
    let c = theta.cos();

    let a00 = (c * r[0][0] - s * r[2][0]).into();
    let a01 = (c * r[0][1] - s * r[2][1]).into();
    let a02 = (c * r[0][2] - s * r[2][2]).into();
    let a20 = (s * r[0][0] + c * r[2][0]).into();
    let a21 = (s * r[0][1] + c * r[2][1]).into();
    let a22 = (s * r[0][2] + c * r[2][2]).into();

    r[0][0] = a00;
    r[0][1] = a01;
    r[0][2] = a02;
    r[2][0] = a20;
    r[2][1] = a21;
    r[2][2] = a22;
}
