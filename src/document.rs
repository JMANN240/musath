use pest::iterators::Pairs;

use crate::{
    Rule,
    body::Body,
    expression::{Expression, Primary},
    header::Header, wave_provider::WaveProvider,
};

#[derive(Debug)]
pub struct Document {
    header: Header,
    body: Body,
}

impl Document {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let mut header = None;
        let mut body = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::header => {
                    header = Some(Header::parse(&mut pair.into_inner()));
                }
                Rule::body => {
                    body = Some(Body::parse(&mut pair.into_inner()));
                }
                Rule::EOI => (),
                _ => unreachable!("expected header, body, EOI, found {:?}", pair),
            };
        }

        Self {
            header: header.unwrap(),
            body: body.unwrap(),
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn eval(&self, t: f64) -> f64 {
        let output = self
            .body()
            .context()
            .functions()
            .get("output")
            .expect("missing output function");

        let mut context = self.body().context().clone();

        context.push_value("t", t);

        output.eval(
            &[Box::new(Expression::Primary(Primary::Identifier(
                String::from("t"),
            )))],
            &context,
        )
    }
}

impl WaveProvider for Document {
    fn value_at_time(&self, t: f64) -> f64 {
        self.eval(t)
    }
}
