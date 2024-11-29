//! Obtain IERS (International Earth Rotation and Reference Systems Service)
//! data, including polar motion.

use std::io::{BufRead, BufReader, Cursor};

use hifitime::Epoch;
use num_traits::cast::FromPrimitive;
use once_cell::sync::OnceCell;
use thiserror::Error;
use uom::si::angle::second;
use uom::si::f64::Angle;

static IERS_DATA: OnceCell<Vec<IersEntry>> = OnceCell::new();

/// An enumeration of errors that can occur when obtaining IERS data.
#[derive(Debug, Error)]
pub enum IersError {
    ///
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    ///
    #[error("{reason}")]
    ParseError {
        ///
        reason: String,
    },
    ///
    #[error("IERS data for {date:?} not found")]
    DateNotFound {
        ///
        date: Epoch,
    },
}

/// Gets IERS data. If the data is not currently initialized, initialization will be attempted.
/// Returns an error if the data files cannot be parsed.
///
/// # Examples
///
/// ```
/// use astro_rs::iers::{self, *};
///
/// let data = iers::get_iers_data().unwrap();
/// assert!(data.len() > 0);
/// ```
pub fn get_iers_data() -> Result<&'static Vec<IersEntry>, IersError> {
    IERS_DATA.get_or_try_init(init_iers_data)
}

fn init_iers_data() -> Result<Vec<IersEntry>, IersError> {
    let iers_a_res = read_iers_file_a();
    let iers_b_res = read_iers_file_b();

    match (iers_a_res, iers_b_res) {
        (Ok(mut iers_a), Ok(iers_b)) => {
            let mut a = 0;
            let mut b = 0;
            'replace: loop {
                while iers_a[a].mjd < iers_b[b].mjd {
                    a += 1;
                    if a == iers_a.len() {
                        break 'replace;
                    }
                }
                while iers_a[a].mjd > iers_b[b].mjd {
                    b += 1;
                    if b == iers_b.len() {
                        break 'replace;
                    }
                }
                while iers_a[a].mjd == iers_b[b].mjd {
                    iers_a[a] = iers_b[b];
                    a += 1;
                    b += 1;
                    if a == iers_a.len() || b == iers_b.len() {
                        break 'replace;
                    }
                }
            }
            Ok(iers_a)
        }
        (Ok(iers_a), Err(_)) => Ok(iers_a),
        (Err(_), Ok(iers_b)) => Ok(iers_b),
        (Err(err_a), Err(_)) => Err(err_a),
    }
}

/// Calculates the polar motion values for the exact given date.
pub fn interpolate_polar_motion(date: &Epoch) -> Result<(Angle, Angle), IersError> {
    get_iers_data().and_then(|data| {
        let mjd_utc = date.to_mjd_utc_days();
        let utc = mjd_utc % 1.0;
        let mjd = i32::from_f64(mjd_utc).ok_or_else(|| IersError::DateNotFound {
            date: date.to_owned(),
        })?;

        let i0 = data
            .iter()
            .position(|entry| entry.mjd == mjd)
            .ok_or_else(|| IersError::DateNotFound {
                date: date.to_owned(),
            })?;

        if i0 == data.len() - 1 {
            let entry = data[0];
            return Ok((entry.pm_x, entry.pm_y));
        }

        let i1 = i0 + 1;
        let e0 = data[i0];
        let e1 = data[i1];
        let dx = e1.pm_x - e0.pm_x;
        let dy = e1.pm_y - e0.pm_y;
        let pm_x = e0.pm_x + ((mjd - e0.mjd) as f64 + utc) / (e1.mjd - e0.mjd) as f64 * dx;
        let pm_y = e0.pm_y + ((mjd - e0.mjd) as f64 + utc) / (e1.mjd - e0.mjd) as f64 * dy;

        Ok((pm_x, pm_y))
    })
}

fn read_iers_file_a() -> Result<Vec<IersEntry>, IersError> {
    let raw = include_bytes!("data/finals2000A.all");
    let mut reader = BufReader::new(Cursor::new(raw));
    let mut result = Vec::new();
    loop {
        let mut line = String::with_capacity(186);
        reader.read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        match IersEntry::try_from_file_a(line) {
            Ok(entry) => result.push(entry),
            Err(e) => {
                if result.is_empty() {
                    return Err(e);
                }
            }
        }
    }
    Ok(result)
}

