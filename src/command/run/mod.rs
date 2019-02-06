///
/// Module to delete image.
///
/// Release under MIT License.
///
use std::path::Path;
use users::{get_current_uid, get_current_gid, get_current_username};
use command::{Command, CommandExitCode, CommandError, CommandParameter};
use io::{InputOutputHelper, convert_path};
use docker::ContainerHelper;
use config::{Config, ConfigApplication, get_config_application};

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
    dck_helper: &ContainerHelper, args: &[String], interactive: bool) -> Result<(), CommandError> {

    io_helper.println(&format!("Running {}...", app));

    let mut application_filename = String::from(app);
    application_filename.push_str(".yml");

    let application_filename_path = Path::new(&config.applications_dir)
        .join(&application_filename);

    let application_filename_full_path = application_filename_path
        .to_str()
        .unwrap();

    let config_application;

    match get_config_application(io_helper, &application_filename_full_path) {
        Ok(r) => config_application = r,
        Err(err) => return Err(CommandError {
            msg: vec![
                format!("Application '{}' not found.", app),
                format!("{}", err)
                ],
            code: CommandExitCode::ApplicationFileNotFound
        })
    }

    // Check if image exists
    let images = dck_helper.list_image(&config_application.image_name);

    if images.len() > 0 {
        io_helper.println("Create container");

        let username;

        match get_current_username() {
            Some(r) => username = r,
            None => return Err(CommandError {
                msg: vec![String::from("Cannot get current user !")],
                code: CommandExitCode::CannotGetCurrentUser
            })
        }

        let mut extra_args = get_extra_args(interactive, &config_application);
        let run_opts = get_run_args(&mut extra_args, username);

        let cmd_args = get_cmd_args(&config_application.cmd_line_args, args);

        if dck_helper.run_container(&config_application.image_name, Some(&run_opts),
            Some(&config_application.cmd_line), Some(&cmd_args)) {
            Ok(())
        } else {
            return Err(CommandError {
                msg: vec![String::from("Error when running container")],
                code: CommandExitCode::ContainerRunError
            })
        }
    } else {
        Err(CommandError {
            msg: vec![
                format!("Image for program {} not found.", app),
                String::from(""),
                String::from("Build it before with:"),
                format!("  d-sh build {}", app)
                ],
            code: CommandExitCode::ContainerImageNotFound
        })
    }
}

///
/// Function to implement delete D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn run(cmd_param: CommandParameter) -> Result<(), CommandError> {

    let config = cmd_param.config.unwrap();

    match cmd_param.args[0].as_ref() {
        "-h" | "--help" => {
            cmd_param.io_helper.println(cmd_param.command.usage);
            Ok(())
        },
        "-i" | "--interactive" => {
            if cmd_param.args.len() > 1 {
                run_application(&config, &cmd_param.args[1], cmd_param.io_helper,
                    cmd_param.dck_helper, &cmd_param.args[2..], true)
            } else {
                Err(CommandError {
                    msg: vec![String::from("You must specify an application !")],
                    code: CommandExitCode::ApplicationNameMissing
                })
            }
        },
        app => {
            run_application(&config, &app, cmd_param.io_helper, cmd_param.dck_helper,
                &cmd_param.args[1..], false)
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
    short_name: "r",
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
