//! Provides tools to construct, serialize, and deserialize FITS files.

use std::fmt::Debug;
use std::rc::Rc;

use thiserror::Error;

/// The expected keyword for the first header card of the primary HDU.
pub const SIMPLE_KEYWORD: [u8; 8] = *b"SIMPLE  ";
/// The header keyword indicating the size of each value in the HDU data.
pub const BITPIX_KEYWORD: [u8; 8] = *b"BITPIX  ";
/// The header keyword indicating how many axes are present in the HDU data.
pub const NAXIS_KEYWORD: [u8; 8] = *b"NAXIS   ";
/// The header keyword indicating the end of the header section.
pub const END_KEYWORD: [u8; 8] = *b"END     ";
/// The expected keyword for the first header card of each HDU following the primary.
pub const XTENSION_KEYWORD: [u8; 8] = *b"XTENSION";

const BLANK_KEYWORD: [u8; 8] = *b"        ";

const HEADER_CARD_LEN: usize = 80;
const HEADER_KEYWORD_LEN: usize = 8;

#[derive(Debug, Error)]
pub enum FitsHeaderError {
    #[error("unexpected byte count - expected {expected} bytes for {intent}, found {found}")]
    InvalidLength {
        expected: usize,
        found: usize,
        intent: String,
    },
    #[error("expected valid string for {intent}, found {found:?}")]
    DeserializationError { found: Vec<u8>, intent: String },
}

/// A representation of the entirety of a FITS file.
#[derive(Debug, Default)]
pub struct HduList {
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
                    // drain padding to reach data
                    while let Some(b) = raw.first() {
                        match *b {
                            b' ' => {
                                raw.remove(0);
                            }
                            _ => break,
                        }
                    }
                    break;
                }
                new_header_bytes = HduList::drain_header_bytes(&mut raw, hdus.len())?;
                cards_read = new_header_bytes.len() / HEADER_CARD_LEN;
                header_raw.append(&mut new_header_bytes);
            }

            let mut header = FitsHeader::from_bytes(header_raw)?;
            let mut data = Vec::new();

            let naxis = *header
                .get_card(NAXIS_KEYWORD)
                .and_then(|card| card.get_value::<u16>().ok())
                .unwrap_or_default();
            if naxis > 0 {
                let bitpix = header
                    .get_card(BITPIX_KEYWORD)
                    .and_then(|card| card.get_value::<Bitpix>().ok())
                    .map(|bitpix| bitpix.value())
                    .unwrap_or_default();
                if bitpix > 0 {
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
                    data_len *= bitpix / 8;
                    data = raw.drain(0..data_len.clamp(data_len, raw.len())).collect();
                    // drain padding to reach next header
                    while let Some(b) = raw.first() {
                        match *b {
                            0 | b' ' => {
                                raw.remove(0);
                            }
                            _ => break,
                        }
                    }
                }
            }
            hdus.push(Hdu { header, data });
        }
        Ok(HduList { hdus })
    }

    /// Validates the existence and format of the SIMPLE header card.
    pub fn is_header_valid(&mut self) -> Result<bool, FitsHeaderError> {
        Ok(true)
    }

    fn drain_header_bytes(raw: &mut Vec<u8>, hdu_num: usize) -> Result<Vec<u8>, FitsHeaderError> {
        let raw_len = raw.len();
        let block_len = if raw_len < HEADER_CARD_LEN {
            return Err(FitsHeaderError::InvalidLength {
                expected: HEADER_CARD_LEN,
                found: raw_len,
                intent: ["HDU ", &hdu_num.to_string(), " header"].concat(),
            });
        } else {
            HEADER_CARD_LEN
        };
        Ok(raw.drain(0..block_len).collect())
    }
}

/// A representation of a Header Data Unit within a FITS file.
#[derive(Debug, Default)]
pub struct Hdu {
    pub header: FitsHeader,
    data: Vec<u8>,
}

impl Hdu {
    /// Constructs an empty HDU.
    pub fn new() -> Self {
        Self::default()
    }
}

/// A representation of the header portion of an HDU.
#[derive(Debug, Default)]
pub struct FitsHeader {
    cards: Vec<FitsHeaderCard>,
}

impl FitsHeader {
    /// Constructs an empty header.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a FitsHeader from the given bytes.
    pub fn from_bytes(mut raw: Vec<u8>) -> Result<FitsHeader, FitsHeaderError> {
        let raw_len = raw.len();
        let num_cards = raw_len / HEADER_CARD_LEN;
        if raw_len % HEADER_CARD_LEN != 0 {
            return Err(FitsHeaderError::InvalidLength {
                expected: (num_cards + 1) * HEADER_CARD_LEN,
                found: raw_len,
                intent: String::from("FITS header"),
            });
        }

        let mut cards = Vec::with_capacity(num_cards);
        while !raw.is_empty() {
            let card_vec = raw.drain(0..HEADER_CARD_LEN).collect::<Vec<u8>>();
            let card_slice: [u8; 80] = card_vec[0..80].try_into().unwrap();
            cards.push(FitsHeaderCard::from(card_slice));
        }

        Ok(FitsHeader { cards })
    }

    /// Searches the header cards for a match with the given keyword.
    pub fn get_card<K>(&mut self, keyword: K) -> Option<&mut FitsHeaderCard>
    where
        FitsHeaderKeyword: PartialEq<K>,
    {
        for card in self.cards.iter_mut() {
            if card.keyword == keyword {
                return Some(card);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct FitsHeaderCard {
    raw: Vec<u8>,
    keyword: FitsHeaderKeyword,
    value: Option<Rc<dyn FitsHeaderValue>>,
    comment: Option<Rc<String>>,
}

impl FitsHeaderCard {
    pub fn get_value<T: FitsHeaderValue + 'static>(&mut self) -> Result<Rc<T>, FitsHeaderError> {
        if let Some(data) = &self.value {
            unsafe {
                let ptr = Rc::into_raw(Rc::clone(data));
                let new_ptr: *const T = ptr.cast();
                Ok(Rc::from_raw(new_ptr))
            }
        } else {
            let value_start_index = self
                .raw
                .iter()
                .position(|b| *b == b'=')
                .map(|index| index + 1)
                .unwrap_or(0);
            let comment_start_index = self
                .raw
                .iter()
                .position(|b| *b == b'/')
                .unwrap_or(self.raw.len());
            let value_bytes = self
                .raw
                .drain(value_start_index..comment_start_index)
                .collect();

            let data = Rc::new(T::from_bytes(Self::trim_value(value_bytes))?);
            let ret = Rc::clone(&data);
            self.value = Some(data);
            Ok(ret)
        }
    }

    /// Gets the comment section of the header card.
    ///
    /// ```
    /// use astro_rs::fits::FitsHeaderCard;
    ///
    /// let mut card = FitsHeaderCard::from(*b"SIMPLE  =                    T / FITS STANDARD                                  ");
    /// assert_eq!(*card.get_comment()?, String::from("FITS STANDARD"));
    /// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
    /// ```
    pub fn get_comment(&mut self) -> Result<Rc<String>, FitsHeaderError> {
        if let Some(data) = &self.comment {
            Ok(Rc::clone(data))
        } else if let Some(comment_start_index) = self
            .raw
            .iter()
            .position(|b| *b == b'/')
            .map(|index| index + 1)
        {
            let value_bytes = self.raw.drain(comment_start_index..).collect();
            let value_string = String::from_utf8(Self::trim_value(value_bytes)).map_err(|er| {
                FitsHeaderError::DeserializationError {
                    found: er.into_bytes(),
                    intent: String::from("header card comment"),
                }
            })?;
            let value = Rc::new(value_string);
            let ret = Rc::clone(&value);
            self.comment = Some(value);

            Ok(ret)
        } else {
            Ok(Default::default())
        }
    }

    fn trim_value(value: Vec<u8>) -> Vec<u8> {
        let mut index1 = 0;
        let mut index2 = value.len();
        for b in value.iter() {
            match *b {
                b' ' => index1 += 1,
                _ => break,
            }
        }
        if index1 == value.len() {
            return Vec::new();
        }
        for b in value.iter().rev() {
            match *b {
                b' ' => index2 -= 1,
                _ => break,
            }
        }
        value[index1..index2].to_vec()
    }
}

impl From<[u8; 80]> for FitsHeaderCard {
    fn from(raw: [u8; 80]) -> Self {
        let keyword_bytes: [u8; 8] = raw[0..8].try_into().unwrap();
        let keyword = FitsHeaderKeyword::from(keyword_bytes);
        FitsHeaderCard {
            raw: raw[8..].to_vec(),
            keyword,
            value: None,
            comment: None,
        }
    }
}

/// A representation of a FITS header keyword.
///
/// ```
/// use astro_rs::fits::FitsHeaderKeyword;
///
/// let simple_keyword = FitsHeaderKeyword::from(*b"SIMPLE  ");
/// assert!(simple_keyword == "SIMPLE");
/// assert!(simple_keyword == *b"SIMPLE  ");
///
/// assert!(simple_keyword != "BITPIX");
/// assert!(simple_keyword != *b"BITPIX  ");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FitsHeaderKeyword {
    raw: [u8; 8],
}

impl From<[u8; 8]> for FitsHeaderKeyword {
    fn from(raw: [u8; 8]) -> Self {
        FitsHeaderKeyword { raw }
    }
}

impl PartialEq<&str> for FitsHeaderKeyword {
    fn eq(&self, other: &&str) -> bool {
        if other.len() > HEADER_KEYWORD_LEN {
            return false;
        }
        let other_bytes = other.as_bytes();
        for (index, b) in self.raw.iter().enumerate() {
            if b != other_bytes.get(index).unwrap_or(&b' ') {
                return false;
            }
        }

        true
    }
}

impl PartialEq<[u8; 8]> for FitsHeaderKeyword {
    fn eq(&self, other: &[u8; 8]) -> bool {
        self.raw == *other
    }
}

/// A trait that allows data to be serialized/deserialized as a FITS header value.
pub trait FitsHeaderValue: Debug {
    /// Attempts to deserialize a value from the given bytes. The given bytes shall not be padded by spaces.
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError>
    where
        Self: Sized;

    /// Serializes the value to bytes. The bytes shall not include padding spaces.
    fn to_bytes(self) -> Vec<u8>;
}

impl FitsHeaderValue for u8 {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        if raw.len() == 1 {
            Ok(*raw.first().unwrap() - 48)
        } else {
            Err(FitsHeaderError::InvalidLength {
                expected: 1,
                found: raw.len(),
                intent: String::from("header card u8 value"),
            })
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![self + 48]
    }
}

impl FitsHeaderValue for u16 {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let value_string =
            String::from_utf8(raw).map_err(|er| FitsHeaderError::DeserializationError {
                found: er.into_bytes(),
                intent: String::from("header card u16 value"),
            })?;
        value_string
            .parse()
            .map_err(|_| FitsHeaderError::DeserializationError {
                found: value_string.into_bytes(),
                intent: String::from("header card u16 value"),
            })
    }

    fn to_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl FitsHeaderValue for u32 {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let value_string =
            String::from_utf8(raw).map_err(|er| FitsHeaderError::DeserializationError {
                found: er.into_bytes(),
                intent: String::from("header card u32 value"),
            })?;
        value_string
            .parse()
            .map_err(|_| FitsHeaderError::DeserializationError {
                found: value_string.into_bytes(),
                intent: String::from("header card u32 value"),
            })
    }

    fn to_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl FitsHeaderValue for String {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        String::from_utf8(raw).map_err(|er| FitsHeaderError::DeserializationError {
            found: er.into_bytes(),
            intent: String::from("header card u16 value"),
        })
    }

    fn to_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

/// An enumeration of valid values corresponding to the BITPIX keyword.
#[derive(Debug, Clone, Copy)]
pub enum Bitpix {
    U8,
    I16,
    I32,
    F32,
    F64,
}

impl Bitpix {
    /// Gets the number of bits that represent a value in the data section of the HDU.
    pub fn value(&self) -> usize {
        match self {
            Bitpix::U8 => 8,
            Bitpix::I16 => 16,
            Bitpix::I32 => 32,
            Bitpix::F32 => 32,
            Bitpix::F64 => 64,
        }
    }
}

impl FitsHeaderValue for Bitpix {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        match raw.as_slice() {
            b"8" => Ok(Bitpix::U8),
            b"16" => Ok(Bitpix::I16),
            b"32" => Ok(Bitpix::I32),
            b"-32" => Ok(Bitpix::F32),
            b"-64" => Ok(Bitpix::F64),
            _ => Err(FitsHeaderError::DeserializationError {
                found: raw,
                intent: String::from("header card bitpix value"),
            }),
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        match self {
            Bitpix::U8 => b"8".to_vec(),
            Bitpix::I16 => b"16".to_vec(),
            Bitpix::I32 => b"32".to_vec(),
            Bitpix::F32 => b"-32".to_vec(),
            Bitpix::F64 => b"-64".to_vec(),
        }
    }
}
