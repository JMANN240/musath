use std::{collections::HashMap, f64::consts::{E, PI, TAU}};

use crate::function::Function;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    values: HashMap<String, Vec<f64>>,
    functions: HashMap<String, Function>,
}

impl Context {
    pub fn new() -> Self {
        let mut context = Self {
            values: HashMap::new(),
            functions: HashMap::new(),
        };

        context.push_value("pi", PI);
        context.push_value("tau", TAU);
        context.push_value("e", E);

        context
    }

    pub fn values(&self) -> &HashMap<String, Vec<f64>> {
        &self.values
    }

    pub fn functions(&self) -> &HashMap<String, Function> {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut HashMap<String, Function> {
        &mut self.functions
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
