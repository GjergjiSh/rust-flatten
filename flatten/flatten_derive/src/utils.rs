use quote::quote;
use syn::{Attribute, Lit, Meta, Type, TypeArray, TypePath};

pub fn parse_characteristic_attributes(field: &syn::Field) -> (String, i64, i64, String) {
    let mut comment = String::new();
    let mut min: i64 = 0;
    let mut max: i64 = 0;
    let mut unit = String::new();

    for attribute in &field.attrs {
        let ident = attribute.path.get_ident().unwrap().to_string();
        match ident.as_str() {
            "comment" => parse_comment(attribute, &mut comment),
            "min" => parse_min(attribute, &mut min),
            "max" => parse_max(attribute, &mut max),
            "unit" => parse_unit(attribute, &mut unit),
            _ => continue,
        }
    }

    (comment, min, max, unit)
}

#[inline]
pub fn parse_characteristic_type(ty: &Type) -> proc_macro2::TokenStream {
    if !is_array(ty) {
        quote! { CharacteristicType::VALUE }
    } else if is_multidimensional_array(ty) {
        quote! { CharacteristicType::MAP }
    } else if !is_multidimensional_array(ty) {
        quote! { CharacteristicType::CURVE }
    } else {
        panic!("Unexpected characteristic type");
    }
}

fn parse_unit(attribute: &Attribute, unit: &mut String) {
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

fn parse_max(attribute: &Attribute, max: &mut i64) {
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

fn parse_min(attribute: &Attribute, min: &mut i64) {
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

fn parse_comment(attribute: &Attribute, comment: &mut String) {
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

fn is_multidimensional_array(ty: &Type) -> bool {
    fn check_dimensions(ty: &Type) -> usize {
        match ty {
            Type::Array(TypeArray { elem, .. }) => 1 + check_dimensions(elem),
            _ => 0,
        }
    }

    check_dimensions(ty) > 1
}
