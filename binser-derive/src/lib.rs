mod repr;

use proc_macro::TokenStream;
use repr::Repr;
use std::usize;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Generics, Index, Path, TraitBound,
    TraitBoundModifier, TypeParamBound,
};

fn add_trait_bounds(generics: &mut Generics, path: Path) {
    for param in generics.type_params_mut() {
        param.bounds.push(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: path.clone(),
        }));
    }
}

fn encode_struct(data: DataStruct) -> proc_macro2::TokenStream {
    match data.fields {
        Fields::Unnamed(fields) => {
            let tys = fields.unnamed.iter().map(|field| &field.ty);
            let counter = (0..usize::MAX).map(Index::from);

            quote::quote! {
                #(<#tys as ::binser::Encode>::encode(self.#counter, writer));*
            }
        }
        Fields::Named(fields) => {
            let tys = fields.named.iter().map(|field| &field.ty);
            let names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());

            quote::quote! {
                #(<#tys as ::binser::Encode>::encode(self.#names, writer));*
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
    let mut input = syn::parse_macro_input!(input as DeriveInput);
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

    add_trait_bounds(&mut input.generics, syn::parse_quote! { ::binser::Encode });

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    (quote::quote! {
        impl #impl_generics ::binser::Encode for #name #ty_generics #where_clause {
            fn encode(self, writer: &mut ::binser::Writer) {
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
                Ok(Self(#(<#tys as ::binser::Decode>::decode(reader)?),*))
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
                    #(#names: <#tys as ::binser::Decode>::decode(reader)?),*
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

        Err(::binser::Error::InvalidVariant)
    }
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as DeriveInput);
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

    add_trait_bounds(&mut input.generics, syn::parse_quote! { ::binser::Decode });

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    (quote::quote! {
        impl #impl_generics ::binser::Decode for #name #ty_generics #where_clause {
            fn decode(reader: &mut ::binser::Reader) -> Result<Self, ::binser::Error> {
                #body
            }
        }
    })
    .into()
}

fn size_struct(data: DataStruct) -> proc_macro2::TokenStream {
    let fields = match data.fields {
        Fields::Unnamed(fields) => fields.unnamed,
        Fields::Named(fields) => fields.named,
        Fields::Unit => return quote::quote! { 0 },
    };

    let tys = fields.iter().map(|field| &field.ty);
    quote::quote! {
        0 #(+ <#tys as ::binser::Size>::SIZE)*
    }
}

fn size_enum(repr: Repr) -> proc_macro2::TokenStream {
    quote::quote! { <#repr as ::binser::Size>::SIZE  }
}

#[proc_macro_derive(Size)]
pub fn derive_size(input: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(input as DeriveInput);
    let body = match input.data {
        Data::Struct(data) => size_struct(data),
        Data::Enum(_) => {
            let repr = match Repr::parse(&input.attrs) {
                Ok(repr) => repr,
                Err(err) => panic!("failed to parse repr: {}", err),
            };

            size_enum(repr)
        }
        Data::Union(_) => panic!("only structs and enums can #[derive(Size)]"),
    };

    add_trait_bounds(&mut input.generics, syn::parse_quote! { ::binser::Size });

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    (quote::quote! {
        impl #impl_generics ::binser::Size for #name #ty_generics #where_clause {
            const SIZE: usize = #body;
        }
    })
    .into()
}
