//! Defines the BITPIX header value.

use crate::fits::FitsHeaderError;

use super::FitsHeaderValue;

/// An enumeration of valid values corresponding to the BITPIX keyword.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
