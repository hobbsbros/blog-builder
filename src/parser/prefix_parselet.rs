//! Prefix parselets for the Blog Builder parser.

use super::{
    Expression,
    Parser,
    Token,
    Tokenizer,
};

/// Prefix parselet abstraction.
pub trait PrefixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression;
}