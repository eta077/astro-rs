//! Provides tools to construct, serialize, and deserialize FITS files.

use super::header_value::*;

use std::fmt::Debug;
use std::rc::Rc;

use thiserror::Error;

/// The expected keyword for the first header card of the primary HDU.
pub const SIMPLE_KEYWORD: [u8; 8] = *b"SIMPLE  ";
/// The header keyword indicating the size of each value in the HDU data section.
pub const BITPIX_KEYWORD: [u8; 8] = *b"BITPIX  ";
/// The header keyword indicating how many axes are present in the HDU data section.
pub const NAXIS_KEYWORD: [u8; 8] = *b"NAXIS   ";
/// The header keyword indicating the end of the header section.
pub const END_KEYWORD: [u8; 8] = *b"END     ";
/// The expected keyword for the first header card of each HDU following the primary.
pub const XTENSION_KEYWORD: [u8; 8] = *b"XTENSION";

pub(crate) const FITS_RECORD_LEN: usize = 2880;
pub(crate) const HEADER_CARD_LEN: usize = 80;
pub(crate) const HEADER_KEYWORD_LEN: usize = 8;

/// An enumeration of errors that could occur when processing a FITS header element.
#[derive(Debug, Error)]
pub enum FitsHeaderError {
    /// Indicates an unexpected length of bytes was encountered during processing.
    #[error("unexpected byte count - expected {expected} bytes for {intent}, found {found}")]
    InvalidLength {
        /// The number of bytes expected by the operation.
        expected: usize,
        /// The number of bytes found by the operation.
        found: usize,
        /// The objective of the operation.
        intent: String,
    },
    /// Indicates invalid bytes were encountered during processing.
    #[error("expected valid string for {intent}, found {found:?}")]
    DeserializationError {
        /// The bytes that were found by the operation.
        found: Vec<u8>,
        /// The objective of the operation.
        intent: String,
    },
}

/// The header portion of an HDU.
#[derive(Debug, Default, Clone)]
pub struct FitsHeader {
    /// The card images contained in the header.
    pub cards: Vec<FitsHeaderCard>,
}

impl FitsHeader {
    /// Constructs an empty header.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a FitsHeader from the given bytes.
    pub fn from_bytes(raw: Vec<u8>) -> FitsHeader {
        let raw_len = raw.len();
        let num_cards = raw_len / HEADER_CARD_LEN;

        let mut cards = Vec::with_capacity(num_cards);
        for i in 0..num_cards {
            let index = i * HEADER_CARD_LEN;
            let card_slice: [u8; 80] = raw[index..index + HEADER_CARD_LEN].try_into().unwrap();
            cards.push(FitsHeaderCard::from(card_slice));
        }

        FitsHeader { cards }
    }

