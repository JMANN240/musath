use pest::iterators::Pairs;

use crate::{Rule, context::Context};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Primary(Primary),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

impl Expression {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Self {
        let remainder_pair = pairs.next().unwrap();
        assert!(
            matches!(remainder_pair.as_rule(), Rule::remainder),
            "expected remainder, found {:?}",
            remainder_pair
        );
        Remainder::parse(&mut remainder_pair.into_inner())
    }

    pub fn eval(&self, context: &Context) -> f64 {
        match self {
            Self::Binary(left, operator, right) => {
                operator.eval(left.eval(context), right.eval(context))
            }
            Self::Unary(operator, operand) => operator.eval(operand.eval(context)),
            Self::Primary(primary) => primary.eval(context),
        }
    }
}

pub struct Remainder;

impl Remainder {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Expression {
        let first_term_pair = pairs.next().unwrap();
        assert!(
            matches!(first_term_pair.as_rule(), Rule::term),
            "expected term, found {:?}",
            first_term_pair
        );
        let first_term = Term::parse(&mut first_term_pair.into_inner());

        let mut remainder = first_term;

        while let Some(operator_pair) = pairs.next() {
            let next_term_pair = pairs.next().unwrap();
            assert!(
                matches!(next_term_pair.as_rule(), Rule::term),
                "expected term, found {:?}",
                next_term_pair
            );
            let next_term = Term::parse(&mut next_term_pair.into_inner());

            let operator = match operator_pair.as_rule() {
                Rule::rem => BinaryOperator::Remainder,
                _ => unreachable!("expected %, found {}", operator_pair),
            };

            remainder = Expression::Binary(Box::new(remainder), operator, Box::new(next_term))
        }

        remainder
    }
}

pub struct Term;

impl Term {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Expression {
        let first_factor_pair = pairs.next().unwrap();
        assert!(
            matches!(first_factor_pair.as_rule(), Rule::factor),
            "expected factor, found {:?}",
            first_factor_pair
        );
        let first_factor = Factor::parse(&mut first_factor_pair.into_inner());

        let mut term = first_factor;

        while let Some(operator_pair) = pairs.next() {
            let next_factor_pair = pairs.next().unwrap();
            assert!(
                matches!(next_factor_pair.as_rule(), Rule::factor),
                "expected factor, found {:?}",
                next_factor_pair
            );
            let next_factor = Factor::parse(&mut next_factor_pair.into_inner());

            let operator = match operator_pair.as_rule() {
                Rule::add => BinaryOperator::Add,
                Rule::sub => BinaryOperator::Subtract,
                _ => unreachable!("expected + or -, found {}", operator_pair),
            };

            term = Expression::Binary(Box::new(term), operator, Box::new(next_factor))
        }

        term
    }
}

pub struct Factor;

impl Factor {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Expression {
        let first_power_pair = pairs.next().unwrap();
        assert!(
            matches!(first_power_pair.as_rule(), Rule::power),
            "expected power, found {:?}",
            first_power_pair
        );
        let first_power = Power::parse(&mut first_power_pair.into_inner());

        let mut factor = first_power;

        while let Some(operator_pair) = pairs.next() {
            let next_power_pair = pairs.next().unwrap();
            assert!(
                matches!(next_power_pair.as_rule(), Rule::power),
                "expected factor, found {:?}",
                next_power_pair
            );
            let next_power = Power::parse(&mut next_power_pair.into_inner());

            let operator = match operator_pair.as_rule() {
                Rule::mul => BinaryOperator::Multiply,
                Rule::div => BinaryOperator::Divide,
                _ => unreachable!("expected * or /, found {}", operator_pair),
            };

            factor = Expression::Binary(Box::new(factor), operator, Box::new(next_power))
        }

        factor
    }
}

pub struct Power;

impl Power {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Expression {
        let first_unary_pair = pairs.next().unwrap();
        assert!(
            matches!(first_unary_pair.as_rule(), Rule::unary),
            "expected unary, found {:?}",
            first_unary_pair
        );
        let first_unary = Unary::parse(&mut first_unary_pair.into_inner());

        let mut power = first_unary;

        if let Some(operator_pair) = pairs.next() {
            let next_power = Power::parse(pairs);

            let operator = match operator_pair.as_rule() {
                Rule::pow => BinaryOperator::Exponentiate,
                _ => unreachable!("expected ^, found {}", operator_pair),
            };

            power = Expression::Binary(Box::new(power), operator, Box::new(next_power))
        }

