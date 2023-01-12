use crate::tokens::{Delimeter, TokenTree, TokenTreeTy};
use crate::TokenStream;

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Some(some) => some.to_tokens(tokens),
            None => {}
        }
    }
}

pub struct Delimited {
    tt: TokenTree,
}

impl ToTokens for Delimited {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend_one(self.tt.clone())
    }
}

#[inline(always)]
fn delimited<T: ToTokens>(inner: &T, delim: Delimeter) -> Delimited {
    let mut ts = TokenStream::new_empty();
    inner.to_tokens(&mut ts);
    let tt = TokenTree {
        col: 0,
        row: 0,
        ty: TokenTreeTy::Group(delim, ts.into_inner()),
    };
    Delimited { tt }
}

pub fn parenthesized<T: ToTokens>(inner: &T) -> Delimited {
    delimited(inner, Delimeter::Paren)
}

pub fn bracketed<T: ToTokens>(inner: &T) -> Delimited {
    delimited(inner, Delimeter::Bracket)
}

pub fn braced<T: ToTokens>(inner: &T) -> Delimited {
    delimited(inner, Delimeter::Brace)
}

pub struct Multiple {
    ts: TokenStream,
}

impl ToTokens for Multiple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(&self.ts)
    }
}

pub fn multiple(mut f: impl FnMut(&mut TokenStream)) -> Multiple {
    let mut ts = TokenStream::new_empty();
    f(&mut ts);
    Multiple { ts }
}

pub fn to_tokens<T: ToTokens>(v: &T) -> TokenStream {
    let mut ts = TokenStream::new_empty();
    v.to_tokens(&mut ts);
    ts
}
