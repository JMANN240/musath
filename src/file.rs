use std::fmt::Display;

use pest::Parser;

use crate::{MusathParser, Rule, body::Body, header::Header};

#[derive(Debug)]
pub struct Musath {
    header: Header,
    body: Body,
}

impl Musath {
    pub fn parse(input: &str) -> Self {
        let pairs = MusathParser::parse(Rule::file, input).unwrap();

        let mut header = None;
        let mut body = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::header => {
                    header = Some(Header::parse(pair.as_str()));
                }
                Rule::body => {
                    body = Some(Body::parse(pair.as_str()));
                }
                Rule::EOI => (),
                rule => unreachable!("expected function or EOI, found {:?}", rule),
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

        output.eval(&context)
    }
}

impl Display for Musath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for function in self.body().context().functions().values() {
            writeln!(f, "{}", function)?;
        }

        Ok(())
    }
}
