//! Serialize and deserialize FITS data.
//! See https://archive.stsci.edu/fits/fits_standard/fits_standard.html for the FITS API.

pub mod hdu_types;
mod header;
mod header_value;

use std::fmt::Debug;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use std::rc::Rc;
use std::slice::IterMut;

pub use header::*;
pub use header_value::*;

/// The expected keyword for the name of an extension.
pub const EXTNAME_KEYWORD: [u8; 8] = *b"EXTNAME ";
const BLANK_KEYWORD: [u8; 8] = *b"        ";

/// A representation of the entirety of a FITS file.
#[derive(Debug)]
pub struct HduList<T> {
    reader: BufReader<T>,
    hdus: Vec<Hdu>,
}

impl Default for HduList<Cursor<Vec<u8>>> {
    fn default() -> Self {
        Self {
            reader: BufReader::new(Cursor::new(Vec::<u8>::new())),
            hdus: Default::default(),
        }
    }
}

impl<T: Read> HduList<T> {
    /// Constructs an empty HduList.
    pub fn new(reader: BufReader<T>) -> Self {
        HduList {
            reader,
            hdus: Vec::new(),
        }
    }

    /// Retrieves the HDU at the given index.
    pub fn get_by_index(&mut self, index: usize) -> Option<&mut Hdu> {
        let mut cur_hdus = self.hdus.len();
        while cur_hdus < index + 1 {
            let new_hdu = self.read_hdu()?;
            self.hdus.push(new_hdu);
            cur_hdus += 1;
        }
        Some(&mut self.hdus[index])
    }

    /// Retrieves the HDU with the given value for the `EXTNAME` keyword.
    pub fn get_by_name(&mut self, name: &str) -> Option<&mut Hdu> {
        let mut index = self
            .hdus
            .iter_mut()
            .position(|hdu| hdu.get_name() == name)
            .unwrap_or(self.hdus.len() - 1);
        loop {
            let mut new_hdu = self.read_hdu()?;
            if new_hdu.get_name() == name {
                break;
            }

            self.hdus.push(new_hdu);
            index += 1;
        }
        Some(&mut self.hdus[index])
    }

    /// Returns a mutable pointer to the first HDU, or `None` if the list is empty.
    pub fn first_mut(&mut self) -> Option<&mut Hdu> {
        if self.hdus.is_empty() {
            let new_hdu = self.read_hdu()?;
            self.hdus.push(new_hdu);
        }
        Some(&mut self.hdus[0])
    }

    /// Deserializes all HDUs if necessary, then returns a mutable iterator over the HDUs.
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
    pub fn push(&mut self, hdu: Hdu) {
        while let Some(new_hdu) = self.read_hdu() {
            self.hdus.push(new_hdu);
        }
        self.hdus.push(hdu);
    }

    /// Writes the HDU list via the given writer.
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
                for x in 1..=naxis {
                    let mut naxisx_keyword = NAXIS_KEYWORD;
                    let x_bytes = x.to_string().into_bytes();
                    for (index, i) in x_bytes.iter().enumerate() {
                        naxisx_keyword[index + 5] = *i;
                    }

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
        Some(Hdu {
            header,
            data_raw,
            ..Default::default()
        })
    }
}

/// A Header Data Unit within a FITS file.
#[derive(Debug, Default, Clone)]
pub struct Hdu {
    /// The header section of the HDU.
    pub header: FitsHeader,
    data_raw: Vec<u8>,
    data: Option<Rc<dyn FitsDataCollection>>,
}

impl Hdu {
    /// Constructs an HDU with the given header and data.
    pub fn new() -> Self {
        Self::default()
    }

    /// Serializes the contents of the HDU to bytes.
    pub fn to_bytes(self) -> Vec<u8> {
        let mut result = self.header.to_bytes();
        let mut data_raw = if let Some(data) = self.data {
            let mut data_raw = data.to_bytes();
            if data_raw.len() % FITS_RECORD_LEN != 0 {
                let num_records = (data_raw.len() / FITS_RECORD_LEN) + 1;
                let final_len = num_records * FITS_RECORD_LEN;
                // TODO: need to determine which padding value to use
                data_raw.resize(final_len, b' ');
            }
            data_raw
        } else {
            self.data_raw
        };
        result.append(&mut data_raw);
        result
    }

    /// Gets the name of the HDU, or an empty string if the name cannot be determined.
    pub fn get_name(&mut self) -> String {
        let value = self
            .header
            .cards
            .iter_mut()
            .find(|card| *card.get_keyword() == EXTNAME_KEYWORD)
            .and_then(|card| card.get_value::<String>().ok())
            .unwrap_or_default();
        (*value).clone()
    }

    /// Gets the data section of the HDU.
    pub fn get_data<T: FitsDataCollection + 'static>(&mut self) -> Result<Rc<T>, FitsHeaderError> {
        if let Some(data) = &self.data {
            unsafe {
                let ptr = Rc::into_raw(Rc::clone(data));
                let new_ptr: *const T = ptr.cast();
                Ok(Rc::from_raw(new_ptr))
            }
        } else {
            let data = Rc::new(T::from_bytes(self.data_raw.drain(..).collect())?);
            let ret = Rc::clone(&data);
            self.data = Some(data);
            Ok(ret)
        }
    }
}

/// A trait that allows data to be serialized/deserialized as the data section of an HDU.
pub trait FitsDataCollection: Debug {
    /// Attempts to deserialize a data collection from the given bytes.
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError>
    where
        Self: Sized;

    /// Serializes the data collection to bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

impl FitsDataCollection for Vec<u8> {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        Ok(raw)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_owned()
    }
}

impl FitsDataCollection for Vec<i16> {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let mut data = Vec::with_capacity(raw.len() / 2);
        for chunk in raw.chunks_exact(2) {
            data.push(i16::from_be_bytes(chunk.try_into().unwrap()));
        }
        Ok(data)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.len() * 2);
        for chunk in self {
            data.extend_from_slice(&chunk.to_be_bytes());
        }
        data
    }
}

impl FitsDataCollection for Vec<i32> {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let mut data = Vec::with_capacity(raw.len() / 4);
        for chunk in raw.chunks_exact(4) {
            data.push(i32::from_be_bytes(chunk.try_into().unwrap()));
        }
        Ok(data)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.len() * 4);
        for chunk in &*self {
            data.extend_from_slice(&chunk.to_be_bytes());
        }
        data
    }
}

impl FitsDataCollection for Vec<f32> {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let mut data = Vec::with_capacity(raw.len() / 4);
        for chunk in raw.chunks_exact(4) {
            data.push(f32::from_be_bytes(chunk.try_into().unwrap()));
        }
        Ok(data)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.len() * 4);
        for chunk in &*self {
            data.extend_from_slice(&chunk.to_be_bytes());
        }
        data
    }
}

impl FitsDataCollection for Vec<f64> {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let mut data = Vec::with_capacity(raw.len() / 8);
        for chunk in raw.chunks_exact(8) {
            data.push(f64::from_be_bytes(chunk.try_into().unwrap()));
        }
        Ok(data)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.len() * 8);
        for chunk in &*self {
            data.extend_from_slice(&chunk.to_be_bytes());
        }
        data
    }
}
