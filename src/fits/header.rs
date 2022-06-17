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
    ///
    /// ```
    /// use astro_rs::fits::*;
    /// use std::rc::Rc;
    ///
    /// // default primary HDU header bytes
    /// let bytes = *b"SIMPLE  =                    T                                                  BITPIX  =                    8                                                  NAXIS   =                    0                                                  END                                                                             ";
    /// let mut header = FitsHeader::from_bytes(bytes.to_vec());
    ///
    /// assert!(*header
    ///     .get_card(SIMPLE_KEYWORD)
    ///     .and_then(|card| card.get_value::<bool>().ok())
    ///     .unwrap_or_default());
    /// assert_eq!(
    ///     header
    ///         .get_card(BITPIX_KEYWORD)
    ///         .and_then(|card| card.get_value::<Bitpix>().ok()),
    ///     Some(Rc::new(Bitpix::U8))
    /// );
    /// assert_eq!(
    ///     header
    ///         .get_card(NAXIS_KEYWORD)
    ///         .and_then(|card| card.get_value::<u16>().ok()),
    ///     Some(Rc::new(0))
    /// );
    /// assert!(header.get_card(END_KEYWORD).is_some());
    /// ```
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
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let hdu = primary_hdu::default();
    /// let mut bytes = b"SIMPLE  =                    T                                                  BITPIX  =                    8                                                  NAXIS   =                    0                                                  END                                                                             ".to_vec();
    /// bytes.resize(2880, b' ');
    ///
    /// assert_eq!(hdu.header.to_bytes(), bytes);
    /// ```
    pub fn to_bytes(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(FITS_RECORD_LEN);
        let filled_cards = self.cards.len();
        for card in self.cards {
            let card_raw: [u8; HEADER_CARD_LEN] = card.into();
            result.extend_from_slice(&card_raw);
        }
        if filled_cards < 36 {
            result.resize(FITS_RECORD_LEN, b' ');
        }
        result
    }

    /// Searches the header cards for a match with the given keyword.
    ///
    /// ```
    /// use astro_rs::fits::*;
    ///
    /// let mut hdu = primary_hdu::default();
    /// assert!(hdu.header.get_card(SIMPLE_KEYWORD).is_some());
    /// assert!(hdu.header.get_card(EXTNAME_KEYWORD).is_none());
    /// ```
    pub fn get_card<K: PartialEq<FitsHeaderKeyword>>(
        &mut self,
        keyword: K,
    ) -> Option<&mut FitsHeaderCard> {
        for card in self.cards.iter_mut() {
            if keyword == card.keyword {
                return Some(card);
            }
        }
        None
    }

    /// Sets the value and comment of the card with the given keyword.
    /// If a card already exists, the data is overwritten.
    /// If a card does not exist, one is created.
    ///
    /// ```
    /// use astro_rs::fits::*;
    /// use std::rc::Rc;
    ///
    /// let mut header = FitsHeader::new();
    /// header.set_card(SIMPLE_KEYWORD, true, None);
    /// assert!(*header
    ///     .get_card(SIMPLE_KEYWORD)
    ///     .and_then(|card| card.get_value::<bool>().ok())
    ///     .unwrap_or_default());
    ///
    /// header.set_card(SIMPLE_KEYWORD, false, Some(String::from("FITS STANDARD")));
    /// let mut card = header.get_card(SIMPLE_KEYWORD).unwrap();
    /// assert!(!*card.get_value::<bool>()?);
    /// assert_eq!(card.get_comment()?, Rc::new(String::from("FITS STANDARD")));
    /// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
    /// ```
    pub fn set_card<
        K: PartialEq<FitsHeaderKeyword> + Into<FitsHeaderKeyword>,
        T: FitsHeaderValue + 'static,
    >(
        &mut self,
        keyword: K,
        value: T,
        comment: Option<String>,
    ) -> Result<(), FitsHeaderError> {
        let fits_keyword = keyword.into();
        let new_card = FitsHeaderCard {
            keyword: fits_keyword,
            value: FitsHeaderValueContainer::new(value, comment)?,
        };
        if let Some(card) = self.get_card(fits_keyword) {
            *card = new_card;
        } else {
            let index = if self
                .cards
                .last()
                .map(|card| card.keyword == END_KEYWORD)
                .unwrap_or_default()
            {
                self.cards.len() - 1
            } else {
                self.cards.len()
            };
            self.cards.insert(index, new_card);
        }
        Ok(())
    }

    /// Sets the value of the card with the given keyword.
    /// If a card already exists, the value is overwritten, and the comment is retained.
    /// If a card does not exist, one is created.
    ///
    /// ```
    /// use astro_rs::fits::*;
    /// use std::rc::Rc;
    ///
    /// let bytes = *b"SIMPLE  =                    T / FITS STANDARD                                  ";
    /// let mut header = FitsHeader::from_bytes(bytes.to_vec());
    /// header.set_value(SIMPLE_KEYWORD, false)?;
    /// let mut card = header.get_card(SIMPLE_KEYWORD).unwrap();
    /// assert!(!*card.get_value::<bool>()?);
    /// assert_eq!(card.get_comment()?, Rc::new(String::from("FITS STANDARD")));
    ///
    /// header.set_value(BITPIX_KEYWORD, Bitpix::U8)?;
    /// assert_eq!(
    ///     header
    ///         .get_card(BITPIX_KEYWORD)
    ///         .and_then(|card| card.get_value::<Bitpix>().ok()),
    ///     Some(Rc::new(Bitpix::U8))
    /// );
    /// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
    /// ```
    pub fn set_value<K, T: FitsHeaderValue + 'static>(
        &mut self,
        keyword: K,
        value: T,
    ) -> Result<(), FitsHeaderError>
    where
        K: PartialEq<FitsHeaderKeyword> + Into<FitsHeaderKeyword>,
    {
        let fits_keyword = keyword.into();
        if let Some(card) = self.get_card(fits_keyword) {
            card.value.set_value(value)?;
        } else {
            let new_card = FitsHeaderCard {
                keyword: fits_keyword,
                value: FitsHeaderValueContainer::new(value, None)?,
            };
            let index = if self
                .cards
                .last()
                .map(|card| card.keyword == END_KEYWORD)
                .unwrap_or_default()
            {
                self.cards.len() - 1
            } else {
                self.cards.len()
            };
            self.cards.insert(index, new_card);
        }
        Ok(())
    }

    /// Sets the comment of the card with the given keyword.
    /// If a card already exists, the comment is overwritten, and the value is retained.
    /// If a card does not exist, this function has no effect.
    ///
    /// ```
    /// use astro_rs::fits::*;
    /// use std::rc::Rc;
    ///
    /// let mut hdu = primary_hdu::default();
    /// hdu.header.set_comment(SIMPLE_KEYWORD, Some(String::from("FITS STANDARD")));
    /// let mut card = hdu.header.get_card(SIMPLE_KEYWORD).unwrap();
    /// assert!(*card.get_value::<bool>()?);
    /// assert_eq!(card.get_comment()?, Rc::new(String::from("FITS STANDARD")));
    ///
    /// hdu.header.set_comment(EXTNAME_KEYWORD, Some(String::from("Error 404")));
    /// assert!(hdu.header.get_card(EXTNAME_KEYWORD).is_none());
    /// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
    /// ```
    pub fn set_comment<K: PartialEq<FitsHeaderKeyword>>(
        &mut self,
        keyword: K,
        comment: Option<String>,
    ) -> Result<(), FitsHeaderError> {
        if let Some(card) = self.get_card(keyword) {
            card.value.set_comment(comment)?;
        }
        Ok(())
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
        let keyword_bytes: [u8; 8] = raw[0..HEADER_KEYWORD_LEN].try_into().unwrap();
        let keyword = FitsHeaderKeyword::from(keyword_bytes);
        let value_bytes: [u8; 72] = raw[HEADER_KEYWORD_LEN..HEADER_CARD_LEN].try_into().unwrap();
        let value = FitsHeaderValueContainer::from(value_bytes);
        FitsHeaderCard { keyword, value }
    }
}

