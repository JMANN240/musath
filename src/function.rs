use std::fmt::Display;

use pest::Parser;

use crate::{MusathParser, Rule, context::Context, expression::Expression};

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    signature: FunctionSignature,
    expression: Expression,
}

impl Function {
    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn parse(input: &str) -> Self {
        let function_declaration = MusathParser::parse(Rule::func_declaration, input)
            .unwrap()
            .next()
            .unwrap();

        let mut signature = None;
        let mut expression = None;

        for pair in function_declaration.into_inner() {
            match pair.as_rule() {
                Rule::func_signature => {
                    signature = Some(FunctionSignature::parse(pair.as_str()));
                }
                Rule::expr => {
                    expression = Some(Expression::parse(pair.as_str()));
                }
                rule => unreachable!(
                    "expected function identifier or expression, found {:?}",
                    rule
                ),
            }
        }

        Self {
            signature: signature.unwrap(),
            expression: expression.unwrap(),
        }
    }

    pub fn eval(&self, context: &Context) -> f64 {
        self.expression().eval(context)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}) = {:?}",
            self.signature().identifier(),
            self.signature().parameters().join(", "),
            self.expression()
        )
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

    pub fn parse(input: &str) -> Self {
        let function_signature = MusathParser::parse(Rule::func_signature, input)
            .unwrap()
            .next()
            .unwrap();

        let mut pairs = function_signature.into_inner();

        let identifier = pairs.next().unwrap().as_str().to_string();

        let mut parameters = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::ident => parameters.push(pair.as_str().to_string()),
                rule => unreachable!("expected identifier, found {:?}", rule),
            }
        }

        Self {
            identifier,
            parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_function_signature() {
        assert_eq!(
            FunctionSignature {
                identifier: String::from("test"),
                parameters: vec![String::from("t")],
            },
            FunctionSignature::parse("test(t)")
        );
    }

    #[test]
    fn test_parse_function() {
        assert_eq!(
            Function {
                signature: FunctionSignature {
                    identifier: String::from("test"),
                    parameters: vec![String::from("t")],
                },
                expression: Expression::Number(1.0)
            },
            Function::parse("test(t) = 1.0")
        );
    }
}
