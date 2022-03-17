use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};

use thiserror::Error;

const HDU_BLOCK_SIZE: usize = 2880;
const CARD_LENGTH: usize = 80;
pub const SIMPLE_KEYWORD: &str = "SIMPLE  ";
pub const NAXIS_KEYWORD: &str = "NAXIS   ";
pub const BITPIX_KEYWORD: &str = "BITPIX  ";
pub const END_KEYWORD: &str = "END     ";

#[derive(Error, Debug)]
pub enum FitsError {
    #[error("file IO error")]
    FileIoError(#[from] std::io::Error),
    #[error("unexpected end of file - expected {expected} bytes for {intent}, found {found}")]
    UnexpectedEof {
        expected: usize,
        found: usize,
        intent: String,
    },
    #[error("expected valid string for {intent}, found {found:?}")]
    DeserializationError { found: Vec<u8>, intent: String },
    #[error("invalid signature - expected 'SIMPLE  =                    (T|F)', found {found:?}")]
    InvalidSignature { found: FitsHeaderCard },
}

#[derive(Debug, Clone, Copy)]
pub enum Bitpix {
    U8,
    I16,
    I32,
    F32,
    F64,
}

impl Bitpix {
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

impl TryFrom<String> for Bitpix {
    type Error = FitsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "8" => Ok(Bitpix::U8),
            "16" => Ok(Bitpix::I16),
            "32" => Ok(Bitpix::I32),
            "-32" => Ok(Bitpix::F32),
            "-64" => Ok(Bitpix::F64),
            _ => Err(FitsError::DeserializationError {
                found: value.into_bytes(),
                intent: String::from("BITPIX value"),
            }),
        }
    }
}

impl Display for Bitpix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Bitpix::U8 => "8",
            Bitpix::I16 => "16",
            Bitpix::I32 => "32",
            Bitpix::F32 => "-32",
            Bitpix::F64 => "-64",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub enum FitsData {
    Raw(Vec<u8>),
    Bool(bool),
    String(String),
    U16(u16),
    Bitpix(Bitpix),
}

impl FitsData {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            FitsData::Raw(value) => value.to_vec(),
            FitsData::Bool(value) => {
                if *value {
                    vec![b'T']
                } else {
                    vec![b'F']
                }
            }
            FitsData::String(value) => value.to_owned().into_bytes(),
            FitsData::U16(value) => value.to_string().into_bytes(),
            FitsData::Bitpix(value) => value.to_string().into_bytes(),
        }
    }
}

#[derive(Debug)]
pub struct FitsHeaderCard {
    pub keyword: String,
    data: FitsData,
    pub comment: Vec<u8>,
}

impl FitsHeaderCard {
    pub fn get_bool_data(&mut self) -> Result<bool, FitsError> {
        match &self.data {
            FitsData::Raw(bytes) => {
                if let Some(value) = bytes.first() {
                    match *value {
                        b'T' => {
                            self.data = FitsData::Bool(true);
                            Ok(true)
                        }
                        b'F' => {
                            self.data = FitsData::Bool(false);
                            Ok(false)
                        }
                        _ => Err(FitsError::DeserializationError {
                            found: bytes.to_owned(),
                            intent: String::from("header card bool data"),
                        }),
                    }
                } else {
                    Err(FitsError::DeserializationError {
                        found: bytes.to_owned(),
                        intent: String::from("header card bool data"),
                    })
                }
            }
            FitsData::Bool(value) => Ok(*value),
            _ => Err(FitsError::DeserializationError {
                found: self.data.to_bytes(),
                intent: String::from("header card bool data"),
            }),
        }
    }

    pub fn get_u16_data(&mut self) -> Result<u16, FitsError> {
        match &self.data {
            FitsData::Raw(bytes) => {
                let value_string = String::from_utf8(bytes.to_owned()).map_err(|er| {
                    FitsError::DeserializationError {
                        found: er.into_bytes(),
                        intent: String::from("header card u16 data"),
                    }
                })?;
                let value = u16::from_str_radix(value_string.as_str(), 10).map_err(|_| {
                    FitsError::DeserializationError {
                        found: value_string.into_bytes(),
                        intent: String::from("header card u16 data"),
                    }
                })?;
                self.data = FitsData::U16(value);
                Ok(value)
            }
            FitsData::U16(value) => Ok(*value),
            _ => Err(FitsError::DeserializationError {
                found: self.data.to_bytes(),
                intent: String::from("header card u16 data"),
            }),
        }
    }

    pub fn get_bitpix_data(&mut self) -> Result<Bitpix, FitsError> {
        match &self.data {
            FitsData::Raw(bytes) => {
                let value_string = String::from_utf8(bytes.to_owned()).map_err(|er| {
                    FitsError::DeserializationError {
                        found: er.into_bytes(),
                        intent: String::from("header card u16 data"),
                    }
                })?;
                let value = Bitpix::try_from(value_string)?;
                self.data = FitsData::Bitpix(value);
                Ok(value)
            }
            FitsData::Bitpix(value) => Ok(*value),
            _ => Err(FitsError::DeserializationError {
                found: self.data.to_bytes(),
                intent: String::from("header card bitpix data"),
            }),
        }
    }
}

#[derive(Debug, Default)]
pub struct FitsHeader {
    pub cards: HashMap<String, FitsHeaderCard>,
}

