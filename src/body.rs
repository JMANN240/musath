use std::collections::HashMap;

use pest::Parser;

use crate::{MusathParser, Rule, context::Context, function::Function};

#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    context: Context,
}

impl Body {
    pub fn parse(input: &str) -> Self {
        let body = MusathParser::parse(Rule::body, input)
            .unwrap()
            .next()
            .unwrap();

        let mut context = Context::new();

        for pair in body.into_inner() {
            match pair.as_rule() {
                Rule::func_declaration => {
                    let function = Function::parse(pair.as_str());

                    context.functions_mut().insert(function.signature().identifier().to_string(), function);
                }
                rule => unreachable!("expected func_declaration, found {:?}", rule),
            };
        }

        Self { context }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}
