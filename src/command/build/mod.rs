///
/// Module to build application.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use std::env::temp_dir;
use std::error::Error;
use std::collections::HashMap;
use command::{Command, CommandError, CommandExitCode, CommandParameter};
use io::{InputOutputHelper, convert_path};
use config::{Config, create_config_filename_path};
use config::dockerfile::DOCKERFILE_BASE_FILENAME;
use rand::Rng;
use self::all::build_all;
use self::base::build_base;
use self::one::build_one_application;
use self::missing::get_missing_application;
use template::Template;
use handlebars::TemplateRenderError;
use serde_json::Value;

mod all;
mod base;
mod dockerfile;
mod missing;
mod one;
#[cfg(test)]
mod tests;

///
/// Option for build command.
///
pub struct BuildOptions {
    /// Build all image
    all: bool,
    /// Build base image
    base: bool,
    /// Force build even if exists
    force: bool,
    /// Build missing image
    missing: bool,
    /// Never checl if binary are update
    skip_redownload: bool
}

const UNKOWN_OPTIONS_MESSAGE: &'static str = "d-sh build: invalid option '{}'\nTry 'd-sh build --help' for more information.\n";

///
/// Generate a random string.
///
fn random_string () -> String {
    let mut rng = rand::thread_rng();
    let letter: char = rng.gen_range(b'A', b'Z') as char;
    let number: u32 = rng.gen_range(0, 999999);

    format!("{}{:06}", letter, number)
}

///
/// Remove folder.
///
fn remove_tmp_dir(io_helper: &InputOutputHelper, tmp_dir: &PathBuf) -> CommandExitCode {
    match io_helper.remove_dir_all(tmp_dir.to_str().unwrap()) {
        Ok(_) => CommandExitCode::Ok,
        Err(_) => CommandExitCode::CannotDeleteTemporaryFolder
    }
}

///
/// Generate template of dockerfile.
///
fn generate_dockerfile(io_helper: &InputOutputHelper, output_filename: &str,
    data: &Value) -> Result<(), CommandError> {
    let handlebars = Template::new();

    let dockerfile_name;

    match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
        Some(r) => dockerfile_name = r,
        None => return Err(CommandError {
            msg: vec![String::from("Unable to get your home dir!")],
            code: CommandExitCode::CannotGetHomeFolder
        })
    }

    if ! io_helper.file_exits(&dockerfile_name) {
        return Err(CommandError {
            msg: vec![format!("The file '{}' doesn't exits. Please run 'init' command first.",
                dockerfile_name)],
            code: CommandExitCode::TemplateNotFound
        });
    }

    let source_template;

    match io_helper.file_read_at_string(&dockerfile_name) {
        Ok(r) => source_template = r,
        Err(err) => return Err(CommandError {
            msg: vec![
                String::from("Unable to read Dockerfile template. Please check right!"),
                format!("{}", err)
                ],
            code: CommandExitCode::CannotGenerateDockerfile
        })
    }

    let content;

    match handlebars.render_template(&source_template, &data) {
        Ok(r) => content = r,
        Err(err) => {
            let err_msg;

            match err {
                TemplateRenderError::TemplateError(err) => err_msg = String::from(err.description()),
                TemplateRenderError::RenderError(err) => err_msg = String::from(err.description()),
                TemplateRenderError::IOError(_, msg) => err_msg = msg
            }

            return Err(CommandError {
                msg: vec![
                    String::from("Something is wrong in Dockerfile template!"),
                    err_msg
                    ],
                code: CommandExitCode::DockerfileTemplateInvalid
            });
        }
    }

    if let Err(err) = io_helper.file_write(&output_filename, &content) {
        return Err(CommandError {
            msg: vec![
                String::from("Unable to generate Dockerfile for build. Please check right!"),
                format!("{}", err)
                ],
            code: CommandExitCode::CannotGenerateDockerfile
        });
    }

    Ok(())
}

///
/// Build one application.
///
///
fn build_some_application(cmd_param: &CommandParameter, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config,
    applications: &Vec<String>) -> Result<(), CommandError> {
    let mut app_build_fail = HashMap::new();

    for app in applications {
        cmd_param.io_helper.println(&format!("Building {}...", app));

        if let Err(err) = build_one_application(cmd_param, &tmp_dir, &options, config, app) {
            app_build_fail.insert(app, err);
        }
    }

    if app_build_fail.is_empty() {
        Ok(())
    } else {
        let mut err_msg = Vec::new();

        for (app, err) in app_build_fail {
            err_msg.push(format!("Build {} failed!", &app));
            err_msg.extend(err.msg);
        }

        return Err(CommandError {
            msg: err_msg,
            code: CommandExitCode::DockerBuildFail
        });
    }
}

///
/// Function to implement build D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn build(cmd_param: CommandParameter) -> Result<(), CommandError> {
    let mut options: BuildOptions = BuildOptions {
        all: false,
        base: false,
        force: false,
        missing: false,
        skip_redownload: false
    };

    // Just get options form command line
    let opts: Vec<&String> = cmd_param.args
        .iter()
        .filter(|a| a.starts_with("-"))
        .collect();
    // Get applications list from command line
    let applications: Vec<String> = cmd_param.args
        .iter()
        .filter(|a| !a.starts_with("-"))
        .map(|a| a.clone())
        .collect();

    for argument in opts {
        match argument.as_ref() {
            "-h" | "--help" => {
                cmd_param.io_helper.println(cmd_param.command.usage);
                return Ok(());
            },
            "-a" | "--all" => options.all = true,
            "-b" | "--base" => options.base = true,
            "-f" | "--force" => options.force = true,
            "-m" | "--missing" => options.missing = true,
            "-s" | "--skip-redownload" => options.skip_redownload = true,
            other => {
                return Err(CommandError {
                    msg: vec![UNKOWN_OPTIONS_MESSAGE.replace("{}", other)],
                    code: CommandExitCode::UnknowOption
                })
            }
        }
    }

    let config = cmd_param.config.unwrap();

    // 1 - Create tmp folder for build
    let mut tmp_dir;

    match &config.tmp_dir {
        Some(t) => tmp_dir = PathBuf::from(convert_path(t)),
        None => tmp_dir = temp_dir()
    }

    tmp_dir.push(random_string());

    if let Err(err) = cmd_param.io_helper.create_dir_all(tmp_dir.to_str().unwrap()) {
        return Err(CommandError {
            msg: vec![
                format!("Cannot create '{}' folder. Please check right!", &tmp_dir.to_str().unwrap()),
                format!("{}", err)
                ],
            code: CommandExitCode::CannotCreateFolder
        });
    }

    let result;

    if options.base {
        cmd_param.io_helper.println("Building base image...");
        result = build_base(&cmd_param, &tmp_dir, &options, &config);
    } else if options.all {
        result = build_all(&cmd_param, &options, &config, &tmp_dir);
    } else if options.missing {
        match get_missing_application(&cmd_param, &config) {
            Ok(list_applications) => result = build_some_application(&cmd_param,
                &tmp_dir, &options, &config, &list_applications),
            Err(err) => result = Err(err)
        }
    } else {
        result = build_some_application(&cmd_param, &tmp_dir, &options, &config,
            &applications);
    }

    // Remove tmp folder
    remove_tmp_dir(cmd_param.io_helper, &tmp_dir);

    result
}

///
/// The `list` command.
///
pub const BUILD: Command = Command {
    /// This command call by `check`.
    name: "build",
    /// description.
    description: "Build container image",
    /// Short name.
    short_name: "b",
    /// `check` command have no parameter.
    min_args: 1,
    max_args: std::usize::MAX,
    /// `check` command have no help.
    usage: "
    Usage:	d-sh build [OPTIONS] PROGRAM1 PROGRAM2 ...

    Build an image for a program

    Options:
      -a, --all                Build all image of program
      -b, --base               Build base image
      -f, --force              Remove existing image before build
      -m, --missing            Build only missing image
      -s, --skip-redownload    If binary is present, don't check if new version is available",
    need_config_file: true,
    exec_cmd: build
};
