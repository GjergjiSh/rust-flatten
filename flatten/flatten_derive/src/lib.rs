extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Flatten)]
pub fn flatten_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => fields.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let characteristics = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap().to_string();
        // Assuming field.ty is of type syn::Type
        let datatype = match &field.ty {
            syn::Type::Path(type_path) => {
                // Extract the last segment as the type name
                let segment = type_path.path.segments.last().unwrap();
                segment.ident.to_string()
            },
            _ => unimplemented!("Unsupported field type"),
        };
        quote! {
            characteristics.push(Characteristic {
                name: String::from(#name),
                // Ensure datatype is treated as a String within the quote! block
                datatype: #datatype.to_string(),
            });
        }
    });

    let expanded = quote! {
        impl Flatten for #name {
            fn a2l_flatten(&self) -> Vec<Characteristic> {
                let mut characteristics = Vec::new();
                #(#characteristics)*
                characteristics
            }
        }
    };

    TokenStream::from(expanded)
}