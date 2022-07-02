mod bitpix;
mod tform;

pub use bitpix::*;
pub use tform::*;

use std::any::{Any, TypeId};
use std::fmt::Debug;

use super::header::FitsHeaderError;

/// A trait that allows data to be serialized/deserialized as a FITS header value.
pub trait FitsHeaderValue: Debug + RealAny {
    /// Attempts to deserialize a value from the given bytes. The given bytes shall not be padded by spaces.
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError>
    where
        Self: Sized;

    /// Serializes the value to bytes. The bytes shall include padding spaces.
    fn to_bytes(&self) -> [u8; 70];
}

// credit to https://github.com/chris-morgan/mopa for this solution
impl dyn FitsHeaderValue {
    /// Determines if the type of `self` is the same as `T`.
    pub fn is<T: FitsHeaderValue + 'static>(&self) -> bool {
        TypeId::of::<T>() == RealAny::real_type_id(self)
    }
}

/// A trait used to get the real type ID for implementors of `FitsHeaderValue`.
pub trait RealAny {
    /// Gets the base type ID for `self`.
    fn real_type_id(&self) -> TypeId;
}

impl<T: Any> RealAny for T {
    fn real_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

/// ```
/// use astro_rs::fits::FitsHeaderError;
/// use astro_rs::fits::FitsHeaderValue;
///
/// // successful deserialization
/// let true_value: bool = FitsHeaderValue::from_bytes(b"T".to_vec())?;
/// assert!(true_value);
/// let false_value: bool = FitsHeaderValue::from_bytes(b"F".to_vec())?;
/// assert!(!false_value);
///
/// // failed deserialization
/// let result: Result<bool, FitsHeaderError> = FitsHeaderValue::from_bytes(b"A".to_vec());
/// assert!(result.is_err());
/// let result: Result<bool, FitsHeaderError> = FitsHeaderValue::from_bytes(b"true".to_vec());
/// assert!(result.is_err());
///
/// // serialization
/// assert_eq!(true_value.to_bytes(), *b"                   T                                                  ");
/// assert_eq!(false_value.to_bytes(), *b"                   F                                                  ");
/// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
/// ```
impl FitsHeaderValue for bool {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        if raw.len() == 1 {
            match *raw.first().unwrap() {
                b'T' => Ok(true),
                b'F' => Ok(false),
                _ => Err(FitsHeaderError::DeserializationError {
                    found: raw,
                    intent: String::from("header card bool value"),
                }),
            }
        } else {
            Err(FitsHeaderError::InvalidLength {
                expected: 1,
                found: raw.len(),
                intent: String::from("header card u8 value"),
            })
        }
    }

    fn to_bytes(&self) -> [u8; 70] {
        let mut result = [b' '; 70];
        if *self {
            result[19] = b'T';
        } else {
            result[19] = b'F';
        }
        result
    }
}

/// ```
/// use astro_rs::fits::FitsHeaderError;
/// use astro_rs::fits::FitsHeaderValue;
///
/// // successful deserialization
/// let max_value: u8 = FitsHeaderValue::from_bytes(b"255".to_vec())?;
/// assert_eq!(max_value, 255);
/// let min_value: u8 = FitsHeaderValue::from_bytes(b"0".to_vec())?;
/// assert_eq!(min_value, 0);
///
/// // failed deserialization
/// let result: Result<u8, FitsHeaderError> = FitsHeaderValue::from_bytes(b"300".to_vec());
/// assert!(result.is_err());
/// let result: Result<u8, FitsHeaderError> = FitsHeaderValue::from_bytes(b"Not a number".to_vec());
/// assert!(result.is_err());
///
/// // serialization
/// assert_eq!(max_value.to_bytes(), *b"                 255                                                  ");
/// assert_eq!(min_value.to_bytes(), *b"                   0                                                  ");
/// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
/// ```
impl FitsHeaderValue for u8 {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let value_string =
            String::from_utf8(raw).map_err(|er| FitsHeaderError::DeserializationError {
                found: er.into_bytes(),
                intent: String::from("header card u8 value"),
            })?;
        value_string
            .parse()
            .map_err(|_| FitsHeaderError::DeserializationError {
                found: value_string.into_bytes(),
                intent: String::from("header card u8 value"),
            })
    }

    fn to_bytes(&self) -> [u8; 70] {
        let mut result = [b' '; 70];
        let value_raw = self.to_string().into_bytes();
        let start = 20 - value_raw.len();
        for (i, b) in value_raw.iter().enumerate() {
            result[start + i] = *b;
        }
        result
    }
}