fn read_iers_file_b() -> Result<Vec<IersEntry>, IersError> {
    let raw = include_bytes!("data/eopc04_IAU2000.62-now");
    let mut reader = BufReader::new(Cursor::new(raw));
    let mut line = String::new();
    for _ in 0..14 {
        reader.read_line(&mut line)?;
    }
    let mut result = Vec::new();
    loop {
        let mut line = String::with_capacity(156);
        reader.read_line(&mut line)?;
        if line.is_empty() {
            break;
        }
        let entry = IersEntry::try_from_file_b(line)?;
        result.push(entry);
    }
    result.sort_by_key(|entry| entry.mjd);
    Ok(result)
}

/// A container for IERS data.
#[derive(Debug, Copy, Clone)]
pub struct IersEntry {
    /// Calendar year
    pub year: i32,
    /// Calendar month
    pub month: u8,
    /// Calendar day (0 hr UTC)
    pub day: u8,
    /// Modified Julian Date (MJD, 0 hr UTC)
    pub mjd: i32,
    /// polar motion x
    pub pm_x: Angle,
    /// polar motion y
    pub pm_y: Angle,
    /// Difference UT1-UTC
    pub ut1_utc: f32,
    /// Length of day
    pub lod: f32,
    /// dX wrt IAU2000A Nutation
    pub dx_2000a: f32,
    /// dY wrt IAU2000A Nutation
    pub dy_2000a: f32,
    /// error in polar motion x
    pub e_pm_x: f32,
    /// error in polar motion y
    pub e_pm_y: f32,
    /// error in UT1-UTC
    pub e_ut1_utc: f32,
    /// error in length of day
    pub e_lod: f32,
    /// error in dX_2000A
    pub e_dx_2000a: f32,
    /// error in dY_2000A
    pub e_dy_2000a: f32,
}

