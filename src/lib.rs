extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed};

#[derive(Debug)]
struct FieldInfo {
    name: proc_macro2::Ident,
}

fn gen_serialize(fi: &FieldInfo) -> proc_macro2::TokenStream {
    let name = &fi.name;
    quote! {
        tup.serialize_element(&self.#name)?;
    }
}

fn extract_field_info(s: DataStruct) -> Vec<FieldInfo> {
    let mut result = Vec::new();
    if let Fields::Named(FieldsNamed { named, .. }) = s.fields {
        for field in named {
            match field.ident {
                Some(ident) => {
                    let fi = FieldInfo { name: ident };
                    result.push(fi);
                }
                None => panic!("couldn't find struct field ident"),
            }
        }
    } else {
        panic!("couldn't extract named fields");
    }
    result
}

#[proc_macro_derive(SerializeCompact)]
pub fn derive(input: TokenStream) -> TokenStream {
    // parse the input into a DeriveInput syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let field_info;
    if let syn::Data::Struct(struc) = input.data {
        field_info = extract_field_info(struc);
    } else {
        panic!("SerializeCompact can only be used on structs");
    }

    let element_count = field_info.len();
    let serialize_elements = field_info.iter().map(|fi| gen_serialize(fi));

    let expanded = quote! {
        #[automatically_derived]
        impl ::serde::Serialize for #struct_name {
            fn serialize<__S>(&self, __serializer: __S) -> Result<__S::Ok, __S::Error>
            where
                __S: ::serde::Serializer
            {
                use ::serde::ser::{SerializeTuple};
                let mut tup = __serializer.serialize_tuple(#element_count)?;
                #(#serialize_elements)*
                tup.end()
            }
        }
    };
    // proc_macro2::TokenStream -> proc_macro::TokenStream
    expanded.into()
}
