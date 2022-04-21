use std::fmt::Debug;

use super::header::FitsHeaderError;

/// A trait that allows data to be serialized/deserialized as a FITS header value.
pub trait FitsHeaderValue: Debug {
    /// Attempts to deserialize a value from the given bytes. The given bytes shall not be padded by spaces.
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError>
    where
        Self: Sized;

    /// Serializes the value to bytes. The bytes shall include padding spaces.
    fn to_bytes(&self) -> [u8; 70];
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
        // TODO: account for escaping ' character
        String::from_utf8(raw).map_err(|er| FitsHeaderError::DeserializationError {
            found: er.into_bytes(),
            intent: String::from("header card String value"),
        })
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

/// An enumeration of valid values corresponding to the BITPIX keyword.
#[derive(Debug, Clone, Copy)]
pub enum Bitpix {
    /// Indicates each data element is an unsigned 8 bit integer value.
    U8,
    /// Indicates each data element is a signed 16 bit integer value.
    I16,
    /// Indicates each data element is a signed 32 bit integer value.
    I32,
    /// Indicates each data element is a signed 32 bit float value.
    F32,
    /// Indicates each data element is a signed 64 bit float value.
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

    fn to_bytes(&self) -> [u8; 70] {
        let mut result = [b' '; 70];
        match *self {
            Bitpix::U8 => result[19] = b'8',
            Bitpix::I16 => {
                let value_raw = b"16";
                let start = 20 - value_raw.len();
                for (i, b) in value_raw.iter().enumerate() {
                    result[start + i] = *b;
                }
            }
            Bitpix::I32 => {
                let value_raw = b"32";
                let start = 20 - value_raw.len();
                for (i, b) in value_raw.iter().enumerate() {
                    result[start + i] = *b;
                }
            }
            Bitpix::F32 => {
                let value_raw = b"-32";
                let start = 20 - value_raw.len();
                for (i, b) in value_raw.iter().enumerate() {
                    result[start + i] = *b;
                }
            }
            Bitpix::F64 => {
                let value_raw = b"-64";
                let start = 20 - value_raw.len();
                for (i, b) in value_raw.iter().enumerate() {
                    result[start + i] = *b;
                }
            }
        }
        result
    }
}
