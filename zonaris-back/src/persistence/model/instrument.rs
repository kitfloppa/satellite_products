use table_macro::Table;

use crate::persistence::repository::{HasId, Id};

crate::pub_fields! {
    #[derive(Clone, Table)]
    struct Instrument {
        #[id] id: Option<Id>,
        name: String,
    }
}

impl Instrument {
    pub fn new(name: &str) -> Self {
        return Self {
            id: None,
            name: String::from(name),
        };
    }
}
