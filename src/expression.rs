use pest::Parser;

use crate::{
    BUILT_IN_FUNCTIONS, BUILT_IN_NUMBERS, MusathParser, PRATT_PARSER, Rule, context::{self, Context},
};

#[derive(Debug, PartialEq, Clone)]
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
}

impl Expression {
    pub fn parse(input: &str) -> Self {
        let expression = MusathParser::parse(Rule::expr, input)
            .unwrap()
            .next()
            .unwrap();

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
                                    Rule::expr => arguments.push(Expression::parse(pair.as_str())),
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
                Rule::expr => Expression::Expression(Box::new(Expression::parse(primary.as_str()))),
                rule => unreachable!("Expr::parse expected number, found {:?}", rule),
            })
            .map_prefix(|op, right| {
                let op = match op.as_rule() {
                    Rule::neg => PrefixUnOp::Negate,
                    rule => unreachable!("expected prefix operation, found {:?}", rule),
                };

                Expression::PrefixUnOp {
                    op,
                    right: Box::new(right),
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
                    rule => unreachable!("expected infix operation, found {:?}", rule),
                };

                Expression::InfixBinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }
            })
            .parse(expression.into_inner())
    }

    pub fn eval(&self, context: &Context) -> f64 {
        match self {
            Self::Expression(expression) => expression.eval(context),
            Self::Number(number) => *number,
            Self::Identifier(identifier) => *context
                .values()
                .get(identifier)
                .unwrap()
                .last()
                .unwrap_or_else(|| {
                    BUILT_IN_NUMBERS
                        .get(identifier)
                        .expect(&format!("undefined identifier {}", identifier))
                }),
            Self::FunctionCall {
                identifier,
                arguments,
            } => {
                if let Some(function) = context.functions().get(identifier) {
                    let mut inner_context = context.clone();

                    for (identifier, value) in function.signature().parameters().iter().zip(arguments.iter()) {
                        inner_context.push_value(identifier, value.eval(&context));
                    }

                    function.eval(&inner_context)
                } else {
                    let builtin = BUILT_IN_FUNCTIONS
                        .get(identifier)
                        .expect(&format!("undefined function {}", identifier));

                    builtin(context, arguments)
                }
            }
            Self::PrefixUnOp { op, right } => op.eval(right.eval(context)),
            Self::InfixBinOp { left, op, right } => {
                op.eval(left.eval(context), right.eval(context))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_expression() {
        let mut context = Context::new();

        assert_eq!(Expression::parse("1.0").eval(&context), 1.0);

        context.push_value("t", 2.0);

        assert_eq!(Expression::parse("t^2").eval(&context), 4.0);

        context.push_value("pi", PI);

        assert_eq!(Expression::parse("sin(2*pi)").eval(&context), 0.0);
    }
}
