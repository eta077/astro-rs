use std::error::Error;

use astro_rs::coordinates::*;

use chrono::{DateTime, Utc};
use measurements::{Angle, Distance};

#[test]
fn test_lookup_by_name() -> Result<(), Box<dyn Error>> {
    let m33_eq_coords = tokio_test::block_on(async { lookup_by_name("M33").await })?;
    let bear_mountain = EarthLocation {
        lat: Angle::from_degrees(41.3),
        lon: Angle::from_degrees(-74.0),
        height: Distance::from_meters(390.0),
    };
    // 11pm EDT on 2012 July 12
    let date_time = DateTime::parse_from_rfc3339("2012-07-12T23:00:00.00-04:00")?;

    let m33_horiz_coords =
        m33_eq_coords.calculate_horizontal_coords(&date_time.with_timezone(&Utc), &bear_mountain);

    println!("{}, {}", m33_horiz_coords.alt.as_degrees(), m33_horiz_coords.az.as_degrees());

    Ok(())
}