    /// Serializes the header into bytes.
    pub fn to_bytes(self) -> Vec<u8> {
        let mut result = Vec::new();
        for card in self.cards {
            let card_raw: [u8; 80] = card.into();
            result.extend_from_slice(&card_raw);
        }
        result
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

/// A card within an HDU header section.
///
/// ```
/// use astro_rs::fits::FitsHeaderCard;
///
/// let card_raw = *b"SIMPLE  =                    T / FITS STANDARD                                  ";
/// let mut card = FitsHeaderCard::from(card_raw);
///
/// assert_eq!(*card.keyword(), "SIMPLE");
/// // deserializes value and comment, discarding padding
/// assert_eq!(*card.get_value::<bool>()?, true);
/// assert_eq!(*card.get_comment()?, String::from("FITS STANDARD"));
///
/// // re-serialize the header card
/// let comparison: [u8; 80] = card.into();
/// assert_eq!(comparison, card_raw);
/// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
/// ```
#[derive(Debug, Clone)]
pub struct FitsHeaderCard {
    keyword: FitsHeaderKeyword,
    value: FitsHeaderValueContainer,
}

impl FitsHeaderCard {
    /// Gets the keyword of the header card.
    pub fn keyword(&self) -> &FitsHeaderKeyword {
        &self.keyword
    }

    /// Gets the value of the header card.
    /// If the value has not yet been deserialized, the deserialization process is attempted.
    /// If the process succeeds, the deserialized value is cached.
    pub fn get_value<T: FitsHeaderValue + 'static>(&mut self) -> Result<Rc<T>, FitsHeaderError> {
        self.value.get_value()
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
        self.value.get_comment()
    }
}

impl From<[u8; 80]> for FitsHeaderCard {
    fn from(raw: [u8; 80]) -> Self {
        let keyword_bytes: [u8; 8] = raw[0..8].try_into().unwrap();
        let keyword = FitsHeaderKeyword::from(keyword_bytes);
        let value_bytes: [u8; 72] = raw[8..80].try_into().unwrap();
        let value = FitsHeaderValueContainer::from(value_bytes);
        FitsHeaderCard { keyword, value }
    }
}

impl From<FitsHeaderCard> for [u8; 80] {
    fn from(card: FitsHeaderCard) -> Self {
        let mut result = [0; 80];
        let keyword_raw: [u8; 8] = card.keyword.into();
        result[0..8].copy_from_slice(&keyword_raw);
        let value_raw: [u8; 72] = card.value.into();
        result[8..80].copy_from_slice(&value_raw);

        result
    }
}

/// A FITS header keyword.
/// This wrapper provides functions to interact with both raw arrays and strings.
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

impl FitsHeaderKeyword {
    /// Appends the given number to the keyword.
    /// If a number is already appended, it is replaced by the given number.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut naxis_keyword = FitsHeaderKeyword::from(NAXIS_KEYWORD);
    /// naxis_keyword.append_number(1);
    /// assert_eq!(naxis_keyword, "NAXIS1");
    /// naxis_keyword.append_number(2);
    /// assert_eq!(naxis_keyword, "NAXIS2");
    ///
    /// let mut tform_keyword = FitsHeaderKeyword::from(TFORM_KEYWORD);
    /// tform_keyword.append_number(100);
    /// assert_eq!(tform_keyword, "TFORM100");
    /// tform_keyword.append_number(10);
    /// assert_eq!(tform_keyword, "TFORM10");
    /// ```
    pub fn append_number(&mut self, number: u16) {
        let mut i = 0;
        while i < 8 {
            let c = self.raw[i];
            if c == b' ' || c.is_ascii_digit() {
                break;
            }
            i += 1;
        }
        if number > 99 {
            self.raw[i] = (number / 100 + 48) as u8;
            i += 1;
        }
        if number > 9 {
            self.raw[i] = (number % 100 / 10 + 48) as u8;
            i += 1;
        }
        self.raw[i] = (number % 10 + 48) as u8;
        i += 1;
        while i < 8 {
            self.raw[i] = b' ';
            i += 1;
        }
    }
}

impl From<[u8; 8]> for FitsHeaderKeyword {
    fn from(raw: [u8; 8]) -> Self {
        FitsHeaderKeyword { raw }
    }
}

impl From<FitsHeaderKeyword> for [u8; 8] {
    fn from(keyword: FitsHeaderKeyword) -> Self {
        keyword.raw
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

impl PartialEq<str> for FitsHeaderKeyword {
    fn eq(&self, other: &str) -> bool {
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

/// A representation of the combined header card value and comment.
/// This wrapper ensures that the total number of bytes between the value and comment will not exceed 72.
#[derive(Debug, Clone)]
pub struct FitsHeaderValueContainer {
    raw: Vec<u8>,
    value: Option<Rc<dyn FitsHeaderValue>>,
    comment: Option<Rc<String>>,
}

impl FitsHeaderValueContainer {
    /// Gets the value of the header card.
    /// If the value has not yet been deserialized, the deserialization process is attempted.
    /// If the process succeeds, the deserialized value is cached.
    pub fn get_value<T: FitsHeaderValue + 'static>(&mut self) -> Result<Rc<T>, FitsHeaderError> {
        if let Some(data) = &self.value {
            unsafe {
                let ptr = Rc::into_raw(Rc::clone(data));
                let new_ptr: *const T = ptr.cast();
                Ok(Rc::from_raw(new_ptr))
            }
        } else {
            let value_start_index = self.raw.iter().position(|b| *b == b'=').unwrap_or(0);
            let comment_start_index = self
                .raw
                .iter()
                .position(|b| *b == b'/')
                .unwrap_or(self.raw.len());
            let mut value_bytes: Vec<u8> = self
                .raw
                .drain(value_start_index..comment_start_index)
                .collect();
            // discard '=' prefix
            value_bytes.remove(0);

            let data = Rc::new(T::from_bytes(Self::trim_value(value_bytes))?);
            let ret = Rc::clone(&data);
            self.value = Some(data);
            Ok(ret)
        }
    }

    /// Gets the comment section of the header card.
    ///
    /// ```
    /// use astro_rs::fits::FitsHeaderValueContainer;
    ///
    /// let mut card_value = FitsHeaderValueContainer::from(*b"=                    T / FITS STANDARD                                  ");
    /// assert_eq!(*card_value.get_comment()?, String::from("FITS STANDARD"));
    /// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
    /// ```
    pub fn get_comment(&mut self) -> Result<Rc<String>, FitsHeaderError> {
        if let Some(data) = &self.comment {
            Ok(Rc::clone(data))
        } else if let Some(comment_start_index) = self.raw.iter().position(|b| *b == b'/') {
            let mut value_bytes: Vec<u8> = self.raw.drain(comment_start_index..).collect();
            // discard '/' prefix
            value_bytes.remove(0);
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
        value
            .iter()
            .position(|b| *b != b' ')
            .map(|index1| {
                let index2 = value
                    .iter()
                    .rposition(|b| *b != b' ')
                    .unwrap_or(value.len())
                    + 1;
                value[index1..index2].to_vec()
            })
            .unwrap_or_default()
    }
}

impl From<[u8; 72]> for FitsHeaderValueContainer {
    fn from(raw: [u8; 72]) -> Self {
        FitsHeaderValueContainer {
            raw: raw.to_vec(),
            value: None,
            comment: None,
        }
    }
}

impl From<FitsHeaderValueContainer> for [u8; 72] {
    fn from(container: FitsHeaderValueContainer) -> Self {
        match (container.value, container.comment) {
            (Some(value), Some(comment)) => {
                let mut result = [b' '; 72];
                result[0] = b'=';
                result[2..72].copy_from_slice(&value.to_bytes());
                let mut comment_start =
                    result.iter().rposition(|b| *b != b' ').unwrap_or_default() + 2;
                result[comment_start] = b'/';
                comment_start += 2;
                let comment_raw = comment.as_bytes();
                result[comment_start..comment_start + comment_raw.len()]
                    .copy_from_slice(comment_raw);
                result
            }
            (Some(value), None) => {
                let mut result = [b' '; 72];
                result[0] = b'=';
                result[2..72].copy_from_slice(&value.to_bytes());
                let comment_start = result.iter().rposition(|b| *b != b' ').unwrap_or_default() + 2;
                let comment_raw = container.raw.as_slice();
                result[comment_start..comment_start + comment_raw.len()]
                    .copy_from_slice(comment_raw);
                result
            }
            (None, Some(comment)) => {
                let mut result = [b' '; 72];
                let value_raw = container.raw.as_slice();
                let mut comment_start = value_raw.len();
                result[0..comment_start].copy_from_slice(value_raw);

                result[comment_start] = b'/';
                comment_start += 2;
                let comment_raw = comment.as_bytes();
                result[comment_start..comment_start + comment_raw.len()]
                    .copy_from_slice(comment_raw);
                result
            }
            (None, None) => {
                let result: [u8; 72] = container.raw[0..72].try_into().unwrap();
                result
            }
        }
    }
}
