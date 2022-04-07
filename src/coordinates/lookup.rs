use super::EquatorialCoord;

use hyper::Client;
use measurements::Angle;
use thiserror::Error;
use urlencoding::encode;

const SIMBAD_BASE_URL: &str =
    "http://simbad.u-strasbg.fr/simbad/sim-id?output.format=votable&Ident=";
const SIMBAD_OUTPUT_PARAMS: &str = "&output.params=ra(d;ICRS;J2000;2000),dec(d;ICRS;J2000;2000)";

/// An enumeration of errors that can occur while performing a coordinate lookup.
#[derive(Debug, Error)]
pub enum AstroLookupError {
    /// Indicates an error occurred while obtaining the coordinate data.
    #[error(transparent)]
    NetworkError(#[from] hyper::Error),
    /// Indicates an error occurred while parsing the coordinate data.
    #[error("{reason}")]
    ParseError {
        /// The reason coordinate data parsing failed.
        reason: String,
    },
    /// Indicates coordinate data for the given name could not be found.
    #[error("Could not find coordinate data for {name}")]
    InvalidName {
        /// The name for which data could not be found.
        name: String,
    },
}

/// Fetches the coordinates of an object with the given identifier.
///
/// ```
/// use astro_rs::coordinates::{self, AstroLookupError, EquatorialCoord};
/// use measurements::Angle;
///
/// let m33_coords = tokio_test::block_on(async {
///     coordinates::lookup_by_name("M33").await
/// })?;
/// assert_eq!(m33_coords, EquatorialCoord {
///     ra: Angle::from_degrees(23.46206906218),
///     dec: Angle::from_degrees(30.66017511198)
/// });
///
/// let no_coords = tokio_test::block_on(async {
///     coordinates::lookup_by_name("something that should not resolve").await
/// });
/// assert!(no_coords.is_err());
/// # Ok::<(), astro_rs::coordinates::AstroLookupError>(())
/// ```
pub async fn lookup_by_name(name: &str) -> Result<EquatorialCoord, AstroLookupError> {
    let client = Client::new();
    let uri_string = [SIMBAD_BASE_URL, &encode(name), SIMBAD_OUTPUT_PARAMS].concat();
    let uri = uri_string
        .parse()
        .map_err(|_| AstroLookupError::InvalidName {
            name: name.to_owned(),
        })?;

    let response = client.get(uri).await?;
    let text_buf = hyper::body::to_bytes(response).await?;
    let xml_string = String::from_utf8(text_buf.as_ref().to_vec()).map_err(|er| {
        AstroLookupError::ParseError {
            reason: er.to_string(),
        }
    })?;

    if !xml_string.contains("<TD>") {
        return Err(AstroLookupError::InvalidName {
            name: name.to_owned(),
        });
    }
    let mut xml_parts = xml_string.split("<TD>");
    let ra = if let Some(ra_string) = xml_parts.nth(1) {
        let ra_trimmed = ra_string.trim_end_matches(|c: char| !c.is_numeric());
        ra_trimmed
            .parse()
            .map_err(|_| AstroLookupError::ParseError {
                reason: ["Could not parse ra value: ", ra_trimmed].concat(),
            })?
    } else {
        return Err(AstroLookupError::ParseError {
            reason: String::from("Could not find ra value"),
        });
    };
    let dec = if let Some(dec_string) = xml_parts.next() {
        let dec_trimmed = dec_string.trim_end_matches(|c: char| !c.is_numeric());
        dec_trimmed
            .parse()
            .map_err(|_| AstroLookupError::ParseError {
                reason: ["Could not parse dec value: ", dec_trimmed].concat(),
            })?
    } else {
        return Err(AstroLookupError::ParseError {
            reason: String::from("Could not find dec value"),
        });
    };

    Ok(EquatorialCoord {
        ra: Angle::from_degrees(ra),
        dec: Angle::from_degrees(dec),
    })
}
