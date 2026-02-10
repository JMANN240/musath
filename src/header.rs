use std::collections::HashMap;

use pest::Parser;

use crate::{MusathParser, Rule};

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    key_values: HashMap<String, HeaderValue>,
}

impl Header {
    pub fn parse(input: &str) -> Self {
        let header = MusathParser::parse(Rule::header, input)
            .unwrap()
            .next()
            .unwrap();

        let mut key_values = HashMap::new();

        for pair in header.into_inner() {
            match pair.as_rule() {
                Rule::header_declaration => {
                    let header_declaration = HeaderDeclaration::parse(pair.as_str());

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
    pub fn parse(input: &str) -> Self {
        let header_declaration = MusathParser::parse(Rule::header_declaration, input)
            .unwrap()
            .next()
            .unwrap();

        let mut key = None;
        let mut value = None;

        for pair in header_declaration.into_inner() {
            match pair.as_rule() {
                Rule::header_key => {
                    key = Some(pair.as_str().to_string());
                }
                Rule::header_value => {
                    value = Some(HeaderValue::parse(pair.as_str()));
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
    pub fn parse(input: &str) -> Self {
        let header_value = MusathParser::parse(Rule::header_value, input)
            .unwrap()
            .next()
            .unwrap();

        let pair = header_value.into_inner().next().unwrap();

        match pair.as_rule() {
            Rule::string => Self::String(pair.as_str().to_string()),
            Rule::number => Self::Number(pair.as_str().parse::<f64>().unwrap()),
            rule => unreachable!("expected string or number, found {:?}", rule),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_value() {
        assert_eq!(
            HeaderValue::String(String::from("test")),
            HeaderValue::parse("\"test\"")
        );

        assert_eq!(HeaderValue::Number(1.0), HeaderValue::parse("1.0"));
    }

    #[test]
    fn test_parse_header_declaration() {
        assert_eq!(
            HeaderDeclaration {
                key: String::from("TEST"),
                value: HeaderValue::String(String::from("test")),
            },
            HeaderDeclaration::parse("TEST = \"test\"")
        );

        assert_eq!(
            HeaderDeclaration {
                key: String::from("TESTA"),
                value: HeaderValue::Number(2.0),
            },
            HeaderDeclaration::parse("TESTA = 2.0")
        );
    }

    #[test]
    fn test_parse_header() {
        let mut key_values = HashMap::<String, HeaderValue>::new();

        key_values.insert(
            String::from("TEST"),
            HeaderValue::String(String::from("test")),
        );
        key_values.insert(String::from("TESTA"), HeaderValue::Number(2.0));

        let header = Header { key_values };

        assert_eq!(
            header,
            Header::parse(
                r#"TEST = "test"
TESTA = 2.0"#
            )
        );
    }
}
