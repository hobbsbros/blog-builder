//! Prefix parselets for the Blog Builder parser.

use super::{
    Expression,
    Parser,
    Token,
    Tokenizer,
};

pub trait PrefixParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression;
}