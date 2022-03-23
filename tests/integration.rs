use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use astro_rs::fits::HduList;

#[test]
fn test_hdu_list_from_bytes() -> Result<(), Box<dyn Error>> {
    let fits_file = File::open("assets/502nmos.fits")?;
    let mut fits_file_reader = BufReader::new(fits_file);
    let mut fits_bytes = Vec::new();
    fits_file_reader.read_to_end(&mut fits_bytes)?;
    
    let hdu_list = HduList::from_bytes(fits_bytes)?;
    assert!(hdu_list.hdus.len() == 2);

    Ok(())
}
