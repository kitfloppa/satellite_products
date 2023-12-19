// https://stackoverflow.com/questions/53866508/how-to-make-a-public-struct-where-all-fields-are-public-without-repeating-pub
#[macro_export]
macro_rules! pub_fields {
    {
        $(#[derive($($macros:tt),*)])*
        struct $name:ident {
            $(#[$field_macro_name:tt$(($field_macro_key:tt $(= $field_macro_value:tt)?))?])*
            $($field:ident: $t:ty,)*
        }
    } => {
        $(#[derive($($macros),*)])*
        pub struct $name {
            $(#[$field_macro_name$(($field_macro_key $(= $field_macro_value)?))?])*
            $(pub $field: $t),*
        }
    }
}

#[macro_export]
macro_rules! mapper {
    ($from_type:ty, $to_type:ty, {
        $($from_field:ident -> $to_field:ident,)*
    }) => {
        impl From<$from_type> for $to_type {
            fn from(data: $from_type) -> Self {
                if let Some(id) = data.id {
                    return Self {
                        id,
                        $($to_field: data.$from_field),*
                    };
                } else {
                    panic!("[{} -> {}] id should be presented!", stringify!($from_type), stringify!($to_type));
                }
            }
        }
    };
}
