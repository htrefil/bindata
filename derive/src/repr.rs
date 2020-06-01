use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::Attribute;

pub enum Repr {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
}

impl Repr {
    pub fn parse(attrs: &[Attribute]) -> Result<Repr, &'static str> {
        let attr = attrs
            .iter()
            .find(|attr| attr.path.is_ident("repr"))
            .ok_or("no repr attribute has been found")?;

        let group = match attr.tokens.clone().into_iter().next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => group,
            _ => return Err("expected a parenthesis delimited group group"),
        };

        let ident = match group.stream().into_iter().next() {
            Some(TokenTree::Ident(ident)) => ident,
            _ => return Err("expected an ident"),
        };

        let repr = match ident.to_string().as_str() {
            "i8" => Repr::I8,
            "u8" => Repr::U8,
            "i16" => Repr::I16,
            "u16" => Repr::U16,
            "i32" => Repr::I32,
            "u32" => Repr::U32,
            "i64" => Repr::I64,
            "u64" => Repr::U64,
            _ => return Err("invalid repr attribute"),
        };

        Ok(repr)
    }
}

impl ToTokens for Repr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = match self {
            Repr::I8 => "i8",
            Repr::U8 => "u8",
            Repr::I16 => "i16",
            Repr::U16 => "u16",
            Repr::I32 => "i32",
            Repr::U32 => "u32",
            Repr::I64 => "i64",
            Repr::U64 => "u64",
        };

        tokens.append(Ident::new(ident, Span::call_site()));
    }
}
