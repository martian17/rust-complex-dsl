use proc_macro::{
    TokenStream,

    TokenTree,
    Group,
    Ident,
    Punct,
    Literal,

    Span,
};
use std::fmt::{Display, Write};
use num_complex::Complex;
use std::iter;
use either::Either;

pub enum ComplexTokenTree{
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal, f64),
    Imaginary(Span),
}

impl ComplexTokenTree{
    pub fn span(&self) -> Span {
        match self {
            ComplexTokenTree::Group(s) => s.span(),
            ComplexTokenTree::Ident(s) => s.span(),
            ComplexTokenTree::Punct(s) => s.span(),
            ComplexTokenTree::Literal(s, _) => s.span(),
            ComplexTokenTree::Imaginary(s) => s.clone(),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            ComplexTokenTree::Group(s) => s.to_string(),
            ComplexTokenTree::Ident(s) => s.to_string(),
            ComplexTokenTree::Punct(s) => s.to_string(),
            ComplexTokenTree::Literal(s, _) => s.to_string(),
            ComplexTokenTree::Imaginary(s) => "i".into(),
                // s.source_text().unwrap(),
        }
    }
}

// Trait wizardry to allow into_static_iterator
type either_det_iter = Either<
    std::array::IntoIter<ComplexTokenTree, 1>,
    std::array::IntoIter<ComplexTokenTree, 2>
>;
trait DeterministicIter {
    fn into_static_iterator(self) -> either_det_iter;
}
impl DeterministicIter for [ComplexTokenTree; 1]{
    fn into_static_iterator(self) -> either_det_iter {
        Either::Left(self.into_iter())
    }
}
impl DeterministicIter for [ComplexTokenTree; 2]{
    fn into_static_iterator(self) -> either_det_iter {
        Either::Right(self.into_iter())
    }
}


pub fn transform_stream (ts: TokenStream) -> impl Iterator<Item = ComplexTokenTree> {
    ts.into_iter().flat_map(|token| {
        match token {
            TokenTree::Group(group) => [ComplexTokenTree::Group(group)].into_static_iterator(),
            TokenTree::Ident(ident) => {
                let str = ident.to_string();
                if str == "i" {
                    [ComplexTokenTree::Imaginary(ident.span())].into_static_iterator()
                }else{
                    [ComplexTokenTree::Ident(ident)].into_static_iterator()
                }
            },
            TokenTree::Punct(punct) => [ComplexTokenTree::Punct(punct)].into_static_iterator(),
            TokenTree::Literal(literal) => {
                let str = literal.to_string();
                if str.ends_with('i') {
                    let value: f64 = str[0..(str.len() - 1)].parse().unwrap();
                    let mut value_literal = Literal::f64_unsuffixed(value);
                    value_literal.set_span(literal.span());
                    [
                        ComplexTokenTree::Literal(value_literal, value),
                        ComplexTokenTree::Imaginary(literal.span()),
                    ].into_static_iterator()
                }else{
                    let value: f64 = str.parse().unwrap();
                    [ComplexTokenTree::Literal(literal, value)].into_static_iterator()
                }
            },
        }
    })
}
