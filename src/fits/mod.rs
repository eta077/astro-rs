//! Serialize and deserialize FITS data.
//! See https://archive.stsci.edu/fits/fits_standard/fits_standard.html for the FITS API.

pub mod hdu_types;
mod header;
mod header_value;

use std::fmt::Debug;
use std::rc::Rc;

pub use header::*;
pub use header_value::*;

const BLANK_KEYWORD: [u8; 8] = *b"        ";

/// A representation of the entirety of a FITS file.
#[derive(Debug, Default)]
pub struct HduList {
    /// The HDUs that compose the HduList.
    pub hdus: Vec<Hdu>,
}

impl HduList {
    /// Constructs an empty HduList.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs an HduList from the given bytes.
    pub fn from_bytes(mut raw: Vec<u8>) -> Result<HduList, FitsHeaderError> {
        let mut hdus = Vec::new();

        while !raw.is_empty() {
            let mut header_raw = Vec::new();
            let mut new_header_bytes = HduList::drain_header_bytes(&mut raw, hdus.len())?;
            let mut cards_read = new_header_bytes.len() / HEADER_CARD_LEN;
            header_raw.append(&mut new_header_bytes);

            // search for the END keyword.
            // this should be the last keyword in the header, so if something other than ' ' is found, stop searching
            loop {
                let mut end_found = false;
                for card in 1..=cards_read {
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
                new_header_bytes = HduList::drain_header_bytes(&mut raw, hdus.len())?;
                cards_read = new_header_bytes.len() / HEADER_CARD_LEN;
                header_raw.append(&mut new_header_bytes);
            }

            let mut header = FitsHeader::from_bytes(header_raw)?;
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
                    data_raw = raw.drain(0..data_len.clamp(0, raw.len())).collect();
                }
            }
            hdus.push(Hdu {
                header,
                data_raw,
                ..Default::default()
            });
        }
        Ok(HduList { hdus })
    }

    /// Validates the existence and format of the SIMPLE header card.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu_list = HduList::new();
    /// // empty header
    /// assert!(!hdu_list.is_header_valid()?);
    ///
    /// let mut hdu = Hdu::new();
    ///
    /// // non-empty header missing simple card
    /// let bitpix_card = FitsHeaderCard::from(*b"BITPIX  =                  -32 / FITS BITS/PIXEL                                ");
    /// hdu.header.insert_card(0, bitpix_card);
    /// assert!(!hdu_list.is_header_valid()?);
    ///
    /// // valid header
    /// let simple_card = FitsHeaderCard::from(*b"SIMPLE  =                    T / FITS STANDARD                                  ");
    /// hdu.header.insert_card(0, simple_card);
    /// hdu_list.hdus.push(hdu);
    /// assert!(hdu_list.is_header_valid()?);
    /// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
    /// ```
    pub fn is_header_valid(&mut self) -> Result<bool, FitsHeaderError> {
        Ok(*self
            .hdus
            .first_mut()
            .and_then(|hdu| hdu.header.get_card(SIMPLE_KEYWORD))
            .and_then(|card| card.get_value::<bool>().ok())
            .unwrap_or_default())
    }

    /// Serializes the HduList to bytes.
    pub fn to_bytes(self) -> Vec<u8> {
        let mut result = Vec::new();
        for hdu in self.hdus {
            result.append(&mut hdu.to_bytes());
        }
        result
    }

    fn drain_header_bytes(raw: &mut Vec<u8>, hdu_num: usize) -> Result<Vec<u8>, FitsHeaderError> {
        let raw_len = raw.len();
        if raw_len < FITS_RECORD_LEN {
            return Err(FitsHeaderError::InvalidLength {
                expected: FITS_RECORD_LEN,
                found: raw_len,
                intent: ["HDU ", &hdu_num.to_string(), " header"].concat(),
            });
        }
        Ok(raw.drain(0..FITS_RECORD_LEN).collect())
    }
}

/// A Header Data Unit within a FITS file.
#[derive(Debug, Default)]
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
