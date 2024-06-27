#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

extern crate proc_macro;
// use flatten::{Characteristic, CharacteristicType};
use proc_macro::TokenStream;
use quote::quote;
use std::ptr;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Expr, ExprLit, Field, Fields, GenericArgument,
    Ident, Lit, Meta, NestedMeta, PathArguments, Type, TypeArray, TypePath,
};

const XCP_ADDR_EXT_APP: u8 = 0;

fn xcp_get_cal_addr_base(calseg_index: usize) -> u32 {
    (((calseg_index as u32) + 1) | 0x8000) << 16 // Address format for calibration segment field is index | 0x8000 in high word, addr_ext is 0 (CANape does not support addr_ext in memory segments)
}

fn xcp_get_cal_ext_addr(offset: u16) -> u32 {
    let calseg_index = 1; // Xcp::get().get_calseg_index(calseg_name);
    let a2l_addr: u32 = offset as u32 + xcp_get_cal_addr_base(calseg_index);
    // dbg!(a2l_addr);
    // dbg!(a2l_ext);
    println!("0x{:X}", a2l_addr);
    a2l_addr
}

#[proc_macro_derive(Flatten, attributes(comment, min, max, unit))]
pub fn flatten_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data_type = &input.ident;

    let gen = match input.data {
        Data::Struct(data_struct) => {
            let field_handlers = data_struct.fields.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let characteristic_type = get_characteristic_type(field_type);

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
                    //TODO: QST Rainer.
                    // let ext = 0; //XCP_ADDR_EXT_APP
                    let offset = ((&self.#field_name as *const _ as *const u8 as usize) - (self as *const _ as *const u8 as usize)) as u16;
                    // let calseg_idx: usize = 0; // TIGHT Coupling to XCP
                    // let a2l_addr: u32 = offset as u32 + ((((calseg_idx as u32) + 1) | 0x8000) << 16);
                    // dbg!(a2l_addr);
                    // println!("INSIDE IMPL 0x{:X}", a2l_addr);

                    // Check if the field type implements Flatten and if so, call a2l_flatten
                    if let Some(nested_characteristics) = <#field_type as Flatten>::a2l_flatten(&self.#field_name) {
                        // dbg!(&self.#field_name);
                        characteristics.extend(nested_characteristics.into_iter().map(|mut c| {
                            // Correctly format the name for nested characteristics, ensuring they are prefixed correctly
                            c.name = format!("{}.{}", stringify!(#data_type), c.name);
                            c
                        }));
                    } else {
                        // dbg!(&self.#field_name);
                        // Only add the characteristic for the field if it's not a nested structure implementing Flatten
                        characteristics.push(Characteristic {
                            name: format!("{}.{}", stringify!(#data_type), stringify!(#field_name)),
                            datatype: stringify!(#field_type).to_string(),
                            comment: #comment.to_string(),
                            min: #min,
                            max: #max,
                            unit: #unit.to_string(),
                            characteristic_type: #characteristic_type,
                            offset: offset,
                            extension: 0 //XCP_ADDR_EXT_APP
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
        Type::Tuple(_) => {
            // println!("Tuple detected");
            true
        }
        _ => false,
    }
}

//TODO: Check if having type vec is realistic
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
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) = len
            {
                if let Ok(dim) = lit_int.base10_parse() {
                    dimensions.push(dim);
                }
            }
            // Recursively check for nested arrays
            dimensions.extend(get_array_dimensions(elem));
            dimensions
        }
        _ => Vec::new(),
    }
}

fn is_multidimensional_array(ty: &Type) -> bool {
    fn check_dimensions(ty: &Type) -> usize {
        match ty {
            Type::Array(TypeArray { elem, .. }) => 1 + check_dimensions(elem),
            _ => 0,
        }
    }

    check_dimensions(ty) > 1
}

#[inline]
fn get_characteristic_type(ty: &Type) -> proc_macro2::TokenStream {
    if !is_array(ty) {
        quote! { CharacteristicType::VALUE }
    } else if is_multidimensional_array(ty) {
        quote! { CharacteristicType::MAP }
    } else {
        quote! { CharacteristicType::CURVE }
    }
}