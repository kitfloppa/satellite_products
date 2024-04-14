use syn::{self, Field};

pub fn contains_attribute(field: &Field, name: &str) -> bool {
    return field
        .attrs
        .iter()
        .any(|it| it.path.get_ident().map(|it| it == name).unwrap_or(false));
}
