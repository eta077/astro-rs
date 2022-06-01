//! Serialize and deserialize FITS data.
//! See https://archive.stsci.edu/fits/fits_standard/fits_standard.html for the FITS API.

mod hdu_types;
mod header;
mod header_value;

use std::fmt::Debug;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use std::slice::IterMut;

pub use hdu_types::*;
pub use header::*;
pub use header_value::*;

/// The expected keyword for the name of an extension.
pub const EXTNAME_KEYWORD: [u8; 8] = *b"EXTNAME ";
const BLANK_KEYWORD: [u8; 8] = *b"        ";

/// A representation of the entirety of a FITS file.
#[derive(Debug)]
pub struct HduList<R> {
    reader: BufReader<R>,
    hdus: Vec<Hdu>,
}

impl Default for HduList<Cursor<Vec<u8>>> {
    fn default() -> Self {
        Self {
            reader: BufReader::new(Cursor::new(Vec::new())),
            hdus: Default::default(),
        }
    }
}

impl<R: Read> HduList<R> {
    /// Constructs an empty HduList.
    pub fn new(reader: BufReader<R>) -> Self {
        HduList {
            reader,
            hdus: Vec::new(),
        }
    }

    /// Retrieves the HDU at the given index, or None if an HDU doesn't exist at the index.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// assert!(hdu_list.get_by_index(0).is_none());
    ///
    /// hdu_list.push(primary_hdu::default());
    /// assert!(hdu_list.get_by_index(0).is_some());
    /// ```
    pub fn get_by_index(&mut self, index: usize) -> Option<&mut Hdu> {
        let mut cur_hdus = self.hdus.len();
        while cur_hdus < index + 1 {
            let new_hdu = self.read_hdu()?;
            self.hdus.push(new_hdu);
            cur_hdus += 1;
        }
        Some(&mut self.hdus[index])
    }

    /// Retrieves the HDU with the given value for the `EXTNAME` keyword, or None if an HDU
    /// with the given name doesn't exist.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// // empty list
    /// assert!(hdu_list.get_by_name("hdu_name").is_none());
    ///
    /// // name does not match
    /// let mut img_hdu = image_hdu::default();
    /// let name_card = FitsHeaderCard::from(*b"EXTNAME = 'name_of_hdu'                                                         ");
    /// img_hdu.header.cards.insert(img_hdu.header.cards.len() - 1, name_card);
    /// hdu_list.push(img_hdu);
    /// assert!(hdu_list.get_by_name("hdu_name").is_none());
    ///
    /// // name matches
    /// let mut img_hdu = image_hdu::default();
    /// let name_card = FitsHeaderCard::from(*b"EXTNAME = 'hdu_name'                                                            ");
    /// img_hdu.header.cards.insert(img_hdu.header.cards.len() - 1, name_card);
    /// hdu_list.push(img_hdu);
    /// assert!(hdu_list.get_by_name("hdu_name").is_some());
    /// ```
    pub fn get_by_name(&mut self, name: &str) -> Option<&mut Hdu> {
        let index = self
            .hdus
            .iter_mut()
            .position(|hdu| hdu.get_name() == name)
            .or_else(|| {
                let mut index = self.hdus.len();
                loop {
                    let mut new_hdu = self.read_hdu()?;
                    if new_hdu.get_name() == name {
                        self.hdus.push(new_hdu);
                        break;
                    }

                    self.hdus.push(new_hdu);
                    index += 1;
                }
                Some(index)
            });

        Some(&mut self.hdus[index?])
    }

    /// Returns a mutable pointer to the first HDU, or `None` if the list is empty.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// assert!(hdu_list.first_mut().is_none());
    ///
    /// hdu_list.push(primary_hdu::default());
    /// assert!(hdu_list.first_mut().is_some());
    /// ```
    pub fn first_mut(&mut self) -> Option<&mut Hdu> {
        if self.hdus.is_empty() {
            let new_hdu = self.read_hdu()?;
            self.hdus.push(new_hdu);
        }
        Some(&mut self.hdus[0])
    }

    /// Deserializes all HDUs if necessary, then returns a mutable iterator over the HDUs.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// hdu_list.push(primary_hdu::default());
    ///
    /// // find the primary HDU
    /// assert!(hdu_list
    ///     .iter_mut()
    ///     .find_map(|hdu| if hdu.header.get_card(SIMPLE_KEYWORD).is_some() {
    ///         Some(hdu)
    ///     } else {
    ///         None
    ///     })
    ///     .is_some())
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<Hdu> {
        while let Some(new_hdu) = self.read_hdu() {
            self.hdus.push(new_hdu);
        }
        self.hdus.iter_mut()
    }

