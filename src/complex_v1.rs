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

#[derive(Clone)]
enum BinaryOperator{
    Add(Punct),
    Sub(Punct),
    Mul(Punct),
    Div(Punct),
}

#[derive(Clone)]
enum UnaryOperator{
    Plus(Punct),
    Minus(Punct),
}

impl BinaryOperator {
    fn punct(&self) -> &Punct {
        match self {
            BinaryOperator::Add(p) => p,
            BinaryOperator::Sub(p) => p,
            BinaryOperator::Mul(p) => p,
            BinaryOperator::Div(p) => p,
        }
    }
}
 
// impl UnaryOperator {
//     fn punct(&self) -> &Punct {
//         match self {
//             UnaryOperator::Plus(p) => p,
//             UnaryOperator::Minus(p) => p,
//         }
//     }
// }

enum Operator{
    Binary(BinaryOperator),
    Unary(UnaryOperator),
}

impl Operator{
    pub fn precedence(&self) -> i16 {
        match self {
            Operator::Binary(bin) => match bin {
                BinaryOperator::Add(_) => 2,
                BinaryOperator::Sub(_) => 2,
                BinaryOperator::Mul(_) => 4,
                BinaryOperator::Div(_) => 4,
            },
            Operator::Unary(uni) => match uni {
                UnaryOperator::Plus(_) => 5,
                UnaryOperator::Minus(_) => 5,
            },
        }
    }
}

struct BinaryExpression{
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
}


struct UnaryExpression{
    operator: UnaryOperator,
    value: Box<Expression>,
}


struct ComplexValue{
    real: f64,
    imag: f64,
    span: Span,
}

struct IdentExpression{
    ident: Ident,
}

// RustExpression kann sowohl einzelner Identifier als auch group mit {} enthalten
struct RustExpression{
    group: Group,
}

struct Procedure{
    ident: Ident,
    args: Vec<Expression>,
}

fn to_string<T: Display>(token: &T) -> String{
    let mut str = String::new();
    write!(str, "{token}");
    return str;
}

// impl Procedure{
//     pub fn string(&self) -> String{
//         return to_string(self.ident);
//     }
// }

enum Expression{
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Value(ComplexValue),
    Rust(RustExpression),
    Ident(IdentExpression),
    Procedure(Procedure),
}

impl Expression{
    // fn is_constant(&self) -> bool {
    //     match self {
    //         Expression::Binary()
    //     }
    // }
    fn toTokenTree(&self) -> TokenTree {
        match self {
            Expression::Binary(expr) => TokenTree::Group(Group::new(
                Delimiter::None,
                TokenStream::from_iter(vec![
                    expr.left.toTokenTree(),
                    TokenTree::Punct(expr.operator.punct().clone()),
                    expr.right.toTokenTree(),
                ])
            )),
            Expression::Unary(expr) => {
                match &expr.operator {
                    UnaryOperator::Plus(_) => {
                        // Wenn es Plus ist, dann hat der Operator also keine Wirkung
                        expr.value.toTokenTree()
                    },
                    UnaryOperator::Minus(punct) => TokenTree::Group(Group::new(
                        Delimiter::None,
                        TokenStream::from_iter(vec![
                            TokenTree::Punct(punct.clone()),
                            expr.value.toTokenTree(),
                        ])
                    )),
                }
            },
            Expression::Value(value) => TokenTree::Group(Group::new(
                Delimiter::None,
                TokenStream::from_iter(vec![
                    TokenTree::from(Ident::new("Complex", value.span)),
                    TokenTree::from(Punct::new(':', Spacing::Joint)),
                    TokenTree::from(Punct::new(':', Spacing::Joint)),
                    TokenTree::from(Ident::new("new", value.span)),
                    TokenTree::from(Group::new(Delimiter::Parenthesis, TokenStream::from_iter(vec![
                        TokenTree::from(Literal::f64_suffixed(value.real)),
                        TokenTree::from(Punct::new(',', Spacing::Alone)),
                        TokenTree::from(Literal::f64_suffixed(value.imag)),
                    ]))),
                ])
            )),
            Expression::Rust(expr) => TokenTree::Group(expr.group.clone()),
            Expression::Ident(expr) => TokenTree::Ident(expr.ident.clone()),
            Expression::Procedure(proc) => {
                let str = to_string(&proc.ident);
                // Es werde später implementiert
                panic!("Unrecognized procedure {str}");
                // if str == "EPI" {
                //     if proc.args.len() !== 1 {
                //         panic!("In EPI, expected exactly one argument");
                //         let arg = proc.args[0];
                //         let mut portion = arg.toRealTokenTree();
                //     }
                // }else{
                //     panic!("Unrecognized procedure {str}");
                // }
            }
        }
    }
}

pub fn complex(ts: TokenStream) -> TokenStream {
    return TokenStream::from(dsl_helper(ts).toTokenTree());
}

fn dsl_parse_args(ts: TokenStream) -> Vec<Expression> {
    // temporary implementation
    return vec![
        dsl_helper(ts),
    ];
}

fn dsl_helper(ts: TokenStream) -> Expression {
    let mut values: Vec<Expression> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();

    fn reduce (precedence: i16, values: &mut Vec<Expression>, operators: &mut Vec<Operator>) {
        while !operators.is_empty() {
            let op_ref = operators.last().unwrap();
            if op_ref.precedence() < precedence {
                break;
            }
            // jetzt poppen, weil wir op benutzen
            let op = operators.pop().unwrap();
            let expr = match op {
                Operator::Binary(bin) => {
                    let right = values.pop().unwrap();
                    let left = values.pop().unwrap();
                    Expression::Binary(BinaryExpression{
                        operator: bin.clone(),
                        left: Box::new(left),
                        right: Box::new(right),
                    })
                },
                Operator::Unary(uni) => {
                    let value = values.pop().unwrap();
                    Expression::Unary(UnaryExpression{
                        operator: uni.clone(),
                        value: Box::new(value),
                    })
                },
            };
            values.push(expr);
        }
    }

    // Gibt an, ob die Letzte Token ein Operator war
    let mut was_operator = true;
    let mut iter = ts.clone().into_iter();
    while let Some(token) = iter.next() {
        match &token {
            TokenTree::Group(x) => {
                // RustExpression, einfacher Klammern, oder Procedure Argumenten
                match x.delimiter() {
                    Delimiter::Parenthesis => {
                        if was_operator {
                            // einfacher Klammern
                            let value = dsl_helper(x.stream());
                            values.push(value);
                        }else{
                            // Procedure Argumenten
                            let args = dsl_parse_args(x.stream());
                            if let Some(Expression::Ident(ident)) = values.pop() {
                                values.push(Expression::Procedure(Procedure {
                                    ident: ident.ident,
                                    args: args,
                                }))
                            } else {
                                panic!("Procedureaufruf hat versagen");
                            }
                        }
                    },
                    Delimiter::None => {
                        if !was_operator {
                            panic!("Erwartete einen Operator, aber ein Expression gefunden");
                        }
                        // Ist gleich einfacher Klammern
                        let value = dsl_helper(x.stream());
                        values.push(value);
                    },
                    Delimiter::Brace => {
                        if !was_operator {
                            panic!("Erwartete einen Operator, aber ein Expression gefunden");
                        }
                        values.push(Expression::Rust(RustExpression{
                            group: x.clone(),
                        }));
                    },
                    Delimiter::Bracket => {
                        panic!("Bracket ist hier illegal");
                    }
                }
                was_operator = false;
            },
            TokenTree::Ident(x) => {
                // Kann i oder allgemeiner Identifier sein
                let str = to_string(x);
                let value = if str == "i" {
                    Expression::Value(ComplexValue{
                        real: 0.0,
                        imag: 1.0,
                        span: x.span(),
                    })
                }else{
                    Expression::Ident(IdentExpression{
                        ident: x.clone(),
                    })
                };
                values.push(value);
                was_operator = false;
            },
            TokenTree::Punct(x) => {
                let operator = if !was_operator {
                    Operator::Binary(match x.as_char() {
                        '+' => BinaryOperator::Add(x.clone()),
                        '-' => BinaryOperator::Sub(x.clone()),
                        '*' => BinaryOperator::Mul(x.clone()),
                        '/' => BinaryOperator::Div(x.clone()),
                        _ => {
                            let x_char = x.as_char();
                            panic!("Unexpected operator {x_char}")
                        },
                    })
                } else {
                    Operator::Unary(match x.as_char() {
                        '+' => UnaryOperator::Plus(x.clone()),
                        '-' => UnaryOperator::Minus(x.clone()),
                        _ => {
                            let x_char = x.as_char();
                            panic!("Unexpected operator {x_char}")
                        },
                    })
                };
                reduce(operator.precedence(), &mut values, &mut operators);
                operators.push(operator);
                was_operator = true;
            },
            TokenTree::Literal(x) => {
                let str = to_string(x);
                let value = Expression::Value(if str.ends_with("i") {
                    let len = str.len();
                    ComplexValue {
                        real: 0.0,
                        imag: str[0..(len-1)].parse().unwrap(),
                        span: x.span(),
                    }
                } else {
                    ComplexValue {
                        real: str.parse().unwrap(),
                        imag: 0.0,
                        span: x.span(),
                    }
                });
                values.push(value);
                was_operator = false;
            }
        }
    }
    if was_operator {
        panic!("Expression darf nicht mit einer Operator enden");
    }
    reduce(0, &mut values, &mut operators);
    return values.pop().unwrap();
}
