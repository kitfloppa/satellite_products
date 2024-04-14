use table_macro::{Property, Table};

use crate::persistence::repository::Id;

#[derive(Clone, Table, Property)]
pub struct Instrument {
    #[id]
    #[none]
    id: Option<Id>,
    name: String,
}

impl Instrument {
    pub fn new(name: &str) -> Self {
        return Self {
            id: None,
            name: String::from(name),
        };
    }
}
