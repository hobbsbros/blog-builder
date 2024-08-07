//! Newline parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenizer,
};

/// Parselet for newline characters.
pub struct NewlineParselet;

impl PrefixParselet for NewlineParselet {
    fn parse(&self, _: &Parser, tokenizer: &mut Tokenizer, _: &Token) -> Expression {
        while let Some(t) = tokenizer.peek() {
            if t.get_class() == TokenClass::Newline {
                tokenizer.next();
            } else if t.get_class() == TokenClass::Alphanumeric && t.get_value().len() == 0 {
                tokenizer.next();
            } else {
                break;
            }
        }

        Expression::Newline
    }
}