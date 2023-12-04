pub mod struct_utils;

pub type DynError = Box<dyn std::error::Error + Send + Sync>;
