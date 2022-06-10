mod fits_tests {
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

    #[test]
    fn test_hdu_image() -> Result<(), Box<dyn Error>> {
        let fits_file = File::open("assets/eagle_nebula/502nmos.fits")?;
        let fits_file_reader = BufReader::new(fits_file);
        let mut hdu_list = HduList::new(fits_file_reader);
        let primary_hdu = hdu_list.first_mut().unwrap();

        let dimensions = primary_hdu.get_dimensions();
        assert_eq!(dimensions, vec![1600, 1600]);

        let data = primary_hdu.get_data::<Vec<f32>>()?;
        let mut data_min = 0.0;
        let mut data_max = 0.0;
        for value in &data {
            if *value > data_max {
                data_max = *value;
            } else if *value < data_min {
                data_min = *value;
            }
        }
        assert_eq!(data_max, 2925.8718);
        assert_eq!(data_min, -12.439324);
        Ok(())
    }

    #[test]
    fn test_image_to_hdu() -> Result<(), Box<dyn Error>> {
        use image::GenericImageView;

        let img = image::open("assets/eagle_nebula/eagle_composite.jpg")?;
        let (dim_x, dim_y) = img.dimensions();
        let size = (dim_x * dim_y) as usize;
        let mut r = Vec::with_capacity(size);
        let mut g = Vec::with_capacity(size);
        let mut b = Vec::with_capacity(size);
        for (_x, _y, rgb) in img.pixels() {
            r.push(rgb.0[0] as i32);
            g.push(rgb.0[1] as i32);
            b.push(rgb.0[2] as i32);
        }

        let mut r_fits = HduList::default();
        let mut r_hdu = primary_hdu::default();
        r_hdu.header.set_card(
            BITPIX_KEYWORD,
            Bitpix::I32,
            Some(String::from("array data type")),
        )?;
        let mut naxis_keyword = FitsHeaderKeyword::from(NAXIS_KEYWORD);
        r_hdu.header.set_card(
            naxis_keyword,
            2u16,
            Some(String::from("number of array dimensions")),
        )?;
        naxis_keyword.append_number(1);
        r_hdu.header.set_card(naxis_keyword, dim_x, None)?;
        naxis_keyword.append_number(2);
        r_hdu.header.set_card(naxis_keyword, dim_y, None)?;

        r_hdu.header.set_comment(
            SIMPLE_KEYWORD,
            Some(String::from("conforms to FITS standard")),
        )?;
        r_hdu.header.set_card(*b"EXTEND  ", true, None)?;

        r_hdu.set_data(&r);
        r_fits.push(r_hdu);
        let file = File::create("red.fits")?;
        r_fits.write(&mut BufWriter::new(file))?;

        Ok(())
    }
}
