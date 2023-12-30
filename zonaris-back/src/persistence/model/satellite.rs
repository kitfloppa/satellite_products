use anyhow::{Error, Result};
use table_macro::Table;

use crate::{
    persistence::repository::{HasId, Id},
    pub_fields,
    service::celestrak::TLE,
};

pub_fields! {
    #[derive(Clone, Table)]
    struct Satellite {
        #[id] id: Option<Id>,
        name: String,

        catnr: Option<i64>, // Satellite Catalog Number

        // TODO: make optional and maybe transfer to other table
        tle1: String,
        tle2: String,
    }
}

impl Satellite {
    // TODO: make tle optional
    pub fn new(name: &str, tle1: &str, tle2: &str) -> Result<Self> {
        return Satellite::try_from(TLE::new(name, tle1, tle2));
    }
}

impl TryFrom<TLE> for Satellite {
    type Error = Error;

    fn try_from(tle: TLE) -> Result<Self> {
        let catnr = tle.get_catnr()?;
        return Ok(Self {
            id: None,
            name: tle.name,

            catnr: Some(catnr.try_into()?),

            tle1: tle.tle1,
            tle2: tle.tle2,
        });
    }
}
