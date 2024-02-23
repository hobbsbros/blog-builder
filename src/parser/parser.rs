//! Parser for the Blog Builder.

use std::collections::HashMap;

use crate::Error;

use super::{
    Expression,
    PrefixParselet,
    TokenClass,
    Tokenizer,
};

use super::{
    AlphanumericParselet,
    ControlParselet,
    HashParselet,
    NewlineParselet,
    ParagraphParselet,
};

pub struct Parser {
    prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>>,
}

impl Parser {
    pub fn new() -> Self {
        let mut prefix_parselets: HashMap<TokenClass, Box<dyn PrefixParselet>> = HashMap::new();

        prefix_parselets.insert(TokenClass::Alphanumeric, Box::new(AlphanumericParselet {}));
        prefix_parselets.insert(TokenClass::BeginParagraph, Box::new(ParagraphParselet {}));
        prefix_parselets.insert(TokenClass::Control, Box::new(ControlParselet {}));
        prefix_parselets.insert(TokenClass::Hash, Box::new(HashParselet {}));
        prefix_parselets.insert(TokenClass::Newline, Box::new(NewlineParselet {}));

        Self {
            prefix_parselets,
        }
    }

    pub fn parse_tokens(&self, tokenizer: &mut Tokenizer, precedence: usize) -> Vec<Expression> {
        let mut expressions = Vec::new();

        while let Some(token) = tokenizer.peek() {
            if token.get_precedence() > precedence {
                tokenizer.next();

                let prefix_parselet = match self.prefix_parselets.get(&token.get_class()) {
                    Some (p) => p,
                    None => Error::UnrecognizedToken.throw_msg(
                        &format!(
                            "`{}` (of type {})",
                            &token.get_value(),
                            token.get_class(),
                        ),
                    ),
                };

                let expression = prefix_parselet.parse(self, tokenizer, &token);

                expressions.push(expression);
            } else {
                break;
            }
        }

        expressions
    }

    pub fn parse(&self, input: &str) -> Vec<Expression> {
        let mut tokenizer = Tokenizer::new(input.to_string());
        self.parse_tokens(&mut tokenizer, 0)
    }
}

#[test]
fn simple_parse() {
    let example = "# Hello, \\href{world}{www.google.com}!\nHave you ## ever tried using \\href{Google \\bold{Search}}{www.google.com}?";
    let parser = Parser::new();
    println!("{}", example);
    let expressions = parser.parse(example);
    println!("Parsed results");
    dbg!(expressions);
}