use std::env;
use std::error::Error;

/// An enumeration of SESAME database options
#[allow(missing_docs)]
pub enum SesameDatabase {
    /// Indicates all available databases should be queried
    All,
    Simbad,
    Ned,
    Vizier,
}

impl SesameDatabase {
    /// Returns the abbreviation for the database, usable in the query URL.
    pub fn to_str(&self) -> &str {
        match self {
            SesameDatabase::All => "A",
            SesameDatabase::Simbad => "S",
            SesameDatabase::Ned => "N",
            SesameDatabase::Vizier => "V",
        }
    }
}

impl TryFrom<String> for SesameDatabase {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "a" | "all" => Ok(SesameDatabase::All),
            "s" | "simbad" => Ok(Self::Simbad),
            "n" | "ned" => Ok(Self::Ned),
            "v" | "vizier" => Ok(Self::Vizier),
            _ => Err(["Unknown SESAME database option: ", &value].concat()),
        }
    }
}

/// Configuration for SESAME queries.
pub struct SesameConfig {
    /// The SESAME database to query.
    pub database: SesameDatabase,
    /// The URL at which the SESAME service is hosted.
    pub urls: Vec<String>,
}

impl Default for SesameConfig {
    fn default() -> Self {
        Self {
            database: SesameDatabase::All,
            urls: vec![
                String::from("http://cdsweb.u-strasbg.fr/cgi-bin/nph-sesame/"),
                String::from("http://vizier.cfa.harvard.edu/viz-bin/nph-sesame/"),
            ],
        }
    }
}

impl SesameConfig {
    /// Constructs a new SesameConfig. The associated environment variables
    /// (SESAME_DATABASE and SESAME_URLS) are examined first; if those
    /// variables cannot be parsed, a config with default values is returned.
    pub fn init() -> Self {
        Self::from_env().unwrap_or_default()
    }

    fn from_env() -> Result<Self, Box<dyn Error>> {
        let database = SesameDatabase::try_from(env::var("SESAME_DATABASE")?)?;
        let urls: Vec<String> = env::var("SESAME_URLS")?
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect();
        if urls.is_empty() {
            return Err("At least one SESAME URL must be present".into());
        }
        Ok(Self { database, urls })
    }
}
