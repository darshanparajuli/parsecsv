use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CsvSerialize)]
pub fn derive_csv_serialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let csv_deserialize = impl_csv_serialize(ast);
    TokenStream::from(csv_deserialize)
}

pub(crate) fn impl_csv_serialize(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let serialize_calls: Vec<_> = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let ident = &f.ident;
                if i < named.len() - 1 {
                    quote! {
                        self.#ident.serialize(w)?;
                        write!(w, ",")?;
                    }
                } else {
                    quote! {
                        self.#ident.serialize(w)?;
                    }
                }
            })
            .collect(),
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(syn::FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let index = syn::Index::from(i);
                if i < unnamed.len() - 1 {
                    quote! {
                        self.#index.serialize(w)?;
                        write!(w, ",")?;
                    }
                } else {
                    quote! {
                        self.#index.serialize(w)?;
                    }
                }
            })
            .collect(),
        _ => {
            panic!("Unsupported data")
        }
    };

    let expanded = quote! {
        impl Serializer for #name {
            fn serialize<'a>(&self, w: &'a mut dyn std::io::Write) -> Result<(), std::io::Error> {
                #(#serialize_calls)*
                Ok(())
            }
        }
    };

    expanded.into()
}