/// ```
/// use astro_rs::fits::FitsHeaderError;
/// use astro_rs::fits::FitsHeaderValue;
///
/// // successful deserialization
/// let max_value: u16 = FitsHeaderValue::from_bytes(b"65535".to_vec())?;
/// assert_eq!(max_value, 65535);
/// let min_value: u16 = FitsHeaderValue::from_bytes(b"0".to_vec())?;
/// assert_eq!(min_value, 0);
///
/// // failed deserialization
/// let result: Result<u16, FitsHeaderError> = FitsHeaderValue::from_bytes(b"66000".to_vec());
/// assert!(result.is_err());
/// let result: Result<u16, FitsHeaderError> = FitsHeaderValue::from_bytes(b"Not a number".to_vec());
/// assert!(result.is_err());
///
/// // serialization
/// assert_eq!(max_value.to_bytes(), *b"               65535                                                  ");
/// assert_eq!(min_value.to_bytes(), *b"                   0                                                  ");
/// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
/// ```
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

    fn to_bytes(&self) -> [u8; 70] {
        let mut result = [b' '; 70];
        let value_raw = self.to_string().into_bytes();
        let start = 20 - value_raw.len();
        for (i, b) in value_raw.iter().enumerate() {
            result[start + i] = *b;
        }
        result
    }
}

/// ```
/// use astro_rs::fits::FitsHeaderError;
/// use astro_rs::fits::FitsHeaderValue;
///
/// // successful deserialization
/// let max_value: u32 = FitsHeaderValue::from_bytes(b"4294967295".to_vec())?;
/// assert_eq!(max_value, 4294967295);
/// let min_value: u32 = FitsHeaderValue::from_bytes(b"0".to_vec())?;
/// assert_eq!(min_value, 0);
///
/// // failed deserialization
/// let result: Result<u32, FitsHeaderError> = FitsHeaderValue::from_bytes(b"4300000000".to_vec());
/// assert!(result.is_err());
/// let result: Result<u32, FitsHeaderError> = FitsHeaderValue::from_bytes(b"Not a number".to_vec());
/// assert!(result.is_err());
///
/// // serialization
/// assert_eq!(max_value.to_bytes(), *b"          4294967295                                                  ");
/// assert_eq!(min_value.to_bytes(), *b"                   0                                                  ");
/// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
/// ```
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

    fn to_bytes(&self) -> [u8; 70] {
        let mut result = [b' '; 70];
        let value_raw = self.to_string().into_bytes();
        let start = 20 - value_raw.len();
        for (i, b) in value_raw.iter().enumerate() {
            result[start + i] = *b;
        }
        result
    }
}

/// ```
/// use astro_rs::fits::FitsHeaderError;
/// use astro_rs::fits::FitsHeaderValue;
///
/// // successful deserialization
/// let value: String = FitsHeaderValue::from_bytes(String::from("hello world").into_bytes())?;
/// assert_eq!(value, String::from("hello world"));
/// let quote_value: String = FitsHeaderValue::from_bytes(String::from("'this ''includes'' quotes'").into_bytes())?;
/// assert_eq!(quote_value, String::from("this 'includes' quotes"));
///
/// // failed deserialization
/// let result: Result<String, FitsHeaderError> = FitsHeaderValue::from_bytes(vec![0, 159, 146, 150]);
/// assert!(result.is_err());
///
/// // serialization
/// assert_eq!(value.to_bytes(), *b"'hello world'                                                         ");
/// assert_eq!(quote_value.to_bytes(), *b"'this ''includes'' quotes'                                            ");
/// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
/// ```
impl FitsHeaderValue for String {
    fn from_bytes(mut raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let mut remove_quote = true;
        let mut i = 0;
        while i < raw.len() {
            if raw[i] == b'\'' {
                if remove_quote {
                    raw.remove(i);
                } else {
                    i += 1;
                }
                remove_quote = false;
            } else {
                remove_quote = true;
                i += 1;
            }
        }
        let value = String::from_utf8(raw).map_err(|er| FitsHeaderError::DeserializationError {
            found: er.into_bytes(),
            intent: String::from("header card String value"),
        })?;
        Ok(value.trim().to_owned())
    }

    fn to_bytes(&self) -> [u8; 70] {
        let mut result = [b' '; 70];
        result[0] = b'\'';
        let mut num_quotes = 1;
        let value_raw = self.as_bytes();
        for (i, b) in value_raw.iter().enumerate() {
            if *b == b'\'' {
                result[i + num_quotes] = b'\'';
                num_quotes += 1;
            }
            result[i + num_quotes] = *b;
        }
        result[value_raw.len() + num_quotes] = b'\'';
        result
    }
}
