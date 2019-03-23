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
mod help;
mod io;
mod log;
mod template;

use command::build::BUILD;
use command::check::CHECK;
use command::delete::DELETE;
use command::init::INIT;
use command::list::LIST;
use command::run::RUN;
use command::Command;
use command::CommandExitCode;
use docker::{DefaultContainerHelper};
use download::{DefaultDownloadHelper};
use help::help;
use help::version;
use io::DefaultInputOutputHelper;
use io::InputOutputHelper;
use log::{DefaultLoggerHelper, EmptyLoggerHelper, LoggerHelper};
use std::env;

const ALL_COMMANDS: &'static [Command] = &[BUILD, CHECK, DELETE, INIT, LIST, RUN];

fn run_command(cmd: &str, args: &[String],
               io_helper: &DefaultInputOutputHelper,
               dck_help: &DefaultContainerHelper,
               dl_helper: &DefaultDownloadHelper,
               log_helper: &LoggerHelper) -> CommandExitCode {
    let mut command_to_run = None;

    for c in ALL_COMMANDS {
        if cmd == c.name || cmd == c.short_name {
            command_to_run = Some(c);
            break;
        }
    }

    match command_to_run {
        Some(c) => c.exec(&args, io_helper, dck_help, dl_helper, log_helper),
        None => {
            io_helper.eprintln(&format!("D-SH: '{}' is not a d-sh command.", cmd));
            io_helper.eprintln(&format!("See '{} --help'", args[0]));

            CommandExitCode::CommandNotFound
        }
    }
}

///
/// Main function of D-SH
///
fn main() {
    // Get command line options
    let args: Vec<String> = env::args().collect();
    // Default exit code
    let mut exit_code = CommandExitCode::Ok;

    let log_helper = &EmptyLoggerHelper{};
    let io_helper = &DefaultInputOutputHelper::new(log_helper);
    let dck_help = &DefaultContainerHelper::new(log_helper);
    let dl_helper = &DefaultDownloadHelper::new(log_helper);

    if args.len() == 1 {
        help(ALL_COMMANDS, io_helper);
        exit_code = CommandExitCode::Help;
    } else {
        let command = &args[1];

        match command.as_str() {
            "-h" | "--help" => help(ALL_COMMANDS, io_helper),
            "-v" | "--version" => version(&args, io_helper),
            "-d" | "--debug" => {
                let log_helper = &DefaultLoggerHelper{};
                let io_helper = &DefaultInputOutputHelper::new(log_helper);
                let dck_help = &DefaultContainerHelper::new(log_helper);
                let dl_helper = &DefaultDownloadHelper::new(log_helper);


                exit_code = run_command(&args[2],&args[3..], io_helper, dck_help, dl_helper, log_helper)
            },
            cmd => {
                exit_code = run_command(cmd,&args[2..], io_helper, dck_help, dl_helper, log_helper)
            }
        };
    }

    // TODO  If application format not good, display help

    std::process::exit(exit_code as i32)
}
