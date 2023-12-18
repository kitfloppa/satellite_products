use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Table)]
pub fn table_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_table_macro(&ast)
}

fn impl_table_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut fields_from_row = Vec::new();
    let mut to_col_val_pairs = Vec::new();
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            for field in &data_struct.fields {
                match &field.ident {
                    Some(ident) => {
                        if ident == "id" {
                            continue;
                        }

                        let ident_string = ident.to_string();
                        fields_from_row.push(quote! { #ident: row.get(columns[#ident_string]) });
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

    let gen = quote! {
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
                    id: Some(row.get(columns["id"])),
                    #( #fields_from_row, )*
                });
            }
        }

        #[cfg(feature = "postgres")]
        impl TryInto<Vec<ColumnValuePair>> for #name {
            type Error = anyhow::Error;

            fn try_into(self) -> anyhow::Result<Vec<ColumnValuePair>> {
                let mut r = Vec::new();
                if let Some(id) = self.id {
                    r.push(ColumnValuePair::new("id", id));
                }

                #( #to_col_val_pairs )*

                return Ok(r);
            }
        }
    };

    gen.into()
}
