//! Construct standard HDU types.

use super::*;

/// The header keyword indicating the name of a table column.
pub const TTYPE_KEYWORD: [u8; 8] = *b"TTYPE   ";
/// The header keyword indicating the size in bytes of a table column.
pub const TFORM_KEYWORD: [u8; 8] = *b"TFORM   ";

pub(crate) const DEFAULT_BITPIX_BYTES: [u8; 80] =
    *b"BITPIX  =                    8                                                  ";
pub(crate) const DEFAULT_NAXIS_BYTES: [u8; 80] =
    *b"NAXIS   =                    0                                                  ";
pub(crate) const DEFAULT_PCOUNT_BYTES: [u8; 80] =
    *b"PCOUNT  =                    0                                                  ";
pub(crate) const DEFAULT_GCOUNT_BYTES: [u8; 80] =
    *b"GCOUNT  =                    1                                                  ";
pub(crate) const DEFAULT_TFIELDS_BYTES: [u8; 80] =
    *b"TFIELDS =                    0                                                  ";
pub(crate) const DEFAULT_END_BYTES: [u8; 80] =
    *b"END                                                                             ";

/// Functions related to a Primary type HDU.
pub mod primary_hdu {
    use super::*;

    /// Constructs an HDU pre-populated with the required cards to be a Primary HDU.
    pub fn default() -> Hdu {
        let simple_card = FitsHeaderCard::from(
            *b"SIMPLE  =                    T                                                  ",
        );
        let bitpix_card = FitsHeaderCard::from(DEFAULT_BITPIX_BYTES);
        let naxis_card = FitsHeaderCard::from(DEFAULT_NAXIS_BYTES);
        let end_card = FitsHeaderCard::from(DEFAULT_END_BYTES);
        let header = FitsHeader {
            cards: vec![simple_card, bitpix_card, naxis_card, end_card],
        };

        Hdu {
            header,
            ..Default::default()
        }
    }
}

/// Functions related to an ASCII Table type HDU.
pub mod ascii_table_hdu {
    use super::*;

    /// Constructs an HDU pre-populated with the required cards to be an ASCII Table HDU.
    pub fn default() -> Hdu {
        let xtension_card = FitsHeaderCard::from(
            *b"XTENSION= 'TABLE   '                                                            ",
        );
        let bitpix_card = FitsHeaderCard::from(DEFAULT_BITPIX_BYTES);
        let naxis_card = FitsHeaderCard::from(
            *b"NAXIS   =                    2                                                  ",
        );
        let naxis1_card = FitsHeaderCard::from(
            *b"NAXIS1  =                    0                                                  ",
        );
        let naxis2_card = FitsHeaderCard::from(
            *b"NAXIS2  =                    0                                                  ",
        );
        let pcount_card = FitsHeaderCard::from(DEFAULT_PCOUNT_BYTES);
        let gcount_card = FitsHeaderCard::from(DEFAULT_GCOUNT_BYTES);
        let tfields_card = FitsHeaderCard::from(DEFAULT_TFIELDS_BYTES);
        let end_card = FitsHeaderCard::from(DEFAULT_END_BYTES);
        let header = FitsHeader {
            cards: vec![
                xtension_card,
                bitpix_card,
                naxis_card,
                naxis1_card,
                naxis2_card,
                pcount_card,
                gcount_card,
                tfields_card,
                end_card,
            ],
        };

        Hdu {
            header,
            ..Default::default()
        }
    }
}

/// Functions related to an Image type HDU.
pub mod image_hdu {
    use super::*;

    /// Constructs an HDU pre-populated with the required cards to be an Image HDU.
    pub fn default() -> Hdu {
        let xtension_card = FitsHeaderCard::from(
            *b"XTENSION= 'IMAGE   '                                                            ",
        );
        let bitpix_card = FitsHeaderCard::from(DEFAULT_BITPIX_BYTES);
        let naxis_card = FitsHeaderCard::from(DEFAULT_NAXIS_BYTES);
        let pcount_card = FitsHeaderCard::from(DEFAULT_PCOUNT_BYTES);
        let gcount_card = FitsHeaderCard::from(DEFAULT_GCOUNT_BYTES);
        let end_card = FitsHeaderCard::from(DEFAULT_END_BYTES);
        let header = FitsHeader {
            cards: vec![
                xtension_card,
                bitpix_card,
                naxis_card,
                pcount_card,
                gcount_card,
                end_card,
            ],
        };

        Hdu {
            header,
            ..Default::default()
        }
    }
}

/// Functions related to a Binary Table type HDU.
pub mod binary_table_hdu {
    use super::*;
    use crate::fits::header_value::TForm;

