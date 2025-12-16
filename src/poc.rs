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

pub fn poc_macro(ts: TokenStream) -> TokenStream {
    // this macro returns Complex::new(2.3, 3.4) no matter what it takes in
    if let Some(token) = ts.into_iter().next() {
        return TokenStream::from_iter(vec![
            TokenTree::from(Ident::new("Complex", token.span())),
            TokenTree::from(Punct::new(':', Spacing::Joint)),
            TokenTree::from(Punct::new(':', Spacing::Joint)),
            TokenTree::from(Ident::new("new", token.span())),
            TokenTree::from(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vec![
                TokenTree::from(Literal::f64_suffixed(2.3)),
                TokenTree::from(Punct::new(',', Spacing::Alone)),
                TokenTree::from(Literal::f64_suffixed(3.4)),
            ]))),
        ]);
    }
    panic!("panicing");
}
