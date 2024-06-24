#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate proc_macro;
use a2l_items::{Characteristic, CharacteristicType};
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Field, Fields, GenericArgument, Ident, Lit, Meta, NestedMeta, PathArguments, Type, TypeArray, TypePath};
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
                let characteristic_type;


                let fname_str = field_name.as_ref().unwrap().to_string();
                if is_map(field_type) || is_array(field_type) {
                    characteristic_type = CharacteristicType::CURVE;
                    println!("{} is map or array", fname_str);
                }

                let dimensions = get_array_dimensions(&field.ty);
                if !dimensions.is_empty() {
                    println!("Array dimensions: {:?}", dimensions);
                    // For your example: map: [[i32; 9]; 8]
                    // This will print: Array dimensions: [8, 9]

                    if dimensions.len() >= 2 {
                        let x = dimensions[1]; // 9 in your example
                        let y = dimensions[0]; // 8 in your example
                        println!("X dimension: {}, Y dimension: {}", x, y);
                    }
                }

                if is_multidimensional_array(field_type) {
                    println!("{} is multidimensional array", fname_str)
                } else {
                    println!("{} is NOT multidimensional array", fname_str)
                }

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

fn is_array(field_type: &Type) -> bool {
    match field_type {
        Type::Array(_) => true,
        Type::Path(TypePath { path, .. }) => {
            if let Some(segment) = path.segments.last() {
                let type_name = segment.ident.to_string();

                // Check for Vec or Array
                if type_name == "Vec" || type_name == "Array" {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn is_map(field_type: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = field_type {
        if let Some(segment) = path.segments.last() {
            let type_name = segment.ident.to_string();

            // Check for HashMap
            if type_name == "HashMap" {
                return true;
            }

            // Check for other map types like BTreeMap
            if type_name.ends_with("Map") {
                return true;
            }
        }
    }
    false
}

//TODO: replace with option to abvoid allocating empty vec
fn get_array_dimensions(ty: &Type) -> Vec<usize> {
    match ty {
        Type::Array(TypeArray { elem, len, .. }) => {
            let mut dimensions = Vec::new();
            if let Expr::Lit(ExprLit { lit: Lit::Int(lit_int), .. }) = len {
                if let Ok(dim) = lit_int.base10_parse() {
                    dimensions.push(dim);
                }
            }
            // Recursively check for nested arrays
            dimensions.extend(get_array_dimensions(elem));
            dimensions
        },
        _ => Vec::new(),
    }
}


fn is_multidimensional_array(ty: &Type) -> bool {
    fn check_dimensions(ty: &Type) -> usize {
        match ty {
            Type::Array(TypeArray { elem, .. }) => {
                1 + check_dimensions(elem)
            },
            _ => 0,
        }
    }

    check_dimensions(ty) > 1
}