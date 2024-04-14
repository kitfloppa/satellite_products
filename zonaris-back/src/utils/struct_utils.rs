// TODO: rid of
#[macro_export]
macro_rules! mapper {
    ($from_type:ty, $to_type:ty, {
        $($from_field:ident -> $to_field:ident,)*
    }) => {
        impl From<$from_type> for $to_type {
            fn from(data: $from_type) -> Self {
                if let Some(id) = data.get_id() {
                    return Self {
                        id,
                        $($to_field: data.$from_field().clone()),*
                    };
                } else {
                    panic!("[{} -> {}] id should be presented!", stringify!($from_type), stringify!($to_type));
                }
            }
        }
    };
}