    /// Deserializes all HDUs up to `index` if necessary, then inserts the given `hdu`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// ```should_panic
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// hdu_list.insert(1, image_hdu::default());
    /// ```
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// hdu_list.push(primary_hdu::default());
    /// hdu_list.insert(1, image_hdu::default());
    /// assert_eq!(hdu_list.iter_mut().count(), 2);
    /// ```
    pub fn insert(&mut self, index: usize, hdu: Hdu) {
        let mut cur_hdus = self.hdus.len();
        while cur_hdus < index {
            if let Some(new_hdu) = self.read_hdu() {
                self.hdus.push(new_hdu);
                cur_hdus += 1;
            } else {
                panic!("{} is out of bounds (max {})", index, cur_hdus);
            }
        }
        self.hdus.insert(index, hdu);
    }

    /// Appends `hdu` to the end of the HDU list.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// hdu_list.push(primary_hdu::default());
    /// assert_eq!(hdu_list.iter_mut().count(), 1);
    /// hdu_list.push(image_hdu::default());
    /// assert_eq!(hdu_list.iter_mut().count(), 2);
    /// ```
    pub fn push(&mut self, hdu: Hdu) {
        while let Some(new_hdu) = self.read_hdu() {
            self.hdus.push(new_hdu);
        }
        self.hdus.push(hdu);
    }

    /// Writes the HDU list via the given writer.
    ///
    /// ```
    /// use astro_rs::fits::*;
    /// use std::io::*;
    ///
    /// let in_cursor = Cursor::new(SIMPLE_KEYWORD.to_vec());
    /// let mut hdu_list = HduList::new(BufReader::new(in_cursor));
    /// let out_cursor = Cursor::new(Vec::new());
    /// let mut out_writer = BufWriter::new(out_cursor);
    /// hdu_list.write(&mut out_writer)?;
    /// assert_eq!(out_writer.get_ref().get_ref(), &SIMPLE_KEYWORD.to_vec());
    ///
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn write<W: Write>(&mut self, writer: &mut BufWriter<W>) -> Result<(), std::io::Error> {
        for hdu in &self.hdus {
            writer.write_all(&hdu.clone().to_bytes())?;
        }
        std::io::copy(&mut self.reader, writer)?;
        Ok(())
    }

    /// Validates the existence and format of the SIMPLE header card.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::default();
    /// // empty header
    /// assert!(!hdu_list.is_header_valid()?);
    ///
    /// let mut hdu = Hdu::new();
    ///
    /// // non-empty header missing simple card
    /// let bitpix_card = FitsHeaderCard::from(*b"BITPIX  =                  -32 / FITS BITS/PIXEL                                ");
    /// hdu.header.cards.insert(0, bitpix_card);
    /// hdu_list.push(hdu);
    /// assert!(!hdu_list.is_header_valid()?);
    ///
    /// // valid header
    /// let simple_card = FitsHeaderCard::from(*b"SIMPLE  =                    T / FITS STANDARD                                  ");
    /// hdu_list.first_mut().unwrap().header.cards.insert(0, simple_card);
    /// assert!(hdu_list.is_header_valid()?);
    /// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
    /// ```
    pub fn is_header_valid(&mut self) -> Result<bool, FitsHeaderError> {
        Ok(*self
            .get_by_index(0)
            .and_then(|hdu| hdu.header.get_card(SIMPLE_KEYWORD))
            .and_then(|card| card.get_value::<bool>().ok())
            .unwrap_or_default())
    }

    fn read_hdu(&mut self) -> Option<Hdu> {
        let mut header_raw = Vec::new();
        let mut new_header_bytes = vec![0; FITS_RECORD_LEN];
        self.reader.read_exact(&mut new_header_bytes).ok()?;
        header_raw.append(&mut new_header_bytes);

        // search for the END keyword.
        // this should be the last keyword in the header, so if something other than ' ' is found, stop searching
        loop {
            let mut end_found = false;
            for card in 1..=FITS_RECORD_LEN / HEADER_CARD_LEN {
                let card_index = header_raw.len() - card * HEADER_CARD_LEN;
                match header_raw[card_index..card_index + HEADER_KEYWORD_LEN]
                    .try_into()
                    .unwrap()
                {
                    END_KEYWORD => {
                        end_found = true;
                        break;
                    }
                    BLANK_KEYWORD => continue,
                    _ => {
                        end_found = false;
                        break;
                    }
                }
            }
            if end_found {
                break;
            }
            new_header_bytes = vec![0; FITS_RECORD_LEN];
            self.reader.read_exact(&mut new_header_bytes).ok()?;
            header_raw.append(&mut new_header_bytes);
        }

        let mut header = FitsHeader::from_bytes(header_raw);
        let mut data_raw = Vec::new();

        let naxis = *header
            .get_card(NAXIS_KEYWORD)
            .and_then(|card| card.get_value::<u16>().ok())
            .unwrap_or_default();
        if naxis != 0 {
            if let Some(bitpix) = header
                .get_card(BITPIX_KEYWORD)
                .and_then(|card| card.get_value::<Bitpix>().ok())
            {
                let mut data_len = 1;
                let mut naxisx_keyword = FitsHeaderKeyword::from(NAXIS_KEYWORD);
                for x in 1..=naxis {
                    naxisx_keyword.append_number(x);

                    let naxisx = *header
                        .get_card(naxisx_keyword)
                        .and_then(|card| card.get_value::<u32>().ok())
                        .unwrap_or_default() as usize;
                    data_len *= naxisx;
                }
                data_len *= bitpix.value() / 8;
                if data_len % FITS_RECORD_LEN != 0 {
                    let num_records = (data_len / FITS_RECORD_LEN) + 1;
                    data_len = num_records * FITS_RECORD_LEN;
                }
                data_raw = vec![0; data_len];
                let _ = self.reader.read_exact(&mut data_raw);
            }
        }
        Some(Hdu { header, data_raw })
    }
}

