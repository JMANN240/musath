use std::{collections::HashMap, fmt::Display};

use pest::iterators::Pairs;

use crate::{Rule, expression::Expression};

#[derive(Debug)]
pub struct Function {
    identifier: String,
    parameters: Vec<String>,
    expression: Expression,
}

impl Function {
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn parameters(&self) -> &Vec<String> {
        &self.parameters
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn parse(pairs: Pairs<Rule>) -> Self {
        let mut identifier = None;
        let mut parameters = Vec::new();
        let mut expression = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::func_signature => {
                    let mut pairs = pair.into_inner();

                    identifier = Some(pairs.next().unwrap().as_str().to_string());

                    for pair in pairs {
                        match pair.as_rule() {
                            Rule::ident => parameters.push(pair.as_str().to_string()),
                            rule => unreachable!("expected identifier, found {:?}", rule),
                        }
                    }
                }
                Rule::expr => expression = Some(Expression::parse(pair.into_inner())),
                rule => unreachable!(
                    "expected function identifier or expression, found {:?}",
                    rule
                ),
            }
        }

        Self {
            identifier: identifier.expect("function missing identifier"),
            parameters,
            expression: expression.expect("function missing body"),
        }
    }

    pub fn eval(&self, functions: &HashMap<String, Function>, arguments: &Vec<f64>) -> f64 {
        self.expression.eval(
            functions,
            &self
                .parameters
                .iter()
                .cloned()
                .zip(arguments.iter().copied())
                .collect::<HashMap<String, f64>>(),
        )
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}) = {:?}", self.identifier(), self.parameters().join(", "), self.expression())
    }
}
