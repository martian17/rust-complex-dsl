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
use std::fmt::{Display, Write};
use num_complex::Complex;

enum Operator{
    Add,
    Sub,
    Mul,
    Div,
    Plus,
    Minus,
}

impl Operator{
    pub fn precedence(&self) -> i16 {
        match self {
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 4,
            Operator::Div => 4,
            Operator::Plus => 5,
            Operator::Minus => 5,
        }
    }
}

enum BinaryOperator{
    Add,
    Sub,
    Mul,
    Div,
}

enum UnaryOperator{
    Plus,
    Minus,
}

trait ToAst{
    fn to_ast(&self) -> Ast;
}

struct ExprKind{
    Binary{
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary{
        op: UnaryOperator,
        expr: Box<Expression>
    },
    // want to reuse the ComplexValue, also aliasen wir es
    Literal(Complex),
    Ident(Ident),
    Call{
        func: Ident,
        args: Vec<Expression>,
    },
    Rust(TokenStream),
}

struct Expression{
    kind: ExprKind
    span: Span
}



enum ComplexTokenTree{
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    Imaginary(Span),
}

fn transform_stream (ts: TokenStream) -> Stream<> {
    
}

impl ToComplexTokenStream for TokenStream {
    fn to_complex_token_stream(&self) -> stream {
        

    }
}

// TokenStream -> AST -> Process AST -> TokenStream
pub fn complex_expression(ts: TokenStream) -> TokenStream {
}






