use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Read};

use astro_rs::fits::*;

#[test]
fn test_hdu_list_from_bytes() -> Result<(), Box<dyn Error>> {
    {
        let fits_file = File::open("assets/eagle_nebula/502nmos.fits")?;
        let mut fits_file_reader = BufReader::new(fits_file);
        let mut fits_bytes = Vec::new();
        fits_file_reader.read_to_end(&mut fits_bytes)?;

        let in_cursor = Cursor::new(fits_bytes.clone());
        let mut hdu_list = HduList::new(BufReader::new(in_cursor));
        assert_eq!(hdu_list.iter_mut().count(), 2);
        let out_cursor = Cursor::new(Vec::new());
        let mut out_writer = BufWriter::new(out_cursor);
        hdu_list.write(&mut out_writer)?;
        assert_eq!(out_writer.get_ref().get_ref(), &fits_bytes);
    }

    {
        let fits_file = File::open("assets/M17/502nmos.fits")?;
        let mut fits_file_reader = BufReader::new(fits_file);
        let mut fits_bytes = Vec::new();
        fits_file_reader.read_to_end(&mut fits_bytes)?;

        let in_cursor = Cursor::new(fits_bytes.clone());
        let mut hdu_list = HduList::new(BufReader::new(in_cursor));
        assert_eq!(hdu_list.iter_mut().count(), 2);
        let out_cursor = Cursor::new(Vec::new());
        let mut out_writer = BufWriter::new(out_cursor);
        hdu_list.write(&mut out_writer)?;
        assert_eq!(out_writer.get_ref().get_ref(), &fits_bytes);
    }

    Ok(())
}

#[test]
fn test_hdu_list_new() -> Result<(), Box<dyn Error>> {
    let mut hdu_list = HduList::default();
    hdu_list.push(primary_hdu::default());
    hdu_list.push(image_hdu::default());

    assert!(hdu_list.is_header_valid()?);

    Ok(())
}

#[test]
fn test_hdu_table_column() -> Result<(), Box<dyn Error>> {
    let fits_file =
        File::open("assets/CDA/science/ao23/cat9/25975/primary/acisf25975N002_evt2.fits")?;
    let fits_file_reader = BufReader::new(fits_file);
    let mut hdu_list = HduList::new(fits_file_reader);
    let table_hdu = hdu_list.get_by_name("EVENTS").unwrap();
    let energy_data = binary_table_hdu::column_by_name::<f32>(table_hdu, "energy").unwrap();
    let energy_average = energy_data.iter().sum::<f32>() / energy_data.len() as f32;

    assert_eq!(energy_average, 9012.468);

    Ok(())
}
