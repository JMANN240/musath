use std::{fmt::Debug, sync::Arc};

use pest::iterators::Pairs;

use crate::{Rule, context::Context, expression::Expression};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    signature: FunctionSignature,
    body: FunctionBody,
}

impl Function {
    pub fn new(identifier: impl Into<String>, body: FunctionBodyClosure) -> Self {
        Self {
            signature: FunctionSignature {
                identifier: identifier.into(),
                parameters: Vec::new(),
            },
            body: FunctionBody::Closure(body),
        }
    }

    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let mut signature = None;
        let mut expression = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::function_signature => {
                    signature = Some(FunctionSignature::parse(&mut pair.into_inner()));
                }
                Rule::expression => {
                    expression = Some(Expression::parse(&mut pair.into_inner()));
                }
                rule => unreachable!(
                    "expected function identifier or expression, found {:?}",
                    rule
                ),
            }
        }

        Self {
            signature: signature.unwrap(),
            body: FunctionBody::Expression(expression.unwrap()),
        }
    }

    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn body(&self) -> &FunctionBody {
        &self.body
    }

    pub fn eval(&self, arguments: &[Box<Expression>], context: &Context) -> f64 {
        self.body().eval(arguments, context)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionSignature {
    identifier: String,
    parameters: Vec<String>,
}

impl FunctionSignature {
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
    pub fn parameters(&self) -> &Vec<String> {
        &self.parameters
    }

    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let identifier = pairs.next().unwrap().as_str().to_string();

        let parameters = pairs
            .map(|pair| match pair.as_rule() {
                Rule::identifier => pair.as_str().to_string(),
                _ => unreachable!("expected identifier, found {:?}", pair),
            })
            .collect();

        Self {
            identifier,
            parameters,
        }
    }
}

pub type FunctionBodyClosure = Arc<dyn Fn(&[Box<Expression>], &Context) -> f64 + Send + Sync>;

#[derive(Clone)]
pub enum FunctionBody {
    Closure(FunctionBodyClosure),
    Expression(Expression),
}

impl Debug for FunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closure(_) => write!(f, "anonymous closure"),
            Self::Expression(expression) => write!(f, "{:?}", expression),
        }
    }
}

impl PartialEq for FunctionBody {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Closure(self_closure), Self::Closure(other_closure)) => {
                Arc::ptr_eq(self_closure, other_closure)
            }
            (Self::Expression(self_expression), Self::Expression(other_expression)) => {
                self_expression.eq(other_expression)
            }
            _ => false,
        }
    }
}

impl FunctionBody {
    pub fn eval(&self, arguments: &[Box<Expression>], context: &Context) -> f64 {
        match self {
            Self::Closure(closure) => closure(arguments, context),
            Self::Expression(expression) => expression.eval(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::{
        MusathParser,
        expression::{BinaryOperator, Primary},
    };

    use super::*;

    #[test]
    fn test_parse_function_signature() {
        assert_eq!(
            FunctionSignature::parse(
                &mut MusathParser::parse(Rule::function_signature, "test(t)")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            FunctionSignature {
                identifier: String::from("test"),
                parameters: vec![String::from("t")],
            },
        );
    }

    #[test]
    fn test_parse_function() {
        assert_eq!(
            Function::parse(
                &mut MusathParser::parse(Rule::function, "test(t) = t + 1")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Function {
                signature: FunctionSignature {
                    identifier: String::from("test"),
                    parameters: vec![String::from("t")],
                },
                body: FunctionBody::Expression(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Identifier(String::from("t")))),
                    BinaryOperator::Add,
                    Box::new(Expression::Primary(Primary::Number(1.0))),
                ),),
            },
        );
    }
}
