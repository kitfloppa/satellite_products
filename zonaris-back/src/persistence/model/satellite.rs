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
        tle1: String,
        tle2: String,
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
