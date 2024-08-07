//! Hash parselet.

use crate::Error;

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenizer,
};

/// Parselet for hash-prefixed expressions.
pub struct HashParselet;

impl PrefixParselet for HashParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression {
        let inside = parser.parse_tokens(tokenizer, token.get_precedence());

        match token.get_value().len() {
            0 => unreachable!(),
            1 => Expression::Title (inside),
            2 => Expression::Heading (inside),
            3 => Expression::Subheading (inside),
            _ => Error::TooManyHashes.throw_msg(&token.get_value()),
        }
    }
}