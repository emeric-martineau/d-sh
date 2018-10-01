//!
//! # D-SH a tool to create easy container's applications.
//!
//! Release under MIT License.
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate regex;

extern crate glob;

mod command;
mod config;
mod docker;
mod help;
mod io;

use std::env;
use command::Command;
use command::CommandExitCode;
use command::check::CHECK;
use command::delete::DELETE;
use command::init::INIT;
use command::list::LIST;
use help::help;
use help::version;
use io::DefaultInputOutputHelper;
use io::InputOutputHelper;
use docker::DefaultContainerHelper;

const ALL_COMMANDS: &'static [Command] = &[CHECK, DELETE, INIT, LIST];

///
/// Main function of D-SH
///
fn main() {
    // Get command line options
    let args: Vec<String> = env::args().collect();
    // Default exit code
    let mut exit_code = CommandExitCode::Ok;

    let io_helper = &DefaultInputOutputHelper;
    let dck_help = &DefaultContainerHelper;

    if args.len() == 1 {
        help(ALL_COMMANDS, io_helper);
        exit_code = CommandExitCode::Help;
    } else {
        let command = &args[1];

        match command.as_str() {
            "-h" | "--help" => help(ALL_COMMANDS, io_helper),
            "-v" | "--version" => version(&args, io_helper),
            cmd => {
                let mut command_to_run = None;

                for c in ALL_COMMANDS {
                    if cmd == c.name {
                        command_to_run = Some(c);
                    }
                }

                exit_code = match command_to_run {
                    Some(c) => c.exec(&args[2..], io_helper, dck_help),
                    None => {
                        io_helper.eprintln(&format!("D-SH: '{}' is not a d.sh command.", cmd));
                        io_helper.eprintln(&format!("See '{} --help'", args[0]));

                        CommandExitCode::CommandNotFound
                    }
                }
            }
        };
    }

    std::process::exit(exit_code as i32)
}
