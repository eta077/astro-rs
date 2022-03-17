use std::error::Error;
use std::fs::File;

#[test]
fn test_read_fits_file() -> Result<(), Box<dyn Error>> {
    let fits_file = File::open("assets/502nmos.fits")?;

    astro_rs::fits::read_fits_file(fits_file, true)?;

    Ok(())
}
