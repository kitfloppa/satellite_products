use crate::repository::{HasId, Id};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Satellite {
    id: Option<Id>,
    name: String,
    tle1: String,
    tle2: String,
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
    pub fn new(name: &str, tle1: &str, tle2: &str) -> Satellite {
        Satellite {
            id: None,
            name: String::from(name),
            tle1: String::from(tle1),
            tle2: String::from(tle2),
        }
    }
}