impl From<FitsHeaderCard> for [u8; 80] {
    fn from(card: FitsHeaderCard) -> Self {
        let mut result = [0; HEADER_CARD_LEN];
        let keyword_raw: [u8; HEADER_KEYWORD_LEN] = card.keyword.into();
        result[0..HEADER_KEYWORD_LEN].copy_from_slice(&keyword_raw);
        let value_raw: [u8; 72] = card.value.into();
        result[HEADER_KEYWORD_LEN..HEADER_CARD_LEN].copy_from_slice(&value_raw);

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

impl PartialEq<FitsHeaderKeyword> for [u8; 8] {
    fn eq(&self, other: &FitsHeaderKeyword) -> bool {
        *self == other.raw
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
    /// Constructs a new FitsHeaderValueContainer with the given value and comment.
    pub fn new<T: FitsHeaderValue + 'static>(
        value: T,
        comment: Option<String>,
    ) -> Result<Self, FitsHeaderError> {
        Self::check_comment_length(value.to_bytes(), comment.as_ref())?;
        Ok(FitsHeaderValueContainer {
            raw: Vec::new(),
            value: Some(Rc::new(value)),
            comment: comment.map(Rc::new),
        })
    }

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

    /// Sets the value of the header card.
    pub fn set_value<T: FitsHeaderValue + 'static>(
        &mut self,
        value: T,
    ) -> Result<(), FitsHeaderError> {
        let comment = match (self.value.as_ref(), self.comment.as_ref()) {
            (None, None) => {
                let comment = self.get_comment()?.to_string();
                self.raw.clear();
                comment
            }
            (None, Some(comment)) => {
                self.raw.clear();
                comment.to_string()
            }
            (Some(_), None) => self.get_comment()?.to_string(),
            (Some(_), Some(comment)) => comment.to_string(),
        };
        Self::check_comment_length(value.to_bytes(), Some(&comment))?;
        self.value = Some(Rc::new(value));
        Ok(())
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
        } else if let Some(comment_start_index) = self
            .raw
            .iter()
            .position(|b| *b == b'/')
            .or_else(|| self.raw.iter().rposition(|b| *b != b' ').map(|idx| idx + 1))
        {
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

    /// Sets the comment section of the header card.
    pub fn set_comment(&mut self, comment: Option<String>) -> Result<(), FitsHeaderError> {
        let value_raw = match (self.value.as_ref(), self.comment.as_ref()) {
            (Some(value), Some(_comment)) => value.to_bytes(),
            (Some(value), None) => {
                self.raw.clear();
                value.to_bytes()
            }
            (None, Some(_comment)) => {
                let mut value_raw = [b' '; 70];
                let idx_diff = if self.raw.len() > 70 {
                    self.raw.len() - 70
                } else {
                    0
                };
                for i in idx_diff..self.raw.len() {
                    value_raw[i - idx_diff] = self.raw[i];
                }
                value_raw
            }
            (None, None) => {
                self.get_comment()?;
                let mut value_raw = [b' '; 70];
                let idx_diff = if self.raw.len() > 70 {
                    self.raw.len() - 70
                } else {
                    0
                };
                for i in idx_diff..self.raw.len() {
                    value_raw[i - idx_diff] = self.raw[i];
                }
                value_raw
            }
        };
        Self::check_comment_length(value_raw, comment.as_ref())?;
        self.comment = comment.map(Rc::new);
        Ok(())
    }

    fn check_comment_length(
        value_raw: [u8; 70],
        comment: Option<&String>,
    ) -> Result<(), FitsHeaderError> {
        if let Some(comment_str) = comment {
            let comment_start = value_raw
                .iter()
                .rposition(|b| *b != b' ')
                .unwrap_or_default();
            let diff = 68_usize.checked_sub(comment_start).unwrap_or_default(); // minus an additional 2 for the delimiter
            if diff < comment_str.len() {
                return Err(FitsHeaderError::InvalidLength {
                    expected: diff,
                    found: comment_str.len(),
                    intent: String::from("header card comment"),
                });
            }
        }
        Ok(())
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
                comment_start += 1;
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
