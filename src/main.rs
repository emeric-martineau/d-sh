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
mod common;
mod docker;
mod help;
mod io;

use std::env;
use command::Command;
use command::check::CHECK;
use command::init::INIT;
use help::help;
use help::version;
use io::DefaultInputOutputHelper;
use io::InputOutputHelper;
use docker::DefaultContainerHelper;

const ALL_COMMANDS: &'static [Command] = &[CHECK, INIT];

///
/// Main function of D-SH
///
fn main() {
    // Get command line options
    let args: Vec<String> = env::args().collect();
    // Default exit code
    let mut exit_code: i32 = 0;

    let io_helper = &mut DefaultInputOutputHelper;
    let dck_help = &mut DefaultContainerHelper;

    if args.len() == 1 {
        help(ALL_COMMANDS, io_helper);
        exit_code = 1
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

                        2
                    }
                }
            }
        };
    }

    std::process::exit(exit_code)
}