    /// Constructs an HDU pre-populated with the required cards to be a Binary Table HDU.
    pub fn default() -> Hdu {
        let xtension_card = FitsHeaderCard::from(
            *b"XTENSION= 'BINTABLE'                                                            ",
        );
        let bitpix_card = FitsHeaderCard::from(DEFAULT_BITPIX_BYTES);
        let naxis_card = FitsHeaderCard::from(
            *b"NAXIS   =                    2                                                  ",
        );
        let naxis1_card = FitsHeaderCard::from(
            *b"NAXIS1  =                    0                                                  ",
        );
        let naxis2_card = FitsHeaderCard::from(
            *b"NAXIS2  =                    0                                                  ",
        );
        let pcount_card = FitsHeaderCard::from(DEFAULT_PCOUNT_BYTES);
        let gcount_card = FitsHeaderCard::from(DEFAULT_GCOUNT_BYTES);
        let tfields_card = FitsHeaderCard::from(DEFAULT_TFIELDS_BYTES);
        let end_card = FitsHeaderCard::from(DEFAULT_END_BYTES);
        let header = FitsHeader {
            cards: vec![
                xtension_card,
                bitpix_card,
                naxis_card,
                naxis1_card,
                naxis2_card,
                pcount_card,
                gcount_card,
                tfields_card,
                end_card,
            ],
        };

        Hdu {
            header,
            ..Default::default()
        }
    }

    /// Obtains the data in the column of the given name, or None if a column with the given name cannot be found.
    pub fn column_by_name<T>(hdu: &mut Hdu, name: &str) -> Option<Vec<T>> {
        let mut n = 1;
        let mut column_start = 0;
        let mut tform = None;
        let mut naxis_keyword = NAXIS_KEYWORD;
        naxis_keyword[5] = b'2';
        let num_rows = *hdu
            .header
            .get_card(naxis_keyword)
            .and_then(|card| card.get_value::<u32>().ok())
            .unwrap_or_default() as usize;
        naxis_keyword[5] = b'1';
        let row_len = *hdu
            .header
            .get_card(naxis_keyword)
            .and_then(|card| card.get_value::<u32>().ok())
            .unwrap_or_default() as usize;
        let mut tform_keyword = FitsHeaderKeyword::from(TFORM_KEYWORD);
        let mut ttype_keyword = FitsHeaderKeyword::from(TTYPE_KEYWORD);
        while n as usize <= num_rows {
            tform_keyword.append_number(n);
            if let Some(card) = hdu.header.get_card(tform_keyword) {
                if let Ok(tform_value) = card.get_value::<TForm>() {
                    ttype_keyword.append_number(n);
                    if let Some(value) = hdu
                        .header
                        .get_card(ttype_keyword)
                        .and_then(|card| card.get_value::<String>().ok())
                    {
                        if value.eq_ignore_ascii_case(name) {
                            tform = Some(tform_value);
                            break;
                        }
                    }

                    column_start += tform_value.value();
                }
                n += 1;
            } else {
                break;
            }
        }
        if let Some(tform) = tform {
            return Some(*tform.create_column(hdu.data_raw(), column_start, row_len, num_rows));
        }
        None
    }

    /// Obtains the data in the column of the given index, or None if a column with the given index cannot be found.
    /// Note that column indeces start at 1.
    pub fn column_by_index<T>(hdu: &mut Hdu, index: u16) -> Option<Vec<T>> {
        let mut n = 1;
        let mut column_start = 0;
        let mut tform = None;
        let mut naxis_keyword = NAXIS_KEYWORD;
        naxis_keyword[5] = b'2';
        let num_rows = *hdu
            .header
            .get_card(naxis_keyword)
            .and_then(|card| card.get_value::<u32>().ok())
            .unwrap_or_default() as usize;
        if index as usize > num_rows {
            return None;
        }
        naxis_keyword[5] = b'1';
        let row_len = *hdu
            .header
            .get_card(naxis_keyword)
            .and_then(|card| card.get_value::<u32>().ok())
            .unwrap_or_default() as usize;
        let mut tform_keyword = FitsHeaderKeyword::from(TFORM_KEYWORD);
        while n <= index {
            tform_keyword.append_number(n);
            if let Some(card) = hdu.header.get_card(tform_keyword) {
                if let Ok(value) = card.get_value::<TForm>() {
                    if n == index {
                        tform = Some(value);
                    } else {
                        column_start += value.value();
                    }
                }
                n += 1;
            } else {
                break;
            }
        }
        if let Some(tform) = tform {
            return Some(*tform.create_column(hdu.data_raw(), column_start, row_len, num_rows));
        }
        None
    }
}
