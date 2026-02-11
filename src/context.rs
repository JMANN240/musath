use std::{
    collections::HashMap,
    f64::consts::{E, PI, TAU},
    sync::Arc,
};

use crate::{
    expression::{Expression, Primary},
    function::Function,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    values: HashMap<String, Vec<f64>>,
    functions: HashMap<String, Function>,
}

impl Context {
    pub fn values(&self) -> &HashMap<String, Vec<f64>> {
        &self.values
    }

    pub fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }

    pub fn function(&self, identifier: impl AsRef<str>) -> Option<&Function> {
        self.functions().get(identifier.as_ref())
    }

    pub fn set_function(&mut self, function: Function) {
        self.functions
            .insert(function.signature().identifier().to_string(), function);
    }

    pub fn value(&self, identifier: impl AsRef<str>) -> Option<&f64> {
        self.values().get(identifier.as_ref())?.last()
    }

    pub fn push_value(&mut self, identifier: impl Into<String>, value: f64) {
        self.values
            .entry(identifier.into())
            .and_modify(|values| values.push(value))
            .or_insert(vec![value]);
    }

    pub fn pop_value(&mut self, identifier: impl Into<String>) {
        self.values.entry(identifier.into()).and_modify(|values| {
            values.pop();
        });
    }
}

impl Default for Context {
    fn default() -> Self {
        let mut context = Self {
            values: HashMap::new(),
            functions: HashMap::new(),
        };

        context.push_value("pi", PI);
        context.push_value("tau", TAU);
        context.push_value("e", E);

        context.set_function(Function::new(
            "abs",
            Arc::new(|arguments, context| arguments[0].eval(context).abs()),
        ));

        context.set_function(Function::new(
            "min",
            Arc::new(|arguments, context| {
                arguments[0].eval(context).min(arguments[1].eval(context))
            }),
        ));

        context.set_function(Function::new(
            "max",
            Arc::new(|arguments, context| {
                arguments[0].eval(context).max(arguments[1].eval(context))
            }),
        ));

        context.set_function(Function::new(
            "sin",
            Arc::new(|arguments, context| arguments[0].eval(context).sin()),
        ));

        context.set_function(Function::new(
            "cos",
            Arc::new(|arguments, context| arguments[0].eval(context).cos()),
        ));

        context.set_function(Function::new(
            "floor",
            Arc::new(|arguments, context| arguments[0].eval(context).floor()),
        ));

        context.set_function(Function::new(
            "ceil",
            Arc::new(|arguments, context| arguments[0].eval(context).ceil()),
        ));

        context.set_function(Function::new(
            "mix",
            Arc::new(|arguments, context| {
                arguments
                    .iter()
                    .map(|argument| argument.eval(context))
                    .sum::<f64>()
                    / arguments.len() as f64
            }),
        ));

        context.set_function(Function::new(
            "sum",
            Arc::new(|arguments, context| {
                if let Expression::Primary(Primary::Identifier(identifier)) = arguments[0].as_ref()
                {
                    let start = arguments
                        .get(1)
                        .expect("expected start")
                        .eval(context)
                        .round() as isize;
                    let end = arguments
                        .get(2)
                        .expect("expected end")
                        .eval(context)
                        .round() as isize;

                    (start..end)
                        .map(|value| {
                            let mut context = context.clone();

                            context.push_value(identifier, value as f64);

                            arguments
                                .get(3)
                                .expect("expected sum expression")
                                .eval(&context)
                        })
                        .sum()
                } else {
                    panic!("expected identifier, found {:?}", arguments[0])
                }
            }),
        ));

        context.set_function(Function::new(
            "prod",
            Arc::new(|arguments, context| {
                if let Expression::Primary(Primary::Identifier(identifier)) = arguments[0].as_ref()
                {
                    let start = arguments
                        .get(1)
                        .expect("expected start")
                        .eval(context)
                        .round() as isize;
                    let end = arguments
                        .get(2)
                        .expect("expected end")
                        .eval(context)
                        .round() as isize;

                    (start..end)
                        .map(|value| {
                            let mut context = context.clone();

                            context.push_value(identifier, value as f64);

                            arguments
                                .get(3)
                                .expect("expected product expression")
                                .eval(&context)
                        })
                        .product()
                } else {
                    panic!("expected identifier, found {:?}", arguments[0])
                }
            }),
        ));

        context
    }
}
