use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use astro_rs::fits::{hdu_types, HduList};

#[test]
fn test_hdu_list_from_bytes() -> Result<(), Box<dyn Error>> {
    {
        let fits_file = File::open("assets/eagle_nebula/502nmos.fits")?;
        let mut fits_file_reader = BufReader::new(fits_file);
        let mut fits_bytes = Vec::new();
        fits_file_reader.read_to_end(&mut fits_bytes)?;

        let mut hdu_list = HduList::from_bytes(fits_bytes.clone());
        let hdu = hdu_list.first_mut().unwrap();
        println!("first hdu has {} cards", hdu.header.cards.len());
        assert_eq!(hdu_list.iter_mut().count(), 2);
        let my_bytes = hdu_list.to_bytes();
        let mut result_file = File::create("test_results.txt").unwrap();
        result_file.write_all(&mut my_bytes.clone()).unwrap();
        assert!(my_bytes == fits_bytes);
    }

    {
        let fits_file = File::open("assets/M17/502nmos.fits")?;
        let mut fits_file_reader = BufReader::new(fits_file);
        let mut fits_bytes = Vec::new();
        fits_file_reader.read_to_end(&mut fits_bytes)?;

        let mut hdu_list = HduList::from_bytes(fits_bytes.clone());
        assert_eq!(hdu_list.iter_mut().count(), 2);
        assert!(hdu_list.to_bytes() == fits_bytes);
    }

    Ok(())
}

#[test]
fn test_hdu_list_new() -> Result<(), Box<dyn Error>> {
    let mut hdu_list = HduList::new();
    hdu_list.push(hdu_types::primary_hdu());
    hdu_list.push(hdu_types::image_hdu());

    assert!(hdu_list.is_header_valid()?);

    Ok(())
}