impl IersEntry {
    fn try_from_file_a(mut value: String) -> Result<Self, IersError> {
        // remove new line
        if value.ends_with('\n') {
            value.pop();
        }
        if value.len() != 185 {
            return Err(IersError::ParseError {
                reason: [
                    "IERS entry from source A must be 185 characters, found ",
                    &value.len().to_string(),
                ]
                .concat(),
            });
        }

        let mut remainder = value.split_off(2);
        let mut year = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse year from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(2);

        let month = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse month from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(2);

        let day = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse day from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(9);

        let mjd = value
            .trim()
            .parse::<f32>()
            .map_err(|_| IersError::ParseError {
                reason: ["Failed to parse MJD from ", &value].concat(),
            })? as i32;
        if mjd > 51543 {
            year += 2000;
        } else {
            year += 1900;
        }

        value = remainder;
        remainder = value.split_off(2);

        // let pol_prediction = match value.chars().nth(1).unwrap() {
        //     'I' => false,
        //     'P' => true,
        //     c => {
        //         return Err(IersError::ParseError {
        //             reason: ["Unexpected value for pol flag: '", &c.to_string(), "'"].concat(),
        //         })
        //     }
        // };

        value = remainder;
        remainder = value.split_off(10);

        let pm_x_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse PM_x_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(9);

        let e_pm_x_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_PM_x_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(10);

        let pm_y_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse PM_y_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(9);

        let e_pm_y_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_PM_y_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(3);

        // let ut1_prediction = match value.chars().nth(2).unwrap() {
        //     'I' => false,
        //     'P' => true,
        //     c => {
        //         return Err(IersError::ParseError {
        //             reason: ["Unexpected value for UT1 flag: '", &c.to_string(), "'"].concat(),
        //         })
        //     }
        // };

        value = remainder;
        remainder = value.split_off(10);

        let ut1_utc_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse UT1-UTC_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(10);

        let e_ut1_utc_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_UT1_UTC_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(8);

        let lod_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse LOD_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(7);

        let e_lod_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_LOD_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(3);

        // let nut_prediction = match value.chars().nth(2).unwrap() {
        //     'I' => false,
        //     'P' => true,
        //     c => {
        //         return Err(IersError::ParseError {
        //             reason: ["Unexpected value for Nutation flag: '", &c.to_string(), "'"].concat(),
        //         })
        //     }
        // };

        value = remainder;
        remainder = value.split_off(10);

        let dx_2000a_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse dX_2000A_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(9);

        let e_dx_2000a_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_dX_2000A_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(10);

        let dy_2000a_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse dY_2000A_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(9);

        let e_dy_2000a_a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_dY_2000A_A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(10);

        let pm_x_b_value = value;

        value = remainder;
        remainder = value.split_off(10);

        let pm_y_b_value = value;

        let pm_x_b = pm_x_b_value
            .trim()
            .parse()
            .map_err(|_| IersError::ParseError {
                reason: ["Failed to parse PM_x_B from ", &pm_x_b_value].concat(),
            });
        let pm_y_b = pm_y_b_value
            .trim()
            .parse()
            .map_err(|_| IersError::ParseError {
                reason: ["Failed to parse PM_y_B from ", &pm_y_b_value].concat(),
            });

        let (pm_x, pm_y) = if let (Ok(x), Ok(y)) = (pm_x_b, pm_y_b) {
            (x, y)
        } else {
            (pm_x_a, pm_y_a)
        };

        value = remainder;
        remainder = value.split_off(11);

        let ut1_utc_b = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse UT1-UTC_B from ", &value].concat(),
        });

        let ut1_utc = ut1_utc_b.unwrap_or(ut1_utc_a);

        value = remainder;
        remainder = value.split_off(10);

        let dx_2000a_b = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse dX_2000A_B from ", &value].concat(),
        });

        value = remainder;

        let dy_2000a_b = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse dY_2000A_B from ", &value].concat(),
        });

        let (dx_2000a, dy_2000a) = if let (Ok(x), Ok(y)) = (dx_2000a_b, dy_2000a_b) {
            (x, y)
        } else {
            (dx_2000a_a, dy_2000a_a)
        };

        Ok(IersEntry {
            year,
            month,
            day,
            mjd,
            pm_x: Angle::new::<second>(pm_x),
            pm_y: Angle::new::<second>(pm_y),
            ut1_utc,
            lod: lod_a,
            dx_2000a,
            dy_2000a,
            e_pm_x: e_pm_x_a,
            e_pm_y: e_pm_y_a,
            e_ut1_utc: e_ut1_utc_a,
            e_lod: e_lod_a,
            e_dx_2000a: e_dx_2000a_a,
            e_dy_2000a: e_dy_2000a_a,
        })
    }

    fn try_from_file_b(mut value: String) -> Result<Self, IersError> {
        // remove new line
        if value.ends_with('\n') {
            value.pop();
        }
        if value.len() != 155 {
            return Err(IersError::ParseError {
                reason: [
                    "IERS entry from source B must be 155 characters, found ",
                    &value.len().to_string(),
                ]
                .concat(),
            });
        }
        let mut remainder = value.split_off(4);

        let year = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse year from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(4);

        let month = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse month from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(4);

        let day = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse day from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(7);

        let mjd = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse MJD from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let pm_x = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse PM_x from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let pm_y = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse PM_y from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(12);

        let ut1_utc = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse UT1-UTC from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(12);

        let lod = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse LOD from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let dx_2000a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse dX_2000A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let dy_2000a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse dY_2000A from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let e_pm_x = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_PM_x from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let e_pm_y = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_PM_y from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let e_ut1_utc = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_UT1_UTC from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(11);

        let e_lod = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_LOD from ", &value].concat(),
        })?;

        value = remainder;
        remainder = value.split_off(12);

        let e_dx_2000a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_dX_2000A from ", &value].concat(),
        })?;

        value = remainder;

        let e_dy_2000a = value.trim().parse().map_err(|_| IersError::ParseError {
            reason: ["Failed to parse e_dY_2000A from ", &value].concat(),
        })?;

        Ok(IersEntry {
            year,
            month,
            day,
            mjd,
            pm_x: Angle::new::<second>(pm_x),
            pm_y: Angle::new::<second>(pm_y),
            ut1_utc,
            lod,
            dx_2000a,
            dy_2000a,
            e_pm_x,
            e_pm_y,
            e_ut1_utc,
            e_lod,
            e_dx_2000a,
            e_dy_2000a,
        })
    }
}
