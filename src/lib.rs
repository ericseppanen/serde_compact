extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed};

#[derive(Debug)]
struct FieldInfo {
    name: proc_macro2::Ident,
    ty: syn::Type,
}

fn gen_serialize(fi: &FieldInfo) -> proc_macro2::TokenStream {
    let name = &fi.name;
    quote! {
        tup.serialize_element(&self.#name)?;
    }
}

fn gen_tostruct_field(index: usize, fi: &FieldInfo) -> proc_macro2::TokenStream {
    let name = &fi.name;
    let index = syn::Index::from(index);
    quote! {
        #name: self.#index,
    }
}

fn gen_tuplestruct_field(fi: &FieldInfo) -> proc_macro2::TokenStream {
    let ty = &fi.ty;
    quote! {
        #ty,
    }
}

fn extract_field_info(s: DataStruct) -> Vec<FieldInfo> {
    let mut result = Vec::new();
    if let Fields::Named(FieldsNamed { named, .. }) = s.fields {
        for field in named {
            let result_name;

            // extract the field name
            match field.ident {
                Some(ident) => {
                    result_name = ident;
                }
                None => panic!("couldn't find struct field ident"),
            }

            // extract the field type
            let result_ty = field.ty;

            let fi = FieldInfo {
                name: result_name,
                ty: result_ty,
            };
            result.push(fi);
        }
    } else {
        panic!("couldn't extract named fields");
    }
    result
}

#[proc_macro_derive(SerializeCompact)]
pub fn derive_ser(input: TokenStream) -> TokenStream {
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

// DeserializeCompact defines an intermediate tuple-struct, and
// runs #[derive(Deserialize)] on that.  We then define our own
// Deserialise wrapper that deserializes the tuple-struct and
// performs a final conversion.
#[proc_macro_derive(DeserializeCompact)]
pub fn derive_deser(input: TokenStream) -> TokenStream {
    // parse the input into a DeriveInput syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // The name of the struct
    let struct_name = input.ident;
    // The name of the tuple struct we use as an intermediate value
    let tuplestruct_name = format_ident!("__{}__AsTuple", struct_name);
    // The name of a function that converts from tuple-struct to struct
    let converter_name = format_ident!("__To__{}", struct_name);

    // extract a FieldInfo from each field in the struct
    let field_info;
    if let syn::Data::Struct(struc) = input.data {
        field_info = extract_field_info(struc);
    } else {
        panic!("DeserializeCompact can only be used on structs");
    }

    // an iterator that produces the fields in the tuple-struct declaration
    let tuplestruct_fields = field_info.iter().map(|fi| gen_tuplestruct_field(fi));
    // an iterator that produces the struct field assignements during conversion
    let tostruct_fields = field_info
        .iter()
        .enumerate()
        .map(|(idx, fi)| gen_tostruct_field(idx, fi));

    let expanded = quote! {
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            non_camel_case_types,
            non_snake_case
        )]
        const _: () = {
            #[allow(rust_2018_idioms, clippy::useless_attribute)]
            extern crate serde as _serde;

            // Define a tuple-struct with the same field types as our original struct
            #[automatically_derived]
            #[derive(::serde_derive::Deserialize)]
            struct #tuplestruct_name(
                #(#tuplestruct_fields)*
            );

            // A function that converts from tuple-struct to the original struct
            #[automatically_derived]
            impl #tuplestruct_name {
                pub fn #converter_name(self) -> #struct_name {
                    #struct_name {
                        #(#tostruct_fields)*
                    }
                }
            }

            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #struct_name {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: ::serde::Deserializer<'de>,
                {
                    // Deserialize as a tuple, then convert into the desired struct.
                    Ok(#tuplestruct_name::deserialize(__deserializer)?.#converter_name())
                }
            }
        };
    };
    // proc_macro2::TokenStream -> proc_macro::TokenStream
    expanded.into()
}
