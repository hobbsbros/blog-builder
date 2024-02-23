//! Paragraph parselet.

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    Tokenizer,
};

pub struct ParagraphParselet;

impl PrefixParselet for ParagraphParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, token: &Token) -> Expression {
        let inside = parser.parse_tokens(tokenizer, token.get_precedence());

        Expression::Paragraph (inside)
    }
}