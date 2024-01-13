use super::frames::Icrs;
use super::lookup_config::SesameConfig;
use super::EquatorialCoord;

use once_cell::sync::OnceCell;
use regex::Regex;
use reqwest::Client;
use thiserror::Error;
use uom::si::angle::{degree, Angle};
use urlencoding::encode;

static SESAME_CONFIG: OnceCell<SesameConfig> = OnceCell::new();
static SESAME_PARSER: OnceCell<Regex> = OnceCell::new();

fn init_sesame_parser() -> Regex {
    Regex::new(r"%J\s*([0-9\.]+)\s*([\+\-\.0-9]+)").unwrap()
}

/// An enumeration of errors that can occur while performing a coordinate lookup.
#[derive(Debug, Error)]
pub enum AstroLookupError {
    /// Indicates the environmental variables contributing to the SESAME configuration are invalid.
    #[error("Invalid configuration: {reason}")]
    InvalidConfiguration {
        /// The reason the configuration is invalid.
        reason: String,
    },
    /// Indicates an error occurred while obtaining the coordinate data.
    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),
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
/// # Examples
///
/// ```
/// use astro_rs::coordinates::{self, *};
/// use uom::si::angle::radian;
/// use uom::si::f64::Angle;
///
/// let m33_coords = tokio_test::block_on(async { coordinates::lookup_by_name("M33").await })?;
/// assert_eq!(m33_coords.round(4), Icrs {
///     coords: EquatorialCoord {
///         ra: Angle::new::<radian>(0.4095),
///         dec: Angle::new::<radian>(0.5351)
///     },
/// });
///
/// let no_coords = tokio_test::block_on(async {
///     coordinates::lookup_by_name("something that should not resolve").await
/// });
/// assert!(no_coords.is_err());
/// # Ok::<(), astro_rs::coordinates::AstroLookupError>(())
/// ```
pub async fn lookup_by_name(name: &str) -> Result<Icrs, AstroLookupError> {
    let sesame_config = SESAME_CONFIG.get_or_init(SesameConfig::init);
    let sesame_parser = SESAME_PARSER.get_or_init(init_sesame_parser);
    let client = Client::new();

    let mut err_result = Err(AstroLookupError::InvalidConfiguration {
        reason: String::from("No configured SESAME URLs"),
    });

    for url in &sesame_config.urls {
        let uri_string = [
            url.as_str(),
            if url.ends_with('/') { "" } else { "/" },
            "~",
            sesame_config.database.to_str(),
            "?",
            &encode(name),
        ]
        .concat();

        let result = lookup_by_uri(name, sesame_parser, &client, uri_string).await;

        if result.is_ok() {
            return result;
        } else {
            err_result = result;
        }
    }

    err_result
}

async fn lookup_by_uri(
    name: &str,
    sesame_parser: &Regex,
    client: &Client,
    uri_string: String,
) -> Result<Icrs, AstroLookupError> {
    let response = client.get(&uri_string).send().await?;
    let body_string = response.text().await?;

    if let Some(cap) = sesame_parser.captures(&body_string) {
        let ra_string = &cap[1];
        let dec_string = &cap[2];

        let ra: f64 = ra_string
            .parse()
            .map_err(|_| AstroLookupError::ParseError {
                reason: ["Could not parse ra value: ", ra_string].concat(),
            })?;
        let dec: f64 = dec_string
            .parse()
            .map_err(|_| AstroLookupError::ParseError {
                reason: ["Could not parse dec value: ", dec_string].concat(),
            })?;

        let coords = EquatorialCoord {
            ra: Angle::new::<degree>(ra),
            dec: Angle::new::<degree>(dec),
        };
        return Ok(Icrs { coords });
    }

    Err(AstroLookupError::InvalidName {
        name: name.to_owned(),
    })
}
