mod repr;

use proc_macro::TokenStream;
use repr::Repr;
use std::usize;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Index};

fn encode_struct(data: DataStruct) -> proc_macro2::TokenStream {
    match data.fields {
        Fields::Unnamed(fields) => {
            let tys = fields.unnamed.iter().map(|field| &field.ty);
            let counter = (0..usize::MAX).map(Index::from);

            quote::quote! {
                #(<#tys as ::bindata::Encode>::encode(self.#counter, writer));*
            }
        }
        Fields::Named(fields) => {
            let tys = fields.named.iter().map(|field| &field.ty);
            let names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());

            quote::quote! {
                #(<#tys as ::bindata::Encode>::encode(self.#names, writer));*
            }
        }
        Fields::Unit => quote::quote! {},
    }
}

fn encode_enum(repr: Repr, data: DataEnum) -> proc_macro2::TokenStream {
    for variant in &data.variants {
        match variant.fields {
            Fields::Unit => {}
            _ => panic!("enum fields must not contain any data"),
        }
    }

    let names = data.variants.iter().map(|variant| &variant.ident);
    let discriminants = enum_discriminants(&data);

    quote::quote! {
        match self {
            #(Self::#names => writer.write::<#repr>(#discriminants),)*
        }
    }
}

fn enum_discriminants(data: &DataEnum) -> impl Iterator<Item = &Expr> {
    data.variants
        .iter()
        .map(|variant| match variant.discriminant.as_ref() {
            Some(discriminant) => &discriminant.1,
            None => panic!("enums must have explicit discriminants"),
        })
}

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let body = match input.data {
        Data::Struct(data) => encode_struct(data),
        Data::Enum(data) => {
            let repr = match Repr::parse(&input.attrs) {
                Ok(repr) => repr,
                Err(err) => panic!("failed to parse repr: {}", err),
            };

            encode_enum(repr, data)
        }
        Data::Union(_) => panic!("only structs and enums can #[derive(Encode)]"),
    };

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    (quote::quote! {
        impl #impl_generics ::bindata::Encode for #name #ty_generics #where_clause {
            fn encode(self, writer: &mut ::bindata::Writer) {
                #body
            }
        }
    })
    .into()
}

fn decode_struct(data: DataStruct) -> proc_macro2::TokenStream {
    match data.fields {
        Fields::Unnamed(fields) => {
            let tys = fields.unnamed.iter().map(|field| &field.ty);

            quote::quote! {
                Ok(Self(#(<#tys as ::bindata::Decode>::decode(reader)?),*))
            }
        }
        Fields::Named(fields) => {
            let names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());
            let tys = fields.named.iter().map(|field| &field.ty);

            quote::quote! {
                Ok(Self {
                    #(#names: <#tys as ::bindata::Decode>::decode(reader)?),*
                })
            }
        }
        Fields::Unit => quote::quote! { Ok(Self) },
    }
}

fn decode_enum(repr: Repr, data: DataEnum) -> proc_macro2::TokenStream {
    let names = data.variants.iter().map(|variant| &variant.ident);
    let discriminants = enum_discriminants(&data);

    quote::quote! {
        let value = reader.read::<#repr>()?;

        #(if value == #discriminants {
            return Ok(Self::#names);
        })*

        Err(::bindata::Error::InvalidVariant)
    }
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let body = match input.data {
        Data::Struct(data) => decode_struct(data),
        Data::Enum(data) => {
            let repr = match Repr::parse(&input.attrs) {
                Ok(repr) => repr,
                Err(err) => panic!("failed to parse repr: {}", err),
            };

            decode_enum(repr, data)
        }
        Data::Union(_) => panic!("only structs and enums can #[derive(Encode)]"),
    };

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    (quote::quote! {
        impl #impl_generics ::bindata::Decode for #name #ty_generics #where_clause {
            fn decode(reader: &mut ::bindata::Reader) -> Result<Self, ::bindata::Error> {
                #body
            }
        }
    })
    .into()
}

fn encoded_size_struct(data: DataStruct) -> proc_macro2::TokenStream {
    let fields = match data.fields {
        Fields::Unnamed(fields) => fields.unnamed,
        Fields::Named(fields) => fields.named,
        Fields::Unit => return quote::quote! { 0 },
    };

    let tys = fields.iter().map(|field| &field.ty);
    quote::quote! {
        0 #(+ <#tys as ::bindata::EncodedSize>::SIZE)*
    }
}

fn encoded_size_enum(repr: Repr) -> proc_macro2::TokenStream {
    quote::quote! { <#repr as ::bindata::EncodedSize>::SIZE  }
}

#[proc_macro_derive(EncodedSize)]
pub fn derive_encoded_size(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let body = match input.data {
        Data::Struct(data) => encoded_size_struct(data),
        Data::Enum(_) => {
            let repr = match Repr::parse(&input.attrs) {
                Ok(repr) => repr,
                Err(err) => panic!("failed to parse repr: {}", err),
            };

            encoded_size_enum(repr)
        }
        Data::Union(_) => panic!("only structs and enums can #[derive(EncodedSize)]"),
    };

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    (quote::quote! {
        impl #impl_generics ::bindata::EncodedSize for #name #ty_generics #where_clause {
            const SIZE: usize = #body;
        }
    })
    .into()
}
