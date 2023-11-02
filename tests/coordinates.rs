#[cfg(feature = "coordinates")]
mod coordinate_tests {
    use std::error::Error;

    use astro_rs::coordinates::*;

    use hifitime::Epoch;
    use uom::si::angle::degree;
    use uom::si::f64::{Angle, Length};
    use uom::si::length::meter;

    #[test]
    fn test_lookup_by_name() -> Result<(), Box<dyn Error>> {
        let m33_eq_coords = tokio_test::block_on(async { lookup_by_name("M33").await })?;

        let bear_mountain = EarthLocation {
            lat: Angle::new::<degree>(41.3),
            lon: Angle::new::<degree>(-74.0),
            height: Length::new::<meter>(390.0),
        };
        // 11pm EDT on 2012 July 12
        let date_time = Epoch::from_gregorian_utc_hms(2012, 07, 13, 03, 00, 00);

        let m33_horiz_coords = m33_eq_coords.to_alt_az(&date_time, &bear_mountain)?.coords;

        println!("{:?}", m33_horiz_coords.round(4));

        Ok(())
    }
}
