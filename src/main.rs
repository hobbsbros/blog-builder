//! Main executable for the Blog Builder.

use utils::{
    CommandOption,
    Metadata,
    compile,
    help,
    build,
};

fn main() {
    let metadata = Metadata::get();
    let command_option = metadata.get_command_option();

    match command_option {
        CommandOption::Compile => compile(&metadata),
        CommandOption::Build => build(&metadata),
        CommandOption::Help => help(),
    }
}