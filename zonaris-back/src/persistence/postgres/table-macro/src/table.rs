use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Ident};

use crate::utils::contains_attribute;

const ID_ATTRIBUTE_NAME: &str = "id";

pub fn impl_table_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut id: Option<&Ident> = None;

    let mut fields_from_row = Vec::new();
    let mut to_col_val_pairs = Vec::new();
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            for field in &data_struct.fields {
                match &field.ident {
                    Some(ident) => {
                        let ident_string = ident.to_string();
                        fields_from_row.push(quote! { #ident: row.get(columns[#ident_string]) });

                        if contains_attribute(field, ID_ATTRIBUTE_NAME) {
                            id = Some(ident);
                            continue; // NOTE: id shouldn't be in "update" queries
                        }

                        to_col_val_pairs.push(
                            quote!( r.push(ColumnValuePair::new(#ident_string, self.#ident)); ),
                        );
                    }
                    None => panic!("not supported"),
                };
            }
        }
        _ => panic!("not supported"),
    }

    let gen_hasid = if let Some(ident) = id {
        quote! {
            impl crate::persistence::repository::HasId for #name {
                fn get_id(&self) -> Option<Id> {
                    return self.#ident;
                }

                fn set_id(&mut self, id: Id) {
                    self.#ident = Some(id);
                }
            }
        }
    } else {
        quote! {}
    };

    return quote! {
        #gen_hasid

        #[cfg(feature = "postgres")]
        use tokio_postgres::Row;

        #[cfg(feature = "postgres")]
        use crate::persistence::postgres::repository::ColumnValuePair;

        #[cfg(feature = "postgres")]
        impl TryFrom<Row> for #name {
            type Error = anyhow::Error;

            fn try_from(row: Row) -> anyhow::Result<Self> {
                let columns = row
                    .columns()
                    .iter()
                    .enumerate()
                    .map(|(i, col)| (col.name(), i))
                    .collect::<std::collections::HashMap<_, _>>();

                return Ok(Self {
                    #( #fields_from_row, )*
                });
            }
        }

        #[cfg(feature = "postgres")]
        impl TryInto<Vec<ColumnValuePair>> for #name {
            type Error = anyhow::Error;

            fn try_into(self) -> anyhow::Result<Vec<ColumnValuePair>> {
                let mut r = Vec::new();

                #( #to_col_val_pairs )*

                return Ok(r);
            }
        }
    };
}
