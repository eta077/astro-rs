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

/// An enumeration of valid types corresponding to the TFORM keyword.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum TFormType {
    Logical,
    Bit,
    UnsignedByte,
    I16,
    I32,
    Character,
    F32,
    F64,
    C64,
    C128,
    ArrayDescriptor,
}

impl TryFrom<char> for TFormType {
    type Error = FitsHeaderError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(TFormType::Logical),
            'X' => Ok(TFormType::Bit),
            'B' => Ok(TFormType::UnsignedByte),
            'I' => Ok(TFormType::I16),
            'J' => Ok(TFormType::I32),
            'A' => Ok(TFormType::Character),
            'E' => Ok(TFormType::F32),
            'D' => Ok(TFormType::F64),
            'C' => Ok(TFormType::C64),
            'M' => Ok(TFormType::C128),
            'P' => Ok(TFormType::ArrayDescriptor),
            _ => Err(FitsHeaderError::DeserializationError {
                found: vec![value as u8],
                intent: String::from("header card TFORM type value"),
            }),
        }
    }
}

/// A value corresponding to the TFORM keyword.
#[derive(Debug, Clone)]
pub struct TForm {
    /// The repeat count
    pub r: usize,
    /// The field type
    pub t: TFormType,
    /// Undefined additional characters
    pub a: String,
}

impl TForm {
    /// Gets the number of bytes required by the column.
    pub fn value(&self) -> usize {
        let type_bytes = match self.t {
            TFormType::Logical => 1,
            TFormType::Bit => todo!(),
            TFormType::UnsignedByte => 1,
            TFormType::I16 => 2,
            TFormType::I32 => 4,
            TFormType::Character => 1,
            TFormType::F32 => 4,
            TFormType::F64 => 8,
            TFormType::C64 => 8,
            TFormType::C128 => 16,
            TFormType::ArrayDescriptor => 8,
        };
        self.r * type_bytes
    }

    /// Gets the function to transform raw bytes to the given type.
    pub fn get_values<T>(
        &self,
        data: &[u8],
        column_start: usize,
        row_len: usize,
        num_rows: usize,
    ) -> Box<Vec<T>> {
        let column_len = self.value();
        unsafe {
            match self.t {
                TFormType::Logical => {
                    let mut result = Vec::with_capacity(num_rows * self.r);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let column = data[start..start + column_len].to_vec();
                        for value in column.iter().take(self.r) {
                            result.push(*value != 0);
                        }
                    }

                    let b = Box::new(result);
                    let ptr = Box::into_raw(b);
                    let new_ptr = ptr.cast();
                    Box::from_raw(new_ptr)
                }
                TFormType::Bit => todo!(),
                TFormType::UnsignedByte => {
                    let mut result = Vec::with_capacity(num_rows);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let mut column = data[start..start + column_len].to_vec();
                        result.append(&mut column);
                    }

                    let b = Box::new(result);
                    let ptr = Box::into_raw(b);
                    let new_ptr = ptr.cast();
                    Box::from_raw(new_ptr)
                }
                TFormType::I16 => {
                    let mut result = Vec::with_capacity(num_rows);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let column = data[start..start + column_len].to_vec();
                        let value_size = std::mem::size_of::<i16>();
                        for repeat in 0..self.r {
                            let value_start = repeat * value_size;
                            let value = i16::from_be_bytes(
                                column[value_start..value_start + value_size]
                                    .try_into()
                                    .unwrap(),
                            );
                            result.push(value);
                        }
                    }

                    let b = Box::new(result);
                    let ptr = Box::into_raw(b);
                    let new_ptr = ptr.cast();
                    Box::from_raw(new_ptr)
                }
                TFormType::I32 => {
                    let mut result = Vec::with_capacity(num_rows);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let column = data[start..start + column_len].to_vec();
                        let value_size = std::mem::size_of::<i32>();
                        for repeat in 0..self.r {
                            let value_start = repeat * value_size;
                            let value = i32::from_be_bytes(
                                column[value_start..value_start + value_size]
                                    .try_into()
                                    .unwrap(),
                            );
                            result.push(value);
                        }
                    }

