//! Metadata for the Blog Builder emitter module.

use std::{
    env,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::Error;

#[derive(Clone, Debug)]
pub struct Metadata {
    command_option: CommandOption,
    input: PathBuf,
    stylesheet: Option<String>,
    links: Option<String>,
    sitename: Option<String>,
    menu: Option<String>,
    icon: bool,
    analytics: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum CommandOption {
    Compile,
    Build,
    Help,
}

impl Metadata {
    pub fn get() -> Self {
        let args = env::args().collect::<Vec<String>>();

        if args.len() < 3 {
            return Self::default();
        }

        let command_option = match args[1].as_str() {
            "compile" => CommandOption::Compile,
            "build" => CommandOption::Build,
            _ => CommandOption::Help,
        };
        let input = PathBuf::from(args[2].to_owned());

        let mut stylesheet = None;
        let mut links = None;
        let mut sitename = None;
        let mut menu = None;
        let mut icon = false; 
        let mut analytics = None;

        let mut i = 3;
        while i < args.len() {
            match args[i].as_str() {
                "--style" => {
                    Self::set_style(
                        &args[i + 1],
                        &input,
                        &mut stylesheet,
                        &mut links,
                    );
                    i += 2;
                },
                "--sitename" => {
                    sitename = Some (args[i + 1].to_string());
                    i += 2;
                },
                "--menu" => {
                    // Opens the file provided and reads its contents
                    let filename = &args[i + 1];
                    let mut file = match fs::OpenOptions::new()
                        .read(true)
                        .open(filename)
                    {
                        Ok (f) => f,
                        Err (_) => Error::CannotFindFile.throw_msg(filename),
                    };
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Ok (_) => (),
                        Err (_) => Error::CannotReadFile.throw(),
                    }

                    menu = Some (contents);

                    i += 2;
                },
                "--icon" => {
                    icon = true;
                    i += 1;
                },
                "--analytics" => {
                    // Opens the file provided and reads its contents
                    let filename = &args[i + 1];
                    let mut file = match fs::OpenOptions::new()
                        .read(true)
                        .open(filename)
                    {
                        Ok (f) => f,
                        Err (_) => Error::CannotFindFile.throw_msg(filename),
                    };
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Ok (_) => (),
                        Err (_) => Error::CannotReadFile.throw(),
                    }

                    analytics = Some (contents);

                    i += 2;
                },
                _ => return Self::default(),
            }
        }

        Self {
            command_option,
            input,
            stylesheet,
            links,
            sitename,
            menu,
            icon,
            analytics,
        }
    }

    fn set_style(
        style: &str,
        input: &Path,
        stylesheet: &mut Option<String>,
        links: &mut Option<String>,
    ) {
        let stylesheet_path = &input.join("style.css");
        let mut output = match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&stylesheet_path)
        {
            Ok (f) => f,
            Err (_) => Error::CannotOpenFile.throw_msg(&stylesheet_path.display()),
        };
        let stylesheet_css = match style {
            "citizen" => include_str!("metadata/styles/citizen.css"),
            "modern" => include_str!("metadata/styles/modern.css"),
            "truth" => include_str!("metadata/styles/truth.css"),
            "tech" => include_str!("metadata/styles/tech.css"),
            _ => "",
        };
        match output.write_all(stylesheet_css.as_bytes()) {
            Ok (_) => (),
            Err (_) => Error::CannotWriteFile.throw(),
        };
        
        *stylesheet = Some ("<link rel=\"stylesheet\" href=\"/style.css\">".to_string());

        match style {
            "citizen" => {
                *links = Some (include_str!("metadata/links/citizen.html").to_string());
            },
            "modern" => {
                *links = Some (include_str!("metadata/links/modern.html").to_string());
            },
            "truth" => {
                *links = Some (include_str!("metadata/links/truth.html").to_string());
            },
            "tech" => {
                *links = Some (include_str!("metadata/links/tech.html").to_string());
            },
            _ => return,
        }
    }

    fn default() -> Self {
        Self {
            command_option: CommandOption::Help,
            input: PathBuf::new(),
            stylesheet: None,
            links: None,
            sitename: None,
            menu: None,
            icon: false,
            analytics: None,
        }
    }

    pub fn with_input(&self, input: PathBuf) -> Self {
        Self {
            input,
            ..self.clone()
        }
    }

    pub fn get_stylesheet(&self) -> Option<String> {
        self.stylesheet.to_owned()
    }

    pub fn get_links(&self) -> Option<String> {
        self.links.to_owned()
    }

    pub fn get_sitename(&self) -> Option<String> {
        self.sitename.to_owned()
    }

    pub fn get_menu(&self) -> Option<String> {
        self.menu.to_owned()
    }

    pub fn is_icon(&self) -> bool {
        self.icon
    }

    pub fn get_analytics(&self) -> Option<String> {
        self.analytics.to_owned()
    }

    pub fn get_command_option(&self) -> CommandOption {
        self.command_option
    }

    pub fn get_input(&self) -> PathBuf {
        self.input.to_owned()
    }
}