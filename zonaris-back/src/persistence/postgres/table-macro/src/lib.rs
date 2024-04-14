use proc_macro::TokenStream;
use table::impl_table_macro;

use crate::property::impl_property_macro;

mod property;
mod table;
mod utils;

#[proc_macro_derive(Table, attributes(id))]
pub fn table_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_table_macro(&ast).into();
}

#[proc_macro_derive(Property, attributes(getter, setter, none))]
pub fn property_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    return impl_property_macro(&ast).into();
}