        power
    }
}

pub struct Unary;

impl Unary {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Expression {
        let first_pair = pairs.next().unwrap();

        match first_pair.as_rule() {
            Rule::neg => {
                let unary_pair = pairs.next().unwrap();

                Expression::Unary(
                    UnaryOperator::Negate,
                    Box::new(Unary::parse(&mut unary_pair.into_inner())),
                )
            }
            Rule::primary => Primary::parse(&mut first_pair.into_inner()),
            _ => unreachable!("expected negation or primary, found {:?}", first_pair),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
}

impl UnaryOperator {
    pub fn eval(&self, operand: f64) -> f64 {
        match self {
            Self::Negate => -operand,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponentiate,
    Remainder,
}

impl BinaryOperator {
    pub fn eval(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Subtract => left - right,
            Self::Multiply => left * right,
            Self::Divide => left / right,
            Self::Exponentiate => left.powf(right),
            Self::Remainder => left.rem_euclid(right),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
    Decimal(f64),
    Integer(i64),
    Call(String, Vec<Box<Expression>>),
    Identifier(String),
    Grouping(Box<Expression>),
}

impl Primary {
    pub fn parse(pairs: &mut Pairs<Rule>) -> Expression {
        let pair = pairs.next().unwrap();

        Expression::Primary(match pair.as_rule() {
            Rule::number => {
                let mut pairs = pair.into_inner();

                let specific_pair = pairs.next().unwrap();

                match specific_pair.as_rule() {
                    Rule::decimal => Self::Decimal(specific_pair.as_str().parse::<f64>().unwrap()),
                    Rule::integer => Self::Integer(specific_pair.as_str().parse::<i64>().unwrap()),
                    _ => unreachable!("expected decimal or integer, found {:?}", specific_pair),
                }
            },
            Rule::function_call => {
                let mut pairs = pair.into_inner();

                let identifier_pair = pairs.next().unwrap();
                let identifier = identifier_pair.as_str().to_string();

                let arguments = pairs
                    .map(|expression_pair| {
                        Box::new(Expression::parse(&mut expression_pair.into_inner()))
                    })
                    .collect();

                Self::Call(identifier, arguments)
            }
            Rule::identifier => Self::Identifier(pair.as_str().to_string()),
            Rule::expression => Self::Grouping(Box::new(Expression::parse(&mut pair.into_inner()))),
            _ => unreachable!(
                "expected number, identifier, or grouped expression, found {}",
                pair
            ),
        })
    }

    pub fn eval(&self, context: &Context) -> f64 {
        match self {
            Self::Decimal(number) => *number,
            Self::Integer(number) => *number as f64,
            Self::Call(identifier, arguments) => {
                let function = context
                    .function(identifier)
                    .unwrap_or_else(|| panic!("undefined function {}", identifier));

                let mut inner_context = context.clone();

                for (parameter_identifier, argument_expression) in function
                    .signature()
                    .parameters()
                    .iter()
                    .zip(arguments.iter())
                {
                    inner_context
                        .push_value(parameter_identifier, argument_expression.eval(context));
                }

                function.eval(arguments, &inner_context)
            }
            Self::Identifier(identifier) => *context
                .value(identifier)
                .unwrap_or_else(|| panic!("undefined identifier {}", identifier)),
            Self::Grouping(expression) => expression.eval(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use crate::MusathParser;

    use super::*;

    #[test]
    fn test_parse_primary() {
        assert_eq!(
            Primary::parse(
                &mut MusathParser::parse(Rule::primary, "1")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Primary(Primary::Decimal(1.0)),
        );

        assert_eq!(
            Primary::parse(
                &mut MusathParser::parse(Rule::primary, "2.3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Primary(Primary::Decimal(2.3)),
        );

        assert_eq!(
            Primary::parse(
                &mut MusathParser::parse(Rule::primary, "test")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Primary(Primary::Identifier(String::from("test"))),
        );

        assert_eq!(
            Primary::parse(
                &mut MusathParser::parse(Rule::primary, "(1)")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Primary(Primary::Grouping(Box::new(Expression::Primary(
                Primary::Decimal(1.0)
            )))),
        );

        assert_eq!(
            Primary::parse(
                &mut MusathParser::parse(Rule::primary, "test(1)")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Primary(Primary::Call(
                String::from("test"),
                vec![Box::new(Expression::Primary(Primary::Decimal(1.0)))]
            )),
        );

        assert_eq!(
            Primary::parse(
                &mut MusathParser::parse(Rule::primary, "test( 1 , 2 + 3 )")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Primary(Primary::Call(
                String::from("test"),
                vec![
                    Box::new(Expression::Primary(Primary::Decimal(1.0))),
                    Box::new(Expression::Binary(
                        Box::new(Expression::Primary(Primary::Decimal(2.0))),
                        BinaryOperator::Add,
                        Box::new(Expression::Primary(Primary::Decimal(3.0))),
                    )),
                ]
            )),
        );
    }

    #[test]
    fn test_parse_unary() {
        assert_eq!(
            Unary::parse(
                &mut MusathParser::parse(Rule::unary, "1")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Primary(Primary::Decimal(1.0)),
        );

        assert_eq!(
            Unary::parse(
                &mut MusathParser::parse(Rule::unary, "-1")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Unary(
                UnaryOperator::Negate,
                Box::new(Expression::Primary(Primary::Decimal(1.0)))
            ),
        );

        assert_eq!(
            Unary::parse(
                &mut MusathParser::parse(Rule::unary, "--1")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Unary(
                UnaryOperator::Negate,
                Box::new(Expression::Unary(
                    UnaryOperator::Negate,
                    Box::new(Expression::Primary(Primary::Decimal(1.0)))
                ))
            ),
        );
    }

    #[test]
    fn test_parse_power() {
        assert_eq!(
            Power::parse(
                &mut MusathParser::parse(Rule::power, "1^2")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(1.0))),
                BinaryOperator::Exponentiate,
                Box::new(Expression::Primary(Primary::Decimal(2.0))),
            ),
        );

        assert_eq!(
            Power::parse(
                &mut MusathParser::parse(Rule::power, "1^2^3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(1.0))),
                BinaryOperator::Exponentiate,
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(2.0))),
                    BinaryOperator::Exponentiate,
                    Box::new(Expression::Primary(Primary::Decimal(3.0))),
                )),
            ),
        );

        assert_eq!(
            Power::parse(
                &mut MusathParser::parse(Rule::power, "(1^2)^3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Grouping(Box::new(
                    Expression::Binary(
                        Box::new(Expression::Primary(Primary::Decimal(1.0))),
                        BinaryOperator::Exponentiate,
                        Box::new(Expression::Primary(Primary::Decimal(2.0))),
                    )
                )))),
                BinaryOperator::Exponentiate,
                Box::new(Expression::Primary(Primary::Decimal(3.0))),
            ),
        );
    }

    #[test]
    fn test_parse_term() {
        assert_eq!(
            Term::parse(
                &mut MusathParser::parse(Rule::term, "1 + 2")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(1.0))),
                BinaryOperator::Add,
                Box::new(Expression::Primary(Primary::Decimal(2.0))),
            ),
        );

        assert_eq!(
            Term::parse(
                &mut MusathParser::parse(Rule::term, "1 + 2 - 3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(1.0))),
                    BinaryOperator::Add,
                    Box::new(Expression::Primary(Primary::Decimal(2.0))),
                )),
                BinaryOperator::Subtract,
                Box::new(Expression::Primary(Primary::Decimal(3.0))),
            ),
        );

        assert_eq!(
            Term::parse(
                &mut MusathParser::parse(Rule::term, "1 + -2 - 3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(1.0))),
                    BinaryOperator::Add,
                    Box::new(Expression::Unary(
                        UnaryOperator::Negate,
                        Box::new(Expression::Primary(Primary::Decimal(2.0)))
                    )),
                )),
                BinaryOperator::Subtract,
                Box::new(Expression::Primary(Primary::Decimal(3.0))),
            ),
        );
    }

    #[test]
    fn test_parse_remainder() {
        let remainder = Remainder::parse(
            &mut MusathParser::parse(Rule::remainder, "1 % 2")
                .unwrap()
                .next()
                .unwrap()
                .into_inner(),
        );

        assert_eq!(
            remainder,
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(1.0))),
                BinaryOperator::Remainder,
                Box::new(Expression::Primary(Primary::Decimal(2.0))),
            ),
        );

        assert_eq!(remainder.eval(&Context::default()), 1.0);

        let remainder = Remainder::parse(
            &mut MusathParser::parse(Rule::remainder, "3.5 % 2")
                .unwrap()
                .next()
                .unwrap()
                .into_inner(),
        );

        assert_eq!(
            remainder,
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(3.5))),
                BinaryOperator::Remainder,
                Box::new(Expression::Primary(Primary::Decimal(2.0))),
            ),
        );

        assert_eq!(remainder.eval(&Context::default()), 1.5);
    }

    #[test]
    fn test_parse_expression() {
        assert_eq!(
            Expression::parse(
                &mut MusathParser::parse(Rule::expression, "1 + 2")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(1.0))),
                BinaryOperator::Add,
                Box::new(Expression::Primary(Primary::Decimal(2.0))),
            ),
        );

        assert_eq!(
            Expression::parse(
                &mut MusathParser::parse(Rule::expression, "1 + 2 - 3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(1.0))),
                    BinaryOperator::Add,
                    Box::new(Expression::Primary(Primary::Decimal(2.0))),
                )),
                BinaryOperator::Subtract,
                Box::new(Expression::Primary(Primary::Decimal(3.0))),
            ),
        );

        assert_eq!(
            Expression::parse(
                &mut MusathParser::parse(Rule::expression, "1 * 2 + 3 * 4")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(1.0))),
                    BinaryOperator::Multiply,
                    Box::new(Expression::Primary(Primary::Decimal(2.0))),
                )),
                BinaryOperator::Add,
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(3.0))),
                    BinaryOperator::Multiply,
                    Box::new(Expression::Primary(Primary::Decimal(4.0))),
                )),
            ),
        );

        assert_eq!(
            Expression::parse(
                &mut MusathParser::parse(Rule::expression, "-1 * 2 + 3 / 4")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Binary(
                    Box::new(Expression::Unary(
                        UnaryOperator::Negate,
                        Box::new(Expression::Primary(Primary::Decimal(1.0)))
                    )),
                    BinaryOperator::Multiply,
                    Box::new(Expression::Primary(Primary::Decimal(2.0))),
                )),
                BinaryOperator::Add,
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(3.0))),
                    BinaryOperator::Divide,
                    Box::new(Expression::Primary(Primary::Decimal(4.0))),
                )),
            ),
        );

        assert_eq!(
            Expression::parse(
                &mut MusathParser::parse(Rule::expression, "2 ^ 3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(2.0))),
                BinaryOperator::Exponentiate,
                Box::new(Expression::Primary(Primary::Decimal(3.0))),
            ),
        );

        assert_eq!(
            Expression::parse(
                &mut MusathParser::parse(Rule::expression, "1 + 2 ^ 3")
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
            ),
            Expression::Binary(
                Box::new(Expression::Primary(Primary::Decimal(1.0))),
                BinaryOperator::Add,
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(2.0))),
                    BinaryOperator::Exponentiate,
                    Box::new(Expression::Primary(Primary::Decimal(3.0))),
                )),
            ),
        );

        let expression = Expression::parse(
            &mut MusathParser::parse(Rule::expression, "-1 * 2 + 3 / 4 ^ 5")
                .unwrap()
                .next()
                .unwrap()
                .into_inner(),
        );

        assert_eq!(
            expression,
            Expression::Binary(
                Box::new(Expression::Binary(
                    Box::new(Expression::Unary(
                        UnaryOperator::Negate,
                        Box::new(Expression::Primary(Primary::Decimal(1.0)))
                    )),
                    BinaryOperator::Multiply,
                    Box::new(Expression::Primary(Primary::Decimal(2.0))),
                )),
                BinaryOperator::Add,
                Box::new(Expression::Binary(
                    Box::new(Expression::Primary(Primary::Decimal(3.0))),
                    BinaryOperator::Divide,
                    Box::new(Expression::Binary(
                        Box::new(Expression::Primary(Primary::Decimal(4.0))),
                        BinaryOperator::Exponentiate,
                        Box::new(Expression::Primary(Primary::Decimal(5.0))),
                    )),
                )),
            ),
        );

        assert_eq!(expression.eval(&Context::default()), -2.0 + (3.0 / 1024.0));
    }
}
