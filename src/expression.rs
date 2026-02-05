use std::collections::HashMap;

use pest::iterators::Pairs;

use crate::{BUILT_IN_FUNCTIONS, BUILT_IN_NUMBERS, PRATT_PARSER, Rule, function::Function};

#[derive(Debug, Clone)]
pub enum Expression {
    Expression(Box<Expression>),
    Number(f64),
    Identifier(String),
    FunctionCall {
        identifier: String,
        arguments: Vec<Expression>,
    },
    InfixBinOp {
        left: Box<Expression>,
        op: InfixBinOp,
        right: Box<Expression>,
    },
    PrefixUnOp {
        op: PrefixUnOp,
        right: Box<Expression>,
    },
    PostfixUnOp {
        left: Box<Expression>,
        op: PostfixUnOp,
    },
}

impl Expression {
    pub fn parse(pairs: Pairs<Rule>) -> Self {
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::number => Expression::Number(primary.as_str().parse::<f64>().unwrap()),
                Rule::ident => Expression::Identifier(primary.as_str().to_string()),
                Rule::func_call => {
                    let mut pairs = primary.into_inner();

                    let pair = pairs.next().unwrap();

                    match pair.as_rule() {
                        Rule::ident => {
                            let identifier = pair.as_str().to_string();

                            let mut arguments = Vec::new();

                            for pair in pairs {
                                match pair.as_rule() {
                                    Rule::expr => {
                                        arguments.push(Expression::parse(pair.into_inner()))
                                    }
                                    rule => unreachable!("expected expression, found {:?}", rule),
                                }
                            }

                            Self::FunctionCall {
                                identifier,
                                arguments,
                            }
                        }
                        rule => unreachable!("expected identifier, found {:?}", rule),
                    }
                }
                Rule::expr => Expression::Expression(Box::new(Expression::parse(primary.into_inner()))),
                rule => unreachable!("Expr::parse expected number, found {:?}", rule),
            })
            .map_prefix(|op, right| {
                let op = match op.as_rule() {
                    Rule::neg => PrefixUnOp::Negate,
                    rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
                };

                Expression::PrefixUnOp {
                    op,
                    right: Box::new(right),
                }
            })
            .map_postfix(|left, op| {
                let op = match op.as_rule() {
                    Rule::fac => PostfixUnOp::Factorial,
                    rule => {
                        unreachable!("Expr::parse expected postfix operation, found {:?}", rule)
                    }
                };

                Expression::PostfixUnOp {
                    left: Box::new(left),
                    op,
                }
            })
            .map_infix(|left, op, right| {
                let op = match op.as_rule() {
                    Rule::add => InfixBinOp::Add,
                    Rule::sub => InfixBinOp::Subtract,
                    Rule::mul => InfixBinOp::Multiply,
                    Rule::div => InfixBinOp::Divide,
                    Rule::pow => InfixBinOp::Power,
                    Rule::modulo => InfixBinOp::Modulo,
                    rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
                };

                Expression::InfixBinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }
            })
            .parse(pairs)
    }

    pub fn eval(
        &self,
        functions: &HashMap<String, Function>,
        arguments: &HashMap<String, f64>,
    ) -> f64 {
        match self {
            Self::Expression(expression) => expression.eval(functions, arguments),
            Self::Number(number) => *number,
            Self::Identifier(identifier) => {
                *arguments.get(identifier).unwrap_or_else(|| {
                    BUILT_IN_NUMBERS
                        .get(identifier)
                        .expect(&format!("undefined identifier {}", identifier))
                })},
            Self::FunctionCall {
                identifier,
                arguments: call_arguments,
            } => {
                if let Some(function) = functions.get(identifier) {
                    function.eval(
                        functions,
                        &call_arguments
                            .iter()
                            .map(|argument| argument.eval(functions, arguments))
                            .collect::<Vec<f64>>(),
                    )
                } else {
                    let builtin = BUILT_IN_FUNCTIONS
                        .get(identifier)
                        .expect(&format!("undefined function {}", identifier));

                    builtin(
                        &call_arguments
                            .iter()
                            .map(|argument| argument.eval(functions, arguments))
                            .collect::<Vec<f64>>(),
                    )
                }
            }
            Self::PrefixUnOp { op, right } => op.eval(right.eval(functions, arguments)),
            Self::InfixBinOp { left, op, right } => op.eval(
                left.eval(functions, arguments),
                right.eval(functions, arguments),
            ),
            Self::PostfixUnOp { left, op } => op.eval(left.eval(functions, arguments)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InfixBinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

impl InfixBinOp {
    pub fn eval(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Subtract => left - right,
            Self::Multiply => left * right,
            Self::Divide => left / right,
            Self::Power => left.powf(right),
            Self::Modulo => left % right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PrefixUnOp {
    Negate,
}

impl PrefixUnOp {
    pub fn eval(&self, right: f64) -> f64 {
        match self {
            Self::Negate => -right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PostfixUnOp {
    Factorial,
}

impl PostfixUnOp {
    pub fn eval(&self, left: f64) -> f64 {
        match self {
            Self::Factorial => left,
        }
    }
}
