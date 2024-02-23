//! Tokenizer for the Blog Builder parser module.

use crate::Error;

use std::fmt::{
    Display,
    Formatter,
    Result,
};

pub struct Charstream {
    stream: Vec<char>,
    index: usize,
}

impl Charstream {
    /// Constructs a new character stream.
    pub fn new(input: String) -> Self {
        let stream = input.as_str()
            .chars()
            .collect::<Vec<char>>();

        Self {
            stream,
            index: 0,
        }
    }

    /// Gets the next character from this character stream.
    pub fn next(&mut self) -> Option<char> {
        let peek = self.peek();
        self.index += 1;
        peek
    }

    /// Peeks at the next character from this character stream.
    pub fn peek(&self) -> Option<char> {
        if self.index < self.stream.len() {
            Some (self.stream[self.index])
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    class: TokenClass,
    value: String,
}

impl Token {
    pub fn new(class: TokenClass, value: &str) -> Self {
        Self {
            class,
            value: value.to_string(),
        }
    }

    pub fn get_class(&self) -> TokenClass {
        self.class
    }

    pub fn get_value(&self) -> String {
        self.value.to_owned()
    }

    pub fn get_precedence(&self) -> usize {
        self.class.get_precedence()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TokenClass {
    Alphanumeric,
    BeginParagraph,
    Control,
    OpenCurly,
    CloseCurly,
    Hash,
    Newline,
}

impl TokenClass {
    pub fn get_class(c: char) -> Self {
        match c {
            '\\' => Self::Control,
            '~' => Self::BeginParagraph,
            '{' => Self::OpenCurly,
            '}' => Self::CloseCurly,
            '#' => Self::Hash,
            '\n' => Self::Newline,
            _ => Self::Alphanumeric,
        }
    }

    pub fn get_precedence(&self) -> usize {
        match self {
            Self::Control => 5,
            Self::OpenCurly => 1,
            Self::CloseCurly => 1,
            Self::Hash => 4,
            Self::Newline => 3,
            Self::Alphanumeric => 6,
            Self::BeginParagraph => 2,
        }
    }
}

impl Display for TokenClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use TokenClass::*;

        let msg = match self {
            Control => "Control",
            OpenCurly => "OpenCurly",
            CloseCurly => "CloseCurly",
            Hash => "Hash",
            Newline => "Newline",
            Alphanumeric => "Alphanumeric",
            BeginParagraph => "BeginParagraph",
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Token>,
    index: usize,
}

impl Tokenizer {
    /// Constructs a new tokenizer.
    pub fn new(input: String) -> Self {
        let mut charstream = Charstream::new(input);
        let mut tokens = Vec::new();
    
        while let Some(t) = Self::generate_next(&mut charstream) {
            tokens.push(t)
        }    

        Self {
            tokens,
            index: 0,
        }
    }

    /// Gets the next token from this token stream.
    pub fn next(&mut self) -> Option<Token> {
        let peek = self.peek();
        self.index += 1;
        peek
    }

    /// Peeks at the next token from this token stream.
    pub fn peek(&self) -> Option<Token> {
        if self.index < self.tokens.len() {
            Some (self.tokens[self.index].to_owned())
        } else {
            None
        }
    }

    /// Discards a token of a given type, or throw an error.
    pub fn discard(&mut self, class: TokenClass) -> String {
        if let Some(t) = self.next() {
            if t.get_class() == class {
                t.get_value()
            } else {
                Error::ExpectedTokenOfClass.throw_msg(
                    &format!(
                        "{}, found: {} (`{}`)",
                        class,
                        t.get_class(),
                        t.get_value(),
                    ),
                )
            }
        } else {
            Error::UnexpectedEof.throw()
        }
    }

    /// Gets a token from the character stream.
    fn generate_next(charstream: &mut Charstream) -> Option<Token> {      
        if let Some(c) = charstream.next() {
            match TokenClass::get_class(c) {
                TokenClass::Alphanumeric => {
                    let mut current = format!("{}", c);

                    while let Some(c) = charstream.peek() {
                        if TokenClass::get_class(c) == TokenClass::Alphanumeric {
                            charstream.next();
                            current.push(c);
                        } else {
                            break;
                        }
                    }

                    Some (Token::new(
                        TokenClass::Alphanumeric,
                        &current.trim(),
                    ))
                },
                TokenClass::BeginParagraph => Some (Token::new(
                    TokenClass::BeginParagraph,
                    "~"
                )),
                TokenClass::Control => Some (Token::new(
                    TokenClass::Control,
                    "\\"
                )),
                TokenClass::OpenCurly => Some (Token::new(
                    TokenClass::OpenCurly,
                    "{"
                )),
                TokenClass::CloseCurly => Some (Token::new(
                    TokenClass::CloseCurly,
                    "}"
                )),
                TokenClass::Hash => {
                    let mut current = format!("{}", c);

                    while let Some(c) = charstream.peek() {
                        if TokenClass::get_class(c) == TokenClass::Hash {
                            charstream.next();
                            current.push(c);
                        } else {
                            break;
                        }
                    }

                    Some (Token::new(
                        TokenClass::Hash,
                        &current,
                    ))
                },
                TokenClass::Newline => Some (Token::new(
                    TokenClass::Newline,
                    "\n"
                )),
            }
        } else {
            None
        }
    }
}

#[test]
fn simple_tokenize() {
    let example = "hello world!\n# Heading 2  \nOh noes!\t";
    let tokenizer = Tokenizer::new(example.to_string());
    dbg!(tokenizer);
}