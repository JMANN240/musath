use std::collections::HashMap;

use pest::iterators::Pairs;

use crate::Rule;

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    key_values: HashMap<String, HeaderValue>,
}

impl Header {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let mut key_values = HashMap::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::header_declaration => {
                    let header_declaration = HeaderDeclaration::parse(&mut pair.into_inner());

                    key_values.insert(header_declaration.key, header_declaration.value);
                }
                rule => unreachable!("expected header_declaration, found {:?}", rule),
            };
        }

        Self { key_values }
    }

    pub fn key_values(&self) -> &HashMap<String, HeaderValue> {
        &self.key_values
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HeaderDeclaration {
    key: String,
    value: HeaderValue,
}

impl HeaderDeclaration {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let mut key = None;
        let mut value = None;

        for pair in pairs {
            match pair.as_rule() {
                Rule::header_key => {
                    key = Some(pair.as_str().to_string());
                }
                Rule::header_value => {
                    value = Some(HeaderValue::parse(&mut pair.into_inner()));
                }
                rule => unreachable!("expected header_key or header_value, found {:?}", rule),
            }
        }

        Self {
            key: key.unwrap(),
            value: value.unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum HeaderValue {
    String(String),
    Number(f64),
}

impl HeaderValue {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let pair = pairs.next().unwrap();

        match pair.as_rule() {
            Rule::string => Self::String(pair.as_str().to_string()),
            Rule::number => Self::Number(pair.as_str().parse::<f64>().unwrap()),
            rule => unreachable!("expected string or number, found {:?}", rule),
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::MusathParser;

    use super::*;

    #[test]
    fn test_parse_header_value() {
        assert_eq!(
            HeaderValue::parse(
                &mut MusathParser::parse(Rule::header_value, "\"test\"")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            HeaderValue::String(String::from("test")),
        );

        assert_eq!(
            HeaderValue::parse(
                &mut MusathParser::parse(Rule::header_value, "1.0")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            HeaderValue::Number(1.0),
        );
    }

    #[test]
    fn test_parse_header_declaration() {
        assert_eq!(
            HeaderDeclaration::parse(
                &mut MusathParser::parse(Rule::header_declaration, "TEST = 1.0")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            HeaderDeclaration {
                key: String::from("TEST"),
                value: HeaderValue::Number(1.0),
            },
        );
    }
}
