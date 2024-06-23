#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate proc_macro;
use a2l_items::Characteristic;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, Ident, Lit, Meta, NestedMeta, Type};
use std::ptr;

#[proc_macro_derive(Flatten, attributes(comment, min, max, unit))]
pub fn flatten_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data_type = &input.ident;

    let gen = match input.data {
        Data::Struct(data_struct) => {
            let field_handlers = data_struct.fields.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;

                is_tuple_type(field);

                let mut comment = String::new();
                let mut min: i64 = 0;
                let mut max: i64 = 0;
                let mut unit = String::new();

                for attribute in &field.attrs {
                    let ident = match attribute.path.get_ident().map(Ident::to_string) {
                        Some(ident) => ident,
                        None => continue, //TODO: Check if panic is better
                    };

                    match ident.as_str() {
                        "comment" => _parse_comment(attribute, &mut comment),
                        "min" => _parse_min(attribute, &mut min),
                        "max" => _parse_max(attribute, &mut max),
                        "unit" => _parse_unit(attribute, &mut unit),
                        _ => continue, //TODO: Check if panic is better
                    }
                }

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
                            comment: #comment.to_string(),
                            min: #min,
                            max: #max,
                            unit: #unit.to_string(),
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


fn _parse_unit(attribute: &Attribute, unit: &mut String) {
    let meta = attribute.parse_meta().unwrap_or_else(|e| {
        panic!("Failed to parse 'unit' attribute: {}", e);
    });


    let unit_str = match meta {
        Meta::NameValue(meta) => match meta.lit {
            Lit::Str(unit_str) => unit_str,
            _ => panic!("Expected a string literal for 'unit'"),
        },
        _ => panic!("Expected 'unit' attribute to be a name-value pair"),
    };


    *unit = unit_str.value();
}


fn _parse_max(attribute: &Attribute, max: &mut i64) {
    let meta = attribute.parse_meta().unwrap_or_else(|e| {
        panic!("Failed to parse 'max' attribute: {}", e);
    });


    let max_int = match meta {
        Meta::NameValue(meta) => match meta.lit {
            Lit::Int(max_int) => max_int,
            _ => panic!("Expected an integer literal for 'max'"),
        },
        _ => panic!("Expected 'max' attribute to be a name-value pair"),
    };


    *max = max_int.base10_parse::<i64>().unwrap();
}


fn _parse_min(attribute: &Attribute, min: &mut i64) {
    let meta = attribute.parse_meta().unwrap_or_else(|e| {
        panic!("Failed to parse 'min' attribute: {}", e);
    });


    let min_int = match meta {
        Meta::NameValue(meta) => match meta.lit {
            Lit::Int(min_int) => min_int,
            _ => panic!("Expected an integer literal for 'min'"),
        },
        _ => panic!("Expected 'min' attribute to be a name-value pair"),
    };


    *min = min_int.base10_parse::<i64>().unwrap();
}


fn _parse_comment(attribute: &Attribute, comment: &mut String) {
    let meta = attribute.parse_meta().unwrap_or_else(|e| {
        panic!("Failed to parse 'comment' attribute: {}", e);
    });


    let comment_str = match meta {
        Meta::NameValue(meta) => match meta.lit {
            Lit::Str(comment_str) => comment_str,
            _ => panic!("Expected a string literal for 'comment'"),
        },
        _ => panic!("Expected 'comment' attribute to be a name-value pair"),
    };


    *comment = comment_str.value();
}

fn is_tuple_type(field: &Field) -> bool {
    match &field.ty {
        Type::Tuple(_) => {println!("Tuple detected"); true},
        _ => false,
    }
}