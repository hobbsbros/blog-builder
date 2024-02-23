//! Main library for the Blog Builder.

mod emitter;
mod error;
mod parser;

use std::{
    ffi::OsStr,
    fs,
    io::{Read, Write},
    process,
};

use walkdir::WalkDir;

pub use error::Error;

pub use parser::{
    Parser,
    Expression,
};

pub use emitter::{
    Emitter,
    Metadata,
    CommandOption,
};

const VERSION: &str = "0.1.0";

/// Compiles a file, given its filename.
pub fn compile(metadata: &Metadata) {
    let filename = metadata.get_input();
    println!("Compiling {}", filename.display());

    // Opens the file provided and reads its contents
    let mut file = match fs::OpenOptions::new()
        .read(true)
        .open(&filename)
    {
        Ok (f) => f,
        Err (_) => Error::CannotFindFile.throw_msg(&filename.display()),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok (_) => (),
        Err (_) => Error::CannotReadFile.throw(),
    }

    // Creates a new parser and parses the file contents
    let parser = Parser::new();
    let expressions = parser.parse(&contents);

    let pagename = if expressions.len() > 0 {
        match expressions[0].clone() {
            Expression::Pagename (s) => s.to_owned(),
            _ => "New Page".to_string(),
        }
    } else {
        "New Page".to_string()
    };

    // Creates a new emitter and emits the parser's result
    let emitter = Emitter::new(metadata);
    let emitted = emitter.emit(expressions, &pagename);

    // Write the emitter's result into an HTML file
    let output_filename = &filename.with_extension("html");

    // Are we updating the output file?
    let mut updated = true;

    // Read output file
    // If the output file has updated, 
    match fs::read(&output_filename) {
        Ok (f) => if f == emitted {
            updated = false;
        },
        Err (_) => updated = false,
    };

    if updated {
        let mut output = match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&output_filename)
        {
            Ok (f) => f,
            Err (_) => Error::CannotOpenFile.throw_msg(&output_filename.display()),
        };

        match output.write_all(&emitted) {
            Ok (_) => (),
            Err (_) => Error::CannotWriteFile.throw(),
        }
    }
}

/// Builds a directory into a website.
pub fn build(metadata: &Metadata) {
    let dir = metadata.get_input();

    for entry in WalkDir::new(dir) {
        match entry {
            Ok (e) => if e.path().is_file()
                && e.path().extension() == Some (OsStr::new("txt"))
            {
                let metadata = metadata.with_input(e.path().to_path_buf());
                compile(&metadata);
            },
            Err (_) => Error::CannotReadDir.throw(),
        }
    }
}

pub fn help() {
    println!("Blog Builder");
    println!("Version {}", VERSION);
    println!("Help menu coming soon!");

    process::exit(0);
}