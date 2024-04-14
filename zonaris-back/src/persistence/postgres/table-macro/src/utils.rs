use syn::{Field, GenericArgument, PathArguments, Type};

pub fn contains_attribute(field: &Field, name: &str) -> bool {
    return field
        .attrs
        .iter()
        .any(|it| it.path.get_ident().map(|it| it == name).unwrap_or(false));
}

pub fn is_copy_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        let segments = &tp.path.segments;
        if segments.len() != 1 {
            return false;
        }

        let segment = &segments[0];
        let ident = &segment.ident;
        for well_known_type in WELL_KNOWN_TYPES {
            if ident == well_known_type {
                return true;
            }
        }

        if ident == "Option" {
            if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
                if arguments.args.len() != 1 {
                    return false;
                }

                let arg = &arguments.args[0];
                if let GenericArgument::Type(nested_type) = arg {
                    return is_copy_type(nested_type);
                }
            }
        }
    }

    return false;
}

const WELL_KNOWN_TYPES: [&str; 18] = [
    "i8",
    "u8",
    "i16",
    "u16",
    "i32",
    "u32",
    "i64",
    "u64",
    "i128",
    "u128",
    "isize",
    "usize",
    "f32",
    "f64",
    "bool",
    "char",
    "Id",
    "Reference",
];
