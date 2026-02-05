use std::{collections::HashMap, fmt::Display};

use pest::iterators::Pairs;

use crate::{Rule, function::Function};

#[derive(Debug)]
pub struct Musath {
    functions: HashMap<String, Function>,
}

impl Musath {
    pub fn parse(pairs: Pairs<Rule>) -> Self {
        let mut functions = HashMap::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::func_declaration => {
                    let function = Function::parse(pair.into_inner());

                    functions.insert(function.identifier().to_string(), function);
                }
                Rule::EOI => (),
                rule => unreachable!("expected function or EOI, found {:?}", rule),
            };
        }

        Self { functions }
    }

    pub fn eval(&self, t: f64) -> f64 {
        let output = self
            .functions
            .get("output")
            .expect("missing output function");

        output.eval(&self.functions, &vec![t])
    }
}

impl Display for Musath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in self.functions.values() {
            writeln!(f, "{}", function)?;
        }

        Ok(())
    }
}