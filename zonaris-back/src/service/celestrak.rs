use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use thiserror::Error;

use crate::pub_fields;

pub enum Query {
    /// Catalog Number (1 to 9 digits). Allows return of data for a single catalog number.
    CATNR(u32),

    /// International Designator (yyyy-nnn). Allows return of data for all objects associated with a particular launch.
    INTDES(String),

    /// Groups of satellites provided on the CelesTrak Current Data page.
    GROUP(String),

    /// Satellite Name. Allows searching for satellites by parts of their name.
    NAME(String),

    /// Special data sets for the GEO Protected Zone (GPZ) or GPZ Plus.
    SPECIAL(String),
}

pub_fields! {
    #[derive(Debug)]
    struct TLE {
        name: String,
        tle1: String,
        tle2: String,
    }
}

impl TLE {
    pub fn new(name: &str, tle1: &str, tle2: &str) -> TLE {
        return Self {
            name: String::from(name),
            tle1: String::from(tle1),
            tle2: String::from(tle2),
        };
    }
}

#[derive(Error, Debug)]
pub enum CelestrakError {
    #[error("invalid query")]
    InvalidQuery,
    #[error("invalid output format")]
    InvalidOutputFormat,
}

#[async_trait]
pub trait CelestrakService {
    async fn gp_query(&self, query: Query) -> Result<Vec<TLE>>;
}

pub struct CelestrakServiceDefault {}

impl CelestrakServiceDefault {
    pub fn new() -> Self {
        return Self {};
    }
}

#[async_trait]
impl CelestrakService for CelestrakServiceDefault {
    async fn gp_query(&self, query: Query) -> Result<Vec<TLE>> {
        let mut params = HashMap::<&str, String>::new();
        match query {
            Query::CATNR(catnr) => params.insert("CATNR", catnr.to_string()),
            Query::INTDES(intdes) => params.insert("INTDES", intdes),
            Query::GROUP(group) => params.insert("GROUP", group),
            Query::NAME(name) => params.insert("NAME", name),
            Query::SPECIAL(special) => params.insert("SPECIAL", special),
        };
        params.insert("FORMAT", String::from("TLE"));

        let response = reqwest::Client::new()
            .get("https://celestrak.org/NORAD/elements/gp.php")
            .query(&params)
            .send()
            .await?;

        let text = response.text().await?;
        let lines = text.lines().collect::<Vec<_>>();

        if lines.len() == 1 {
            if lines[0] == "No GP data found" {
                return Ok(Vec::new());
            } else if lines[0] == "Invalid query: \"\"" {
                // TODO: maybe in some cases there error between ""?
                return Err(anyhow!(CelestrakError::InvalidQuery));
            }
        }

        if lines.len() % 3 != 0 {
            return Err(anyhow!(CelestrakError::InvalidOutputFormat));
        }

        let tles = lines
            .chunks_exact(3)
            .map(|slice| TLE::new(slice[0], slice[1], slice[2]))
            .collect::<Vec<_>>();

        return Ok(tles);
    }
}
