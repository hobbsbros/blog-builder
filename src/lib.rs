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

use text_diff::{
    diff,
    Difference,
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
    // 
    // Read output file.. if the output file has updated,
    // overwrite it with the new date
    let updated = match fs::read(&output_filename) {
        Ok (f) => {
            let (_dist, changelist) = diff(
                &String::from_utf8(f).unwrap(),
                &String::from_utf8(emitted.clone()).unwrap(),
                "\n",
            );

            // Changes that *do not apply* to updated date
            let mut changes = 0;

            let mut is_new_date = false;

            // Update in the following conditions:
            // - Last Updated date added
            // - Last Updated date removed
            // - Something other than Last Updated date is new
            for change in &changelist {
                if let Difference::Same (_) = change {
                    // do nothing
                } else if let Difference::Add (a) = change {
                    if a.contains("last-updated-date") && !a.contains("\n\n") {
                        is_new_date = !is_new_date;
                    } else {
                        changes += 1;
                    }
                } else if let Difference::Rem (r) = change {
                    if r.contains("last-updated-date") && !r.contains("\n\n"){
                        is_new_date = !is_new_date;
                    } else {
                        changes += 1;
                    }
                }
            }

            changes > 0 || is_new_date
        },

        // If you can't find the original file, you do need
        // to "update" (by creating a new file)
        Err (_) => true,
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