use crate::persistence::repository::{HasId, Id};

crate::pub_fields! {
    #[derive(Clone)]
    struct Instrument {
        id: Option<Id>,
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

impl HasId for Instrument {
    fn get_id(&self) -> Option<Id> {
        return self.id;
    }

    fn set_id(&mut self, id: Id) {
        self.id = Some(id);
    }
}
