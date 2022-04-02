//! Construct standard HDU types.

use super::{FitsHeader, FitsHeaderCard, Hdu};

const DEFAULT_BITPIX_BYTES: [u8; 80] =
    *b"BITPIX  =                    8                                                  ";
const DEFAULT_NAXIS_BYTES: [u8; 80] =
    *b"NAXIS   =                    0                                                  ";
const DEFAULT_PCOUNT_BYTES: [u8; 80] =
    *b"PCOUNT  =                    0                                                  ";
const DEFAULT_GCOUNT_BYTES: [u8; 80] =
    *b"GCOUNT  =                    1                                                  ";
const DEFAULT_TFIELDS_BYTES: [u8; 80] =
    *b"TFIELDS =                    0                                                  ";
const DEFAULT_END_BYTES: [u8; 80] =
    *b"END                                                                             ";

/// Constructs an HDU pre-populated with the required cards to be a Primary HDU.
pub fn primary_hdu() -> Hdu {
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

/// Constructs an HDU pre-populated with the required cards to be an ASCII Table HDU.
pub fn ascii_table_hdu() -> Hdu {
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

/// Constructs an HDU pre-populated with the required cards to be an Image HDU.
pub fn image_hdu() -> Hdu {
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

/// Constructs an HDU pre-populated with the required cards to be a Binary Table HDU.
pub fn binary_table_hdu() -> Hdu {
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