/// A Header Data Unit within a FITS file.
#[derive(Debug, Default, Clone)]
pub struct Hdu {
    /// The header section of the HDU.
    pub header: FitsHeader,
    data_raw: Vec<u8>,
}

impl Hdu {
    /// Constructs an HDU with the given header and data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Serializes the contents of the HDU to bytes.
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut result = self.header.to_bytes();
        result.append(&mut self.data_raw);
        result
    }

    /// Gets the name of the HDU, or an empty string if the name cannot be determined.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut img_hdu = image_hdu::default();
    /// let name_card = FitsHeaderCard::from(*b"EXTNAME = 'hdu_name'                                                            ");
    /// img_hdu.header.cards.insert(img_hdu.header.cards.len() - 1, name_card);
    /// assert_eq!(img_hdu.get_name(), String::from("hdu_name"));
    /// ```
    pub fn get_name(&mut self) -> String {
        self.header
            .get_card(EXTNAME_KEYWORD)
            .and_then(|card| card.get_value::<String>().ok())
            .map(|name| name.trim().to_owned())
            .unwrap_or_default()
    }

    /// Gets the data section of the HDU.
    pub fn data_raw(&self) -> &Vec<u8> {
        &self.data_raw
    }

    /// Creates a Vec containing the data section of the HDU as the type defined by the BITPIX keyword.
    pub fn get_data<T>(&mut self) -> Vec<T> {
        *self
            .header
            .get_card(BITPIX_KEYWORD)
            .and_then(|card| card.get_value::<Bitpix>().ok())
            .map(|bitpix| unsafe {
                match *bitpix {
                    Bitpix::U8 => crate::return_box!(self.data_raw.to_owned()),
                    Bitpix::I16 => {
                        let mut data = Vec::with_capacity(self.data_raw.len() / 2);
                        for chunk in self.data_raw.chunks_exact(2) {
                            data.push(i16::from_be_bytes(chunk.try_into().unwrap()));
                        }
                        crate::return_box!(data)
                    }
                    Bitpix::I32 => {
                        let mut data = Vec::with_capacity(self.data_raw.len() / 4);
                        for chunk in self.data_raw.chunks_exact(4) {
                            data.push(i32::from_be_bytes(chunk.try_into().unwrap()));
                        }
                        crate::return_box!(data)
                    }
                    Bitpix::F32 => {
                        let mut data = Vec::with_capacity(self.data_raw.len() / 4);
                        for chunk in self.data_raw.chunks_exact(4) {
                            data.push(f32::from_be_bytes(chunk.try_into().unwrap()));
                        }
                        crate::return_box!(data)
                    }
                    Bitpix::F64 => {
                        let mut data = Vec::with_capacity(self.data_raw.len() / 8);
                        for chunk in self.data_raw.chunks_exact(8) {
                            data.push(f64::from_be_bytes(chunk.try_into().unwrap()));
                        }
                        crate::return_box!(data)
                    }
                }
            })
            .unwrap_or_default()
    }

    /// Creates a Vec containing the dimensions of the data section of the HDU as defined by the NAXIS keywords.
    pub fn get_dimensions(&mut self) -> Vec<usize> {
        let naxis = *self
            .header
            .get_card(NAXIS_KEYWORD)
            .and_then(|card| card.get_value::<u16>().ok())
            .unwrap_or_default();
        if naxis == 0 {
            return Vec::new();
        }
        let mut result = Vec::with_capacity(naxis as usize);
        let mut naxisx_keyword = FitsHeaderKeyword::from(NAXIS_KEYWORD);
        for x in 1..=naxis {
            naxisx_keyword.append_number(x);

            let naxisx = *self
                .header
                .get_card(naxisx_keyword)
                .and_then(|card| card.get_value::<u32>().ok())
                .unwrap_or_default() as usize;
            result.push(naxisx);
        }
        result
    }
}

pub(crate) mod hdu_macros {
    /// Creates a box of the given value and casts it to an implicit return type.
    #[macro_export]
    macro_rules! return_box {
        ($result: expr) => {{
            let b = Box::new($result);
            let ptr = Box::into_raw(b);
            let new_ptr = ptr.cast();
            Box::from_raw(new_ptr)
        }};
    }
}