impl FitsHeader {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct Hdu {
    pub header: FitsHeader,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct HduList {
    pub hdus: Vec<Hdu>,
}

pub fn read_fits_file(file: File, mut check_signature: bool) -> Result<HduList, FitsError> {
    let mut file_reader = BufReader::new(file);

    let mut hdus = Vec::new();
    loop {
        let mut hdu_bytes = read_hdu_bytes(&mut file_reader, hdus.is_empty())?;
        if hdu_bytes.is_empty() {
            break;
        }
        let mut header = FitsHeader::new();

        let mut signature_card = read_next_header_card(&mut hdu_bytes)?;
        if check_signature {
            let signature_matches = if signature_card.keyword == SIMPLE_KEYWORD {
                signature_card.get_bool_data().is_ok()
            } else {
                false
            };
            if !signature_matches {
                return Err(FitsError::InvalidSignature {
                    found: signature_card,
                });
            }
            check_signature = false;
        }
        header
            .cards
            .insert(signature_card.keyword.clone(), signature_card);

        loop {
            let card = read_next_header_card(&mut hdu_bytes)?;
            if card.keyword == END_KEYWORD {
                header.cards.insert(card.keyword.clone(), card);
                break;
            }
            header.cards.insert(card.keyword.clone(), card);

            if hdu_bytes.is_empty() {
                hdu_bytes = read_hdu_bytes(&mut file_reader, hdus.is_empty())?;
            }
        }
        let mut data = Vec::new();
        let naxis = header
            .cards
            .get_mut(NAXIS_KEYWORD)
            .and_then(|card| card.get_u16_data().ok())
            .unwrap_or_default() as u8;
        if naxis > 0 {
            let bitpix = header
                .cards
                .get_mut(BITPIX_KEYWORD)
                .and_then(|card| card.get_bitpix_data().ok())
                .map(|bitpix| bitpix.value())
                .unwrap_or_default();
            if bitpix > 0 {
                let mut data_len = 1;
                for x in 1..=naxis {
                    let mut naxisx_keyword = NAXIS_KEYWORD.to_owned();
                    unsafe {
                        let naxisx_keyword_bytes = naxisx_keyword.as_bytes_mut();
                        naxisx_keyword_bytes[5] = x + 48;
                    }
                    let naxisx = header
                        .cards
                        .get_mut(&naxisx_keyword)
                        .and_then(|card| card.get_u16_data().ok())
                        .unwrap_or_default() as usize;
                    data_len *= naxisx;
                }
                data_len *= bitpix / 8;
                data = vec![0; data_len];
                // ignore error if not able to read exact
                let _ = file_reader.read_exact(&mut data);
            }
        }

        let hdu = Hdu { header, data };

        hdus.push(hdu);
    }

    Ok(HduList { hdus })
}

fn read_hdu_bytes(
    file_reader: &mut BufReader<File>,
    first_hdu: bool,
) -> Result<Vec<u8>, FitsError> {
    let mut hdu_bytes = [0; HDU_BLOCK_SIZE];
    if let Err(er) = file_reader.read_exact(&mut hdu_bytes) {
        if first_hdu {
            if er.kind() == ErrorKind::UnexpectedEof {
                return Err(FitsError::UnexpectedEof {
                    expected: HDU_BLOCK_SIZE,
                    found: hdu_bytes.len(),
                    intent: String::from("HDU 0"),
                });
            } else {
                return Err(FitsError::FileIoError(er));
            }
        } else {
            return Ok(Vec::new());
        }
    };
    Ok(hdu_bytes.to_vec())
}

fn read_next_header_card(hdu_bytes: &mut Vec<u8>) -> Result<FitsHeaderCard, FitsError> {
    if hdu_bytes.len() < CARD_LENGTH {
        return Err(FitsError::UnexpectedEof {
            expected: CARD_LENGTH,
            found: hdu_bytes.len(),
            intent: String::from("header card"),
        });
    }
    let mut card_bytes = hdu_bytes.drain(0..CARD_LENGTH).collect::<Vec<u8>>();

    let keyword_bytes = card_bytes.drain(0..8).collect();
    let keyword =
        String::from_utf8(keyword_bytes).map_err(|er| FitsError::DeserializationError {
            found: er.into_bytes(),
            intent: String::from("header card keyword"),
        })?;

    let mut data = Vec::new();
    let mut comment = Vec::new();
    if keyword != END_KEYWORD {
        let mut value = Vec::with_capacity(72);
        for (index, b) in card_bytes.iter().enumerate().rev() {
            match *b {
                b'/' => {
                    comment = trim_value(&mut value);
                    value = Vec::with_capacity(index);
                }
                b'=' => {
                    data = trim_value(&mut value);
                    value = Vec::new();
                }
                _ => value.insert(0, *b),
            }
        }
        if data.is_empty() {
            data = value.clone();
        }
    }

    Ok(FitsHeaderCard {
        keyword,
        data: FitsData::Raw(data),
        comment,
    })
}

fn trim_value(value: &mut Vec<u8>) -> Vec<u8> {
    let mut index1 = 0;
    let mut index2 = value.len();
    for b in value.iter() {
        match *b {
            b' ' | 0 => index1 += 1,
            _ => break,
        }
    }
    if index1 == value.len() {
        return Vec::new();
    }
    for b in value.iter().rev() {
        match *b {
            b' ' | 0 => index2 -= 1,
            _ => break,
        }
    }
    value[index1..index2].to_vec()
}
