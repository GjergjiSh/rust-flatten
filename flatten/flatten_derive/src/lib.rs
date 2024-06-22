#![allow(unused_mut)]
#![allow(unused_variables)]

extern crate proc_macro;
use a2l_items::Characteristic;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Flatten)]
pub fn flatten_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let gen = match input.data {
        Data::Struct(data_struct) => {
            let field_handlers = data_struct.fields.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                let qualified_name = format!("{}.{}", struct_name, field_name);

                quote! {
                    characteristics.push(Characteristic {
                        name: #qualified_name.to_string(),
                        datatype: stringify!(#field_type).to_string(),
                    });

                    // Attempt to call to_a2l on the field if it implements ToA2l, appending its characteristics
                    if let Some(nested_characteristics) = <#field_type as Flatten>::to_a2l_optional(&self.#field_name) {
                        characteristics.extend(nested_characteristics.into_iter().map(|mut c| {
                            c.name = format!("{}.{}", #qualified_name, c.name);
                            c
                        }));
                    }
                }
            });

            quote! {
                impl Flatten for #struct_name {
                    fn a2l_flatten(&self) -> Vec<Characteristic> {
                        let mut characteristics = Vec::new();
                        #(#field_handlers)*
                        characteristics
                    }

                    // Optional to_a2l implementation, returns None for types that do not implement ToA2l
                    fn to_a2l_optional(&self) -> Option<Vec<Characteristic>> {
                        Some(self.a2l_flatten())
                    }
                }
            }
        },
        _ => panic!("Only structs are supported"),
    };

    gen.into()
}