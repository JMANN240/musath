use pest::iterators::Pairs;

use crate::{Rule, context::Context, function::Function};

#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    context: Context,
}

impl Body {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let mut context = Context::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::function => context.set_function(Function::parse(&mut pair.into_inner())),
                _ => unreachable!("expected function, found {:?}", pair),
            };
        }

        Self { context }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}
