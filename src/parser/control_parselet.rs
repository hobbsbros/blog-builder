//! Control parselet.

use crate::Error;

use super::{
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenizer,
};

pub struct ControlParselet;

impl PrefixParselet for ControlParselet {
    fn parse(&self, parser: &Parser, tokenizer: &mut Tokenizer, _: &Token) -> Expression {
        let command = tokenizer.discard(TokenClass::Alphanumeric);

        match command.as_str() {
            "href" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let name = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let href = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Hyperlink {
                    name,
                    href,
                }
            },
            "code" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let language = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let code = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Code {
                    language,
                    code,
                }
            },
            "tile" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let name = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let img = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let href = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Tile {
                    name,
                    img,
                    href,
                }
            },
            "img" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let src = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let alt = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let scale = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Image {
                    img: src,
                    alt,
                    scale,
                }
            },
            "floating" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let src = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let alt = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::FloatingImage {
                    img: src,
                    alt,
                }
            },
            "bold" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Bold (inside)
            },
            "it" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Italic (inside)
            },
            "block" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let quote = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                tokenizer.discard(TokenClass::OpenCurly);
                let citation = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::BlockQuote {
                    quote,
                    citation,
                }
            },
            "header" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Header (inside)
            },
            "footer" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Footer (inside)
            },
            "footnote" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Footnote (inside)
            },
            "footnotes" => Expression::Footnotes,
            "topblock" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Topblock (inside)
            },
            "subtitle" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Subtitle (inside)
            },
            "subsubtitle" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Subsubtitle (inside)
            },
            "menu" => Expression::Menu,
            "date" => Expression::Date,
            "tiles" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let inside = parser.parse_tokens(tokenizer, TokenClass::OpenCurly.get_precedence());
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Tiles (inside)
            },
            "pagename" => {
                tokenizer.discard(TokenClass::OpenCurly);
                let name = tokenizer.discard(TokenClass::Alphanumeric);
                tokenizer.discard(TokenClass::CloseCurly);

                Expression::Pagename (name)
            },
            _ => Error::UnrecognizedControlSequence.throw(),
        }
    }
}