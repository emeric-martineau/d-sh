//!
//! # D-SH a tool to create easy container's applications.
//!
//! Release under MIT License.
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[cfg(test)]
extern crate regex;

extern crate dirs;
extern crate glob;
extern crate handlebars;
extern crate rand;
extern crate users;

mod command;
mod config;
mod docker;
mod download;
mod io;
mod log;
mod template;

use command::{CommandExitCode, ALL_COMMANDS};
use docker::{DefaultContainerHelper};
use download::{DefaultDownloadHelper};
use config::get_config_filename;
use io::DefaultInputOutputHelper;
use io::InputOutputHelper;
use log::{DefaultLoggerHelper, EmptyLoggerHelper, LoggerHelper};
use std::env;



fn run_command<'r>(cmd: &str, args: &[String], config_filename : Option<String>, debug: bool) -> CommandExitCode {
    let mut command_to_run = None;

    for c in ALL_COMMANDS {
        if cmd == c.name || cmd == c.short_name {
            command_to_run = Some(c);
            break;
        }
    }

    let log_helper : &'r LoggerHelper;

    if debug {
        log_helper = &DefaultLoggerHelper{};
    } else {
        log_helper = &EmptyLoggerHelper{};
    }

    let io_helper = DefaultInputOutputHelper::new(log_helper);
    let dck_helper = DefaultContainerHelper::new(log_helper);
    let dl_helper = DefaultDownloadHelper::new(log_helper);

    match command_to_run {
        Some(c) => c.exec(&args, config_filename, &io_helper, &dck_helper, &dl_helper, log_helper),
        None => {
            io_helper.eprintln(&format!("D-SH: '{}' is not a d-sh command.", cmd));
            io_helper.eprintln(&format!("See '{} --command.help'", args[0]));

            CommandExitCode::CommandNotFound
        }
    }
}

///
/// Main function of D-SH
///
fn main() {
    // Get command line options
    let mut args: Vec<String> = env::args().collect();
    // Default exit code
    let mut exit_code = CommandExitCode::Todo;
    let mut debug = false;
    let mut config_filename : Option<String> = None;

    if args.len() == 1 {
        run_command("help", &[], None, false);

        exit_code = CommandExitCode::Help;
    } else {
        while exit_code == CommandExitCode::Todo {
            let command = args[1].clone();

            match command.as_str() {
                "-c" | "--config" => {
                    config_filename = Some(args[2].clone());

                    args.remove(1);
                    args.remove(1);
                },
                "-d" | "--debug" => {
                    debug = true;

                    args.remove(1);
                },
                cmd => {
                    if config_filename.is_none() {
                        config_filename = get_config_filename();
                    }

                    exit_code = run_command(cmd,&args[2..], config_filename.clone(), debug)
                }
            };
        }
    }

    // TODO  If application format not good, display command.help

    std::process::exit(exit_code as i32)
}
