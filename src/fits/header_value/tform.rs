//! Defines the TFORM header value.

use crate::fits::hdu_macros::return_box;
use crate::fits::FitsHeaderError;

use super::FitsHeaderValue;

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

    /// Creates a column of values from the given data.
    pub fn create_column<T>(
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

                    return_box!(result)
                }
                TFormType::Bit => todo!(),
                TFormType::UnsignedByte => {
                    let mut result = Vec::with_capacity(num_rows * self.r);
                    for i in 0..num_rows {
                        let start = row_len * i + column_start;
                        let mut column = data[start..start + column_len].to_vec();
                        result.append(&mut column);
                    }

                    return_box!(result)
                }
                TFormType::I16 => {
                    tform_macros::deserialize_column!(
                        i16,
                        num_rows,
                        row_len,
                        column_start,
                        column_len,
                        self.r,
                        data,
                    )
                }
                TFormType::I32 => {
                    tform_macros::deserialize_column!(
                        i32,
                        num_rows,
                        row_len,
                        column_start,
                        column_len,
                        self.r,
                        data,
                    )
                }
                TFormType::Character => {
                    unsafe fn deserialize_char(value: [u8; 4]) -> char {
                        char::from_u32_unchecked(u32::from_be_bytes(value))
                    }
                    tform_macros::deserialize_column!(
                        char,
                        num_rows,
                        row_len,
                        column_start,
                        column_len,
                        self.r,
                        data,
                        deserialize_char,
                    )
                }
                TFormType::F32 => {
                    tform_macros::deserialize_column!(
                        f32,
                        num_rows,
                        row_len,
                        column_start,
                        column_len,
                        self.r,
                        data,
                    )
                }
                TFormType::F64 => {
                    tform_macros::deserialize_column!(
                        f64,
                        num_rows,
                        row_len,
                        column_start,
                        column_len,
                        self.r,
                        data,
                    )
                }
                TFormType::C64 => todo!(),
                TFormType::C128 => todo!(),
                TFormType::ArrayDescriptor => todo!(),
            }
        }
    }
}

#[macro_use]
mod tform_macros {
    /// Creates a boxed vector deserialized with the given function, or a default function if one is not given.
    macro_rules! deserialize_column {
        (@dfn $value_type: ty) => {{
            <$value_type>::from_be_bytes
        }};
        ($value_type: ty, $num_rows: expr, $row_len: expr, $column_start: expr, $column_len: expr, $repeats: expr, $data: expr,) => {{
            let deserialize_fn = $crate::fits::header_value::tform::tform_macros::deserialize_column!(@dfn $value_type);
            $crate::fits::header_value::tform::tform_macros::deserialize_column!(
                $value_type,
                $num_rows,
                $row_len,
                $column_start,
                $column_len,
                $repeats,
                $data,
                deserialize_fn,
            )
        }};
        ($value_type: ty, $num_rows: expr, $row_len: expr, $column_start: expr, $column_len: expr, $repeats: expr, $data: expr, $deserialize_fn: tt,) => {{
            let mut result = Vec::with_capacity($num_rows * $repeats);
            for i in 0..$num_rows {
                let start = $row_len * i + $column_start;
                let column = $data[start..start + $column_len].to_vec();
                let value_size = std::mem::size_of::<$value_type>();
                for repeat in 0..$repeats {
                    let value_start = repeat * value_size;
                    let raw_value = column[value_start..value_start + value_size]
                        .try_into()
                        .unwrap();
                    result.push($deserialize_fn(raw_value));
                }
            }

            $crate::fits::hdu_macros::return_box!(result)
        }};
    }

    pub(crate) use deserialize_column;
}

/// ```
/// use astro_rs::fits::*;
///
/// // successful deserialization
/// let double_value: TForm = FitsHeaderValue::from_bytes(b"'1D      '".to_vec())?;
/// assert_eq!(double_value.value(), 8);
/// let repeat_int_value: TForm = FitsHeaderValue::from_bytes(b"'2I      '".to_vec())?;
/// assert_eq!(repeat_int_value.value(), 4);
/// let comment_char_value: TForm = FitsHeaderValue::from_bytes(b"'1A comment'".to_vec())?;
/// assert_eq!(comment_char_value.value(), 1);
/// let short_complex_value: TForm = FitsHeaderValue::from_bytes(b"'M       '".to_vec())?;
/// assert_eq!(short_complex_value.value(), 16);
///
/// // failed deserialization
/// let result: Result<TForm, FitsHeaderError> = FitsHeaderValue::from_bytes(b"U".to_vec());
/// assert!(result.is_err());
/// let result: Result<TForm, FitsHeaderError> = FitsHeaderValue::from_bytes(b"nonsense".to_vec());
/// assert!(result.is_err());
///
/// // serialization
/// assert_eq!(double_value.to_bytes(), *b"'1D      '                                                            ");
/// assert_eq!(repeat_int_value.to_bytes(), *b"'2I      '                                                            ");
/// assert_eq!(comment_char_value.to_bytes(), *b"'1A comment'                                                          ");
/// assert_eq!(short_complex_value.to_bytes(), *b"'1M      '                                                            ");
/// # Ok::<(), astro_rs::fits::FitsHeaderError>(())
/// ```
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
                    a: a.trim_end().to_owned(),
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
        if i < 9 {
            i = 9;
        }
        result[i] = b'\'';
        result
    }
}
