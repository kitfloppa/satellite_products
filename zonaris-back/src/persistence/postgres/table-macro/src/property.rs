extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{self, Field, Ident};

use crate::utils::{contains_attribute, is_copy_type};

const SETTER_ATTRIBUTE_NAME: &str = "setter";
const GETTER_ATTRIBUTE_NAME: &str = "getter";
const NONE_ATTRIBUTE_NAME: &str = "none";

const SETTER_PREFIX: &str = "set_";
const GETTER_PREFIX: &str = "get_";

enum PropertyKind {
    Auto,
    None,
    Getter,
    Setter,
    Full,
}

struct FieldInfo<'a> {
    name: String,
    kind: PropertyKind,
    field: &'a Field,
    field_ident: &'a Ident,
}

fn collect_field_info<'a>(ast: &'a syn::DeriveInput) -> Vec<FieldInfo<'a>> {
    let mut result = Vec::new();

    match &ast.data {
        syn::Data::Struct(data_struct) => {
            for field in &data_struct.fields {
                match &field.ident {
                    Some(ident) => {
                        let name = ident.to_string();

                        let is_setter = contains_attribute(field, SETTER_ATTRIBUTE_NAME);
                        let is_getter = contains_attribute(field, GETTER_ATTRIBUTE_NAME);
                        let is_none = contains_attribute(field, NONE_ATTRIBUTE_NAME);

                        if is_none && (is_setter || is_getter) {
                            panic!("incorrect attribute usage")
                        }

                        let mut kind = PropertyKind::Auto;

                        if is_none {
                            kind = PropertyKind::None;
                        }

                        if is_setter && is_getter {
                            kind = PropertyKind::Full;
                        } else if is_setter {
                            kind = PropertyKind::Setter;
                        } else if is_getter {
                            kind = PropertyKind::Getter;
                        }

                        result.push(FieldInfo {
                            name,
                            kind,
                            field,
                            field_ident: &ident,
                        });
                    }
                    None => panic!("not supported"),
                };
            }
        }
        _ => panic!("not supported"),
    }

    return result;
}

pub fn impl_property_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut accessors = Vec::new();
    for field_info in collect_field_info(ast) {
        fn push_setter(accessors: &mut Vec<TokenStream>, field_info: &FieldInfo) {
            let parameter_name = Ident::new(field_info.name.as_str(), Span::call_site());
            let parameter_type = &field_info.field.ty;

            let field_ident = field_info.field_ident;

            let mut name = field_info.name.clone();
            name.insert_str(0, SETTER_PREFIX);

            let name = Ident::new(name.as_str(), Span::call_site());
            accessors.push(quote! {
                    pub fn #name(&mut self, #parameter_name: #parameter_type) {
                        self.#field_ident = #parameter_name;
                    }
            });
        }
        fn push_getter(accessors: &mut Vec<TokenStream>, field_info: &FieldInfo) {
            let return_type = &field_info.field.ty;

            let field_ident = field_info.field_ident;

            let mut name = field_info.name.clone();
            name.insert_str(0, GETTER_PREFIX);

            let name = Ident::new(name.as_str(), Span::call_site());

            accessors.push(if is_copy_type(return_type) {
                quote! {
                    pub fn #name(&self) -> #return_type {
                        return self.#field_ident;
                    }
                }
            } else {
                quote! {
                    pub fn #name(&self) -> &#return_type {
                        return &self.#field_ident;
                    }
                }
            });
        }

        match field_info.kind {
            PropertyKind::Auto => {
                push_getter(&mut accessors, &field_info);
                push_setter(&mut accessors, &field_info);
            }
            PropertyKind::None => {}
            PropertyKind::Getter => push_getter(&mut accessors, &field_info),
            PropertyKind::Setter => push_setter(&mut accessors, &field_info),
            PropertyKind::Full => {
                push_getter(&mut accessors, &field_info);
                push_setter(&mut accessors, &field_info);
            }
        }
    }

    return quote! {
        impl #name {
            #( #accessors )*
        }
    };
}
