extern crate proc_macro;

mod utils;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};
use utils::{parse_characteristic_attributes, parse_characteristic_type};

// A2L Characteristics have a predefined extension which is 0
const XCP_ADDR_EXT_APP: u8 = 0;

#[proc_macro_derive(CharacteristicContainer, attributes(comment, min, max, unit))]
pub fn flatten_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data_type = &input.ident;

    let gen = match input.data {
        Data::Struct(data_struct) => {
            generate_characteristics_container_impl(data_struct, data_type)
        }
        _ => panic!("CharacteristicContainer macro only supports structs"),
    };

    gen.into()
}

fn generate_characteristics_container_impl(
    data_struct: syn::DataStruct,
    data_type: &syn::Ident,
) -> proc_macro2::TokenStream {
    let field_handlers = data_struct.fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let characteristic_type = parse_characteristic_type(field_type);
        let (comment, min, max, unit) = parse_characteristic_attributes(field);

        quote! {
            // Offset is the address of the field relative to the address of the struct
            let offset = ((&self.#field_name as *const _ as *const u8 as usize) - (self as *const _ as *const u8 as usize)) as u16;

            // Check if the type of the field implements the CharacteristicContainer trait
            // If this is the case, the characteristic is a nested struct and its name must
            // be prefixed by the name of the parent. Consider the following:
            // struct Parent { child : Child } -> the name of child field should be Parent.Child.child
            if let Some(nested_characteristics) = <#field_type as CharacteristicContainer>::characteristics(&self.#field_name) {
                characteristics.extend(nested_characteristics.into_iter().map(|mut characteristic| {
                    characteristic.set_name(format!("{}.{}", stringify!(#data_type), characteristic.name()));
                    characteristic
                }));
            // If the type does not implement the CharacteristicContainer trait,
            // we can simply create a new Characteristic from it
            } else {
                characteristics.push(Characteristic::new(
                    format!("{}.{}", stringify!(#data_type), stringify!(#field_name)),
                    stringify!(#field_type).to_string(),
                    #comment.to_string(),
                    #min,
                    #max,
                    #unit.to_string(),
                    #characteristic_type,
                    offset,
                    #XCP_ADDR_EXT_APP,
                ));
            }
        }
    });

    quote! {
        impl CharacteristicContainer for #data_type {
            fn characteristics(&self) -> Option<Vec<Characteristic>> {
                let mut characteristics = Vec::new();
                #(#field_handlers)*
                Some(characteristics)
            }
        }
    }
}
