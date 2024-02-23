//! Error handler for the Blog Builder.

use std::{
    fmt::{
        Display,
        Formatter,
        Result,
    },
    process::exit,
};

pub enum Error {
    CannotGetWorkingDirectory,
    UnrecognizedToken,
    UnrecognizedControlSequence,
    TooManyHashes,
    UnexpectedEof,
    ExpectedTokenOfClass,
    CannotFindFile,
    CannotReadFile,
    CannotOpenFile,
    CannotWriteFile,
    CannotExtractFileStem,
    CannotReadDir,
}

impl Error {
    pub fn throw(&self) -> ! {
        println!("{}", self);
        
        exit(0);
    }

    pub fn throw_msg<S: Display + ?Sized>(&self, msg: &S) -> ! {
        println!("{}: {}", self, msg);
        
        exit(0);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use Error::*;

        let msg = match self {
            CannotGetWorkingDirectory => "cannot get working directory",
            UnrecognizedToken => "unrecognized token",
            UnrecognizedControlSequence => "unrecongized control sequence",
            TooManyHashes => "too many hashes",
            UnexpectedEof => "unexpected end of file",
            ExpectedTokenOfClass => "expected token of class",
            CannotFindFile => "cannot find file",
            CannotReadFile => "cannot read file",
            CannotOpenFile => "cannot open file",
            CannotWriteFile => "cannot write to file",
            CannotExtractFileStem => "cannot extract file stem",
            CannotReadDir => "cannot read input directory",
        };

        write!(f, "[ERROR] {}", msg)
    }
}