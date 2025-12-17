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

mod poc;
mod complex_v1;
mod complex_v2;

use poc::poc_macro as poc_macro_impl;
use complex_v1::complex as complex_v1_impl;
use complex_v2::complex_expression as complex_v2_impl;

#[proc_macro]
pub fn poc_macro(ts: TokenStream) -> TokenStream {
    poc_macro_impl(ts)
}

#[proc_macro]
pub fn complex_v1(ts: TokenStream) -> TokenStream {
    complex_v1_impl(ts)
}

#[proc_macro]
pub fn complex_v2(ts: TokenStream) -> TokenStream {
    complex_v2_impl(ts)
}
