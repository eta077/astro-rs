use std::fmt::Debug;
use std::rc::Rc;

use super::header::FitsHeaderError;

/// A trait that allows data to be serialized/deserialized as a FITS header value.
pub trait FitsHeaderValue: Debug {
    /// Attempts to deserialize a value from the given bytes. The given bytes shall not be padded by spaces.
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError>
    where
        Self: Sized;

    /// Serializes the value to bytes. The bytes shall include padding spaces.
    fn to_bytes(self: Rc<Self>) -> [u8; 70];
}

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

    fn to_bytes(self: Rc<Self>) -> [u8; 70] {
        let mut result = [b' '; 70];
        if *self {
            result[19] = b'T';
        } else {
            result[19] = b'F';
        }
        result
    }
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

    fn to_bytes(self: Rc<Self>) -> [u8; 70] {
        let mut result = [b' '; 70];
        result[19] = *self + 48;
        result
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

    fn to_bytes(self: Rc<Self>) -> [u8; 70] {
        let mut result = [b' '; 70];
        let value_raw = self.to_string().into_bytes();
        let start = 20 - value_raw.len();
        for (i, b) in value_raw.iter().enumerate() {
            result[start + i] = *b;
        }
        result
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

    fn to_bytes(self: Rc<Self>) -> [u8; 70] {
        let mut result = [b' '; 70];
        let value_raw = self.to_string().into_bytes();
        let start = 20 - value_raw.len();
        for (i, b) in value_raw.iter().enumerate() {
            result[start + i] = *b;
        }
        result
    }
}

impl FitsHeaderValue for String {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        String::from_utf8(raw).map_err(|er| FitsHeaderError::DeserializationError {
            found: er.into_bytes(),
            intent: String::from("header card u16 value"),
        })
    }

    fn to_bytes(self: Rc<Self>) -> [u8; 70] {
        let mut result = [b' '; 70];
        result[0] = b'\'';
        let value_raw = self.as_bytes();
        for (i, b) in value_raw.iter().enumerate() {
            result[i + 1] = *b;
        }
        result[value_raw.len()] = b'\'';
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

    fn to_bytes(self: Rc<Self>) -> [u8; 70] {
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
