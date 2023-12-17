use std::collections::HashMap;

use anyhow::{Error, Result};

use crate::{
    persistence::repository::{HasId, Id},
    pub_fields,
    service::celestrak::TLE,
};

pub_fields! {
    #[derive(Clone)]
    struct Satellite {
        id: Option<Id>,
        name: String,
        tle1: String,
        tle2: String,
    }
}

// TODO: to macro or even better to attribute

#[cfg(feature = "postgres")]
use tokio_postgres::Row;

#[cfg(feature = "postgres")]
impl TryFrom<Row> for Satellite {
    type Error = Error;

    fn try_from(row: Row) -> Result<Self> {
        let columns = row
            .columns()
            .iter()
            .enumerate()
            .map(|(i, col)| (col.name(), i))
            .collect::<HashMap<_, _>>();

        return Ok(Self {
            id: Some(row.get(columns["id"])),
            name: row.get(columns["name"]),
            tle1: row.get(columns["tle1"]),
            tle2: row.get(columns["tle2"]),
        });
    }
}

impl HasId for Satellite {
    fn get_id(&self) -> Option<Id> {
        return self.id;
    }

    fn set_id(&mut self, id: Id) {
        self.id = Some(id);
    }
}

impl Satellite {
    pub fn new(name: &str, tle1: &str, tle2: &str) -> Self {
        return Self {
            id: None,
            name: String::from(name),
            tle1: String::from(tle1),
            tle2: String::from(tle2),
        };
    }
}

impl From<TLE> for Satellite {
    fn from(tle: TLE) -> Self {
        return Self {
            id: None,
            name: tle.name,
            tle1: tle.tle1,
            tle2: tle.tle2,
        };
    }
}
