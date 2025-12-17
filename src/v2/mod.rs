use proc_macro::{
    TokenStream,

    TokenTree,
    Group,
    Ident,
    Punct,
    Literal,

    Span,

    Spacing,
    Delimiter
};
use std::iter;
use either::Either;

mod pre_processing;
use pre_processing::transform_stream; 

// TokenStream -> AST -> Process AST -> TokenStream
pub fn complex_expression(ts: TokenStream) -> TokenStream {
    let mut complex_token_stream = transform_stream(ts.clone()).into_iter();
    while let Some(token) = complex_token_stream.next() {
        println!("{}", token.to_string());
    }
    // return a dummy output
    return TokenStream::from_iter(vec![
        TokenTree::from(Ident::new("Complex", Span::call_site())),
        TokenTree::from(Punct::new(':', Spacing::Joint)),
        TokenTree::from(Punct::new(':', Spacing::Joint)),
        TokenTree::from(Ident::new("new", Span::call_site())),
        TokenTree::from(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vec![
            TokenTree::from(Literal::f64_suffixed(2.3)),
            TokenTree::from(Punct::new(',', Spacing::Alone)),
            TokenTree::from(Literal::f64_suffixed(3.4)),
        ]))),
    ]);
}
