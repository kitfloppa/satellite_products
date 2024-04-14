use syn::{self, Field, Type};

pub fn contains_attribute(field: &Field, name: &str) -> bool {
    return field
        .attrs
        .iter()
        .any(|it| it.path.get_ident().map(|it| it == name).unwrap_or(false));
}

pub fn is_copy_type(ty: &Type) -> bool {
    let well_known_types = [
        "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "i128", "u128", "isize", "usize",
        "f32", "f64", "bool", "char",
    ];

    if let Type::Path(tp) = ty {
        let segments = &tp.path.segments;
        if segments.len() != 1 {
            return false;
        }

        let ident = &segments[0].ident;
        for well_known_type in well_known_types {
            if ident == well_known_type {
                return true;
            }
        }
    }

    // TODO: Option<>, Id, Reference<>

    return false;
}