                    let b = Box::new(result);
                    let ptr = Box::into_raw(b);
                    let new_ptr = ptr.cast();
                    Box::from_raw(new_ptr)
                }
                TFormType::Character => {
                    let mut result = Vec::with_capacity(num_rows);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let column = data[start..start + column_len].to_vec();
                        let value_size = std::mem::size_of::<char>();
                        for repeat in 0..self.r {
                            let value_start = repeat * value_size;
                            let value = char::from_u32_unchecked(u32::from_be_bytes(
                                column[value_start..value_start + value_size]
                                    .try_into()
                                    .unwrap(),
                            ));
                            result.push(value);
                        }
                    }

                    let b = Box::new(result);
                    let ptr = Box::into_raw(b);
                    let new_ptr = ptr.cast();
                    Box::from_raw(new_ptr)
                }
                TFormType::F32 => {
                    let mut result = Vec::with_capacity(num_rows);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let column = data[start..start + column_len].to_vec();
                        let value_size = std::mem::size_of::<f32>();
                        for repeat in 0..self.r {
                            let value_start = repeat * value_size;
                            let value = f32::from_be_bytes(
                                column[value_start..value_start + value_size]
                                    .try_into()
                                    .unwrap(),
                            );
                            result.push(value);
                        }
                    }

                    let b = Box::new(result);
                    let ptr = Box::into_raw(b);
                    let new_ptr = ptr.cast();
                    Box::from_raw(new_ptr)
                }
                TFormType::F64 => {
                    let mut result = Vec::with_capacity(num_rows);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let chunk = data[start..start + column_len].to_vec();
                        result.push(f64::from_be_bytes(chunk.try_into().unwrap()));
                    }

                    let b = Box::new(result);
                    let ptr = Box::into_raw(b);
                    let new_ptr = ptr.cast();
                    Box::from_raw(new_ptr)
                }
                TFormType::C64 => todo!(),
                TFormType::C128 => todo!(),
                TFormType::ArrayDescriptor => todo!(),
            }
        }
    }
}

impl FitsHeaderValue for TForm {
    fn from_bytes(raw: Vec<u8>) -> Result<Self, FitsHeaderError> {
        let mut repeats = String::new();
        let mut ttype = None;
        let mut i = 1;
        while i < raw.len() - 1 {
            let ch = raw[i] as char;
            if ch.is_ascii_digit() {
                repeats.push(ch);
                i += 1;
            } else {
                ttype = Some(TFormType::try_from(ch)?);
                i += 1;
                break;
            }
        }
        let r = repeats.parse::<u32>().unwrap_or(1) as usize;
        if let Some(t) = ttype {
            if let Ok(a) = String::from_utf8(raw[i..raw.len() - 1].to_vec()) {
                return Ok(TForm {
                    r,
                    t,
                    a: a.trim().to_owned(),
                });
            }
        }
        Err(FitsHeaderError::DeserializationError {
            found: raw,
            intent: String::from("header card TFORM value"),
        })
    }

    fn to_bytes(&self) -> [u8; 70] {
        let mut result = [b' '; 70];
        let mut i = 0;
        result[i] = b'\'';
        i += 1;
        let repeats = self.r.to_string();
        for b in repeats.bytes() {
            result[i] = b;
            i += 1;
        }
        match self.t {
            TFormType::Logical => result[i] = b'L',
            TFormType::Bit => result[i] = b'X',
            TFormType::UnsignedByte => result[i] = b'B',
            TFormType::I16 => result[i] = b'I',
            TFormType::I32 => result[i] = b'J',
            TFormType::Character => result[i] = b'A',
            TFormType::F32 => result[i] = b'E',
            TFormType::F64 => result[i] = b'D',
            TFormType::C64 => result[i] = b'C',
            TFormType::C128 => result[i] = b'M',
            TFormType::ArrayDescriptor => result[i] = b'P',
        }
        i += 1;
        for b in self.a.bytes() {
            result[i] = b;
            i += 1;
        }
        result
    }
}
