///
/// Module to delete image.
///
/// Release under MIT License.
///
use std::path::Path;
use users::{get_current_uid, get_current_gid, get_current_username};
use command::{Command, CommandExitCode};
use io::{InputOutputHelper, convert_path};
use docker::ContainerHelper;
use config::{Config, ConfigApplication, get_config_application};
use download::DownloadHelper;

#[cfg(test)]
mod tests;

///
/// Construct extra args of run
///
/// `interactive` true if command line interactive
/// `config_application` configuration of current application
///
/// returning vector of string
///
fn get_extra_args(interactive: bool, config_application: &ConfigApplication) -> Vec<String> {
    let mut extra_args: Vec<String> = vec![];

    if interactive || config_application.interactive.unwrap_or(false) {
        extra_args.push(String::from("-it"));
    } else {
        extra_args.push(String::from("-d"));
    }

    if config_application.ipc_host.unwrap_or(false) {
        extra_args.push(String::from("--ipc=host"));
    }

    extra_args
}

///
/// Construct run args of run
///
/// `extra_args` extra arguments
/// `username` username
///
/// returning vector of string
///
fn get_run_args(extra_args: &mut Vec<String>, username: String) -> Vec<String> {
    let mut run_opts: Vec<String> = vec![
        String::from("-v"),
        String::from("/tmp/.X11-unix/:/tmp/.X11-unix/"),
        String::from("-v"),
        String::from("/dev/shm:/dev/shm"),
        String::from("-v"),
        format!("{}:/home/{}", convert_path("~/"), username),
        String::from("-e"),
        String::from("DISPLAY"),
        String::from("-e"),
        format!("USERNAME_TO_RUN={}", username),
        String::from("-e"),
        format!("USERNAME_TO_RUN_GID={}", get_current_gid()),
        String::from("-e"),
        format!("USERNAME_TO_RUN_UID={}", get_current_uid()),
        String::from("--rm")];

    run_opts.append(extra_args);

    run_opts
}

///
/// Convert args to cmd_args
///
/// `args` command line args
///
/// returning vector of string
///
fn get_cmd_args(cmd_line_args: &Option<Vec<String>>, args: &[String]) -> Vec<String> {
    let mut cmd_args: Vec<String> = Vec::new();

    if cmd_line_args.is_some() {
        let cla = &cmd_line_args.as_ref().unwrap();

        for arg in *cla {
            cmd_args.push(arg.to_owned());
        }
    }

    for arg in args {
        cmd_args.push(arg.to_owned());
    }

    cmd_args
}

///
/// Function to run one image.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn run_application(config: &Config, app: &str, io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, args: &[String], interactive: bool)  -> CommandExitCode {

    io_helper.println(&format!("Running {}...", app));

    let mut application_filename = String::from(app);
    application_filename.push_str(".yml");

    let application_filename_path = Path::new(&config.applications_dir)
        .join(&application_filename);

    let application_filename_full_path = application_filename_path
        .to_str()
        .unwrap();

    match get_config_application(io_helper, &application_filename_full_path) {
        Ok(config_application) => {
            // Check if image exists
            let images = dck_helper.list_image(&config_application.image_name);

            if images.len() > 0 {
                io_helper.println("Create container");

                match get_current_username() {
                    Some(username) => {
                        let mut extra_args = get_extra_args(interactive, &config_application);
                        let run_opts = get_run_args(&mut extra_args, username);

                        let cmd_args = get_cmd_args(&config_application.cmd_line_args, args);

                        if dck_helper.run_container(&config_application.image_name, Some(&run_opts),
                            Some(&config_application.cmd_line), Some(&cmd_args)) {
                            CommandExitCode::Ok
                        } else {
                            CommandExitCode::ContainerRunError
                        }
                    },
                    None => {
                        io_helper.eprintln("Cannot get current user !");
                        CommandExitCode::CannotGetCurrentUser
                    }
                }
            } else {
                io_helper.eprintln(&format!("Image for program {} not found.", app));
                io_helper.eprintln("");
                io_helper.eprintln("Build it before with:");
                io_helper.eprintln(&format!("  d-sh build {}", app));

                CommandExitCode::ContainerImageNotFound
            }
        },
        Err(_) => {
            io_helper.eprintln(&format!("Application '{}' not found.", app));
            CommandExitCode::ApplicationFileNotFound
        }
    }
}

///
/// Function to implement delete D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn run(command: &Command, args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, _dl_helper: &DownloadHelper,
    config: Option<&Config>) -> CommandExitCode {

    let config = config.unwrap();

    match args[0].as_ref() {
        "-h" | "--help" => {
            io_helper.println(command.usage);
            CommandExitCode::Ok
        },
        "-i" | "--interactive" => {
            if args.len() > 1 {
                run_application(&config, &args[1], io_helper, dck_helper, &args[2..], true)
            } else {
                io_helper.eprintln("You must specify an application !");

                CommandExitCode::ApplicationNameMissing
            }
        },
        app => {
            run_application(&config, &app, io_helper, dck_helper, &args[1..], false)
        }
    }
}

///
/// The `run` command.
///
pub const RUN: Command = Command {
    /// This command call by `check`.
    name: "run",
    /// description.
    description: "Run container",
    /// Short name.
    short_name: "",
    /// `check` command have no parameter.
    min_args: 1,
    max_args: 2,
    /// `check` command have no help.
    usage: "
    Usage:	d-sh run [-i | --interactive] APPLICATION [APPLICATION ARGS]

    Run an application

    Options:
      -i | --interactive       Run application in terminal
",
    need_config_file: true,
    exec_cmd: run
};
