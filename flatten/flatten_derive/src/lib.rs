#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate proc_macro;
use a2l_items::Characteristic;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use std::ptr;

#[proc_macro_derive(Flatten)]
pub fn flatten_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data_type = &input.ident;

    let gen = match input.data {
        Data::Struct(data_struct) => {
            let field_handlers = data_struct.fields.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;

                quote! {
                    // Check if the field type implements Flatten and if so, call to_a2l_optional
                    if let Some(nested_characteristics) = <#field_type as Flatten>::a2l_flatten(&self.#field_name) {
                        characteristics.extend(nested_characteristics.into_iter().map(|mut c| {
                            // Correctly format the name for nested characteristics, ensuring they are prefixed correctly
                            c.name = format!("{}.{}", stringify!(#data_type), c.name);
                            c
                        }));
                    } else {
                        // Only add the characteristic for the field if it's not a nested structure implementing Flatten
                        characteristics.push(Characteristic {
                            name: format!("{}.{}", stringify!(#data_type), stringify!(#field_name)),
                            datatype: stringify!(#field_type).to_string(),
                        });
                    }
                }
            });

            quote! {
                impl Flatten for #data_type {
                    fn a2l_flatten(&self) -> Option<Vec<Characteristic>> {
                        let mut characteristics = Vec::new();
                        #(#field_handlers)*
                        Some(characteristics)
                    }

                    // fn to_a2l_optional(&self) -> Option<Vec<Characteristic>> {
                    //     Some(self.a2l_flatten())
                    // }
                }
            }
        }
        _ => panic!("Flatten macro only supports structs"),
    };

    gen.into()
}
