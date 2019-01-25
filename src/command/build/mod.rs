///
/// Module to build application.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use std::env::temp_dir;
use std::error::Error;
use std::collections::HashMap;
use command::Command;
use command::CommandError;
use command::CommandExitCode;
use io::{InputOutputHelper, convert_path};
use docker::ContainerHelper;
use config::{Config, create_config_filename_path};
use config::dockerfile::DOCKERFILE_BASE_FILENAME;
use rand::Rng;
use download::DownloadHelper;
use self::base::build_base;
use self::one::build_one_application;
use template::Template;
use handlebars::TemplateRenderError;
use serde_json::Value;

pub mod base;
pub mod one;
pub mod dockerfile;

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
pub fn generate_dockerfile(config: &Config, io_helper: &InputOutputHelper, output_filename: &str,
    dependencies: &str, data: &Value) -> Result<(), CommandError> {
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
fn build_some_application(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config, applications: &Vec<&String>,
    dl_helper: &DownloadHelper) -> Result<(), CommandError> {
    let mut app_build_fail = HashMap::new();

    for app in applications {
        io_helper.println(&format!("Building {}...", app));

        if let Err(err) = build_one_application(io_helper, dck_helper, &tmp_dir, &options, config, app,
            dl_helper) {
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
fn build(command: &Command, args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, dl_helper: &DownloadHelper,
    config: Option<&Config>) -> Result<(), CommandError> {
    let mut options: BuildOptions = BuildOptions {
        all: false,
        base: false,
        force: false,
        missing: false,
        skip_redownload: false
    };

    // Just get options form command line
    let opts: Vec<&String> = args.iter().filter(|a| a.starts_with("-")).collect();
    // Get applications list from command line
    let applications: Vec<&String> = args.iter().filter(|a| !a.starts_with("-")).collect();

    for argument in opts {
        match argument.as_ref() {
            "-h" | "--help" => {
                io_helper.println(command.usage);
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

    let config = config.unwrap();

    // 1 - Create tmp folder for build
    let mut tmp_dir;

    match &config.tmp_dir {
        Some(t) => tmp_dir = PathBuf::from(convert_path(t)),
        None => tmp_dir = temp_dir()
    }

    tmp_dir.push(random_string());

    match io_helper.create_dir_all(tmp_dir.to_str().unwrap()) {
        Ok(_) => {
            let mut result;

            if options.base {
                io_helper.println("Building base image...");
                result = build_base(io_helper, dck_helper, &tmp_dir, &options, config);
            } else if options.all {
                // TODO
                result = Err(CommandError {
                    msg: vec![String::new()],
                    code: CommandExitCode::Todo
                });
            } else if options.missing {
                // TODO
                result = Err(CommandError {
                    msg: vec![String::new()],
                    code: CommandExitCode::Todo
                });
            } else {
                result = build_some_application(io_helper, dck_helper, &tmp_dir, &options, config,
                    &applications, dl_helper);
            }

            // Remove tmp folder
            remove_tmp_dir(io_helper, &tmp_dir);

            result
        },
        Err(_) => {
            Err(CommandError {
                msg: vec![format!("Cannot create '{}' folder. Please check right!", &tmp_dir.to_str().unwrap())],
                code: CommandExitCode::CannotCreateFolder
            })
        }
    }
}

fn build_tmp(command: &Command, args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, dl_helper: &DownloadHelper,
    config: Option<&Config>) -> CommandExitCode {

    let result = build(command, args, io_helper, dck_helper, dl_helper, config);

    match result {
        Ok(_) => CommandExitCode::Ok,
        Err(err) => {
            for err_msg in &err.msg {
                io_helper.eprintln(err_msg);
            }

            err.code
        }
    }
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
    exec_cmd: build_tmp
};

#[cfg(test)]
mod tests {
    use super::{BUILD, build, UNKOWN_OPTIONS_MESSAGE};
    use config::dockerfile::{DOCKERFILE_BASE_FILENAME, ENTRYPOINT_FILENAME, ENTRYPOINT};
    use io::tests::TestInputOutputHelper;
    use docker::tests::TestContainerHelper;
    use config::{create_config_filename_path, Config, ConfigDocker};
    use command::{CommandError, CommandExitCode};
    use download::tests::TestDownloadHelper;

    fn test_result_ok(result: Result<(), CommandError>) {
        if let Err(err) = result {
            panic!(format!("Command return fail with code {:?} and error message {:?}.",
                err.code, err.msg));
        }
    }

    fn test_result_err(result: Result<(), CommandError>, err_code: CommandExitCode) -> Vec<String> {
        match result {
            Ok(_) => panic!("Command should be fail but not!"),
            Err(err) => {
                assert_eq!(err.code, err_code);

                return err.msg;
            }
        }
    }

    #[test]
    fn build_display_help() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        let args = [String::from("-h")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        test_result_ok(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)));

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "\n    Usage:	d-sh build [OPTIONS] PROGRAM1 PROGRAM2 ...\n\n    Build an image for a program\n\n    Options:\n      -a, --all                Build all image of program\n      -b, --base               Build base image\n      -f, --force              Remove existing image before build\n      -m, --missing            Build only missing image\n      -s, --skip-redownload    If binary is present, don't check if new version is available");
    }

    #[test]
    fn build_unknow_option() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        let args = [String::from("--dghhfhdgfhdgf")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let stderr = test_result_err(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)),
            CommandExitCode::UnknowOption);

        assert_eq!(stderr.get(0).unwrap(), &UNKOWN_OPTIONS_MESSAGE.replace("{}", &args[0]));
    }

    fn build_base_with_args(args: &[String], dck_helper: &TestContainerHelper, config: Config) {
        let io_helper = &TestInputOutputHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{dockerfile_from}} {{#if dockerfile_base}}coucou {{dependencies}}{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Create dockerfile
        match create_config_filename_path(&ENTRYPOINT_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from(ENTRYPOINT))
            },
            None => panic!("Unable to create entrypoint for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d3"));

        test_result_ok(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)));

        // Check if temporary folder was created and remove
        let f = io_helper.files_delete.borrow();

        let mut not_found_dockerfile = true;
        let mut not_found_entrypoint = true;
        let mut generate_dockerfile = String::new();

        for filename in f.keys() {
            if filename.ends_with("/Dockerfile") {
                not_found_dockerfile = false;
                generate_dockerfile = filename.to_string();
                assert_eq!(f.get(filename).unwrap(), "tata coucou d1 d2 d3");
            } else if filename.ends_with("/entrypoint.sh") {
                not_found_entrypoint = false;
                assert_eq!(f.get(filename).unwrap(), ENTRYPOINT);
            }
        }

        if not_found_dockerfile {
            panic!("The temporary Dockerfile in '/tmp/xxx/' folder not found!");
        }

        if not_found_entrypoint {
            panic!("The temporary entrypoint.sh in '/tmp/xxx/' folder not found!");
        }

        let builds = dck_helper.builds.borrow();
        let base_build = builds.get(0).unwrap();

        assert_eq!(base_build.tag, "tutu");
        assert_eq!(generate_dockerfile, base_build.dockerfile_name);
        assert!(generate_dockerfile.starts_with(&base_build.base_dir));

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building base image...");
    }

    #[test]
    fn build_base_short_option() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        build_base_with_args(&[String::from("-b")], dck_helper, config);
    }

    #[test]
    fn build_base_short_option_with_force() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        build_base_with_args(&[String::from("-b"), String::from("-f")], dck_helper, config);

        let builds = dck_helper.builds.borrow();
        let base_build = builds.get(0).unwrap();

        assert_eq!(base_build.build_options.get(0).unwrap(), "--no-cache");
    }

    #[test]
    fn build_base_short_option_dockerfile_template_not_found() {
        let dck_helper = &TestContainerHelper::new();
        let io_helper = &TestInputOutputHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        let args = [String::from("-b")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let dockerfile_name;

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                dockerfile_name = cfg_file;
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Create entrypoint
        match create_config_filename_path(&ENTRYPOINT_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from(ENTRYPOINT))
            },
            None => panic!("Unable to create entrypoint for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d3"));

        let stderr = test_result_err(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)),
            CommandExitCode::TemplateNotFound);

        assert_eq!(stderr.get(0).unwrap(), &format!("The file '{}' doesn't exits. Please run 'init' command first.", dockerfile_name));
    }

    #[test]
    fn build_base_short_option_entrypoint_template_not_found() {
        let dck_helper = &TestContainerHelper::new();
        let io_helper = &TestInputOutputHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        let args = [String::from("-b")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let entrypoint_name;

        // Create dockerfile
        match create_config_filename_path(&ENTRYPOINT_FILENAME) {
            Some(cfg_file) => {
                entrypoint_name = cfg_file;
            },
            None => panic!("Unable to create entrypoint for test")
        };

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("hello man!"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d3"));

        let stderr = test_result_err(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)),
            CommandExitCode::TemplateNotFound);

        assert_eq!(stderr.get(0).unwrap(), &format!("The file '{}' doesn't exits. Please run 'init' command first.", entrypoint_name));
    }

    #[test]
    fn build_base_short_option_application_file_format_bad() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);
        let args = [String::from("-b")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{dockerfile_from}} {{#if dockerfile_base}}coucou{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Create dockerfile
        match create_config_filename_path(&ENTRYPOINT_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from(ENTRYPOINT))
            },
            None => panic!("Unable to create entrypoint for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest"));

        test_result_ok(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)));

        // Check if temporary folder was created and remove
        let f = io_helper.files_delete.borrow();

        let mut not_found_dockerfile = true;
        let mut not_found_entrypoint = true;
        let mut generate_dockerfile = String::new();

        for filename in f.keys() {
            if filename.ends_with("/Dockerfile") {
                not_found_dockerfile = false;
                generate_dockerfile = filename.to_string();
                assert_eq!(f.get(filename).unwrap(), "tata coucou");
            } else if filename.ends_with("/entrypoint.sh") {
                not_found_entrypoint = false;
                assert_eq!(f.get(filename).unwrap(), ENTRYPOINT);
            }
        }

        if not_found_dockerfile {
            panic!("The temporary Dockerfile in '/tmp/xxx/' folder not found!");
        }

        if not_found_entrypoint {
            panic!("The temporary entrypoint.sh in '/tmp/xxx/' folder not found!");
        }

        let builds = dck_helper.builds.borrow();
        let base_build = builds.get(0).unwrap();

        assert_eq!(base_build.tag, "tutu");
        assert_eq!(generate_dockerfile, base_build.dockerfile_name);
        assert!(generate_dockerfile.starts_with(&base_build.base_dir));

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building base image...");

        let stderr = io_helper.stderr.borrow();

        assert_eq!(stderr.get(0).unwrap(), "Cannot read list of dependencies of 'app/filezilla.yml' application, please check right or file format!");
    }

    #[test]
    fn build_base_short_option_dockerfile_template_format_bad() {
        let dck_helper = &TestContainerHelper::new();
        let io_helper = &TestInputOutputHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        let args = [String::from("-b")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{#if base}}dffdfd{{/iG}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Create entrypoint
        match create_config_filename_path(&ENTRYPOINT_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from(ENTRYPOINT))
            },
            None => panic!("Unable to create entrypoint for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d3"));

        let stderr = test_result_err(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)),
            CommandExitCode::DockerfileTemplateInvalid);

        assert_eq!(stderr.get(0).unwrap(), "Something is wrong in Dockerfile template!");
        assert_eq!(stderr.get(1).unwrap(), "wrong name of closing helper");
    }

    #[test]
    fn build_base_short_option_docker_build_fail() {
        let dck_helper = &TestContainerHelper::new();
        let io_helper = &TestInputOutputHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        let args = [String::from("-b")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{#if base}}dffdfd{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Create entrypoint
        match create_config_filename_path(&ENTRYPOINT_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from(ENTRYPOINT))
            },
            None => panic!("Unable to create entrypoint for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d3"));

        dck_helper.builds_error.borrow_mut().insert(config.dockerfile.tag.clone(), true);

        let stderr = test_result_err(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)),
            CommandExitCode::DockerBuildFail);

        assert_eq!(stderr.get(0).unwrap(), "Fail to build base image!");
    }

    #[test]
    fn build_base_short_option_with_specified_tmp_dir() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: Some(String::from("~/.tmp/"))
        };

        build_base_with_args(&[String::from("-b")], dck_helper, config);
    }

    fn build_with_args(args: &[String], io_helper: &TestInputOutputHelper,
        dck_helper: &TestContainerHelper, download_helper: &TestDownloadHelper, config: Config) -> String {

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{dockerfile_from}} {{#if (not dockerfile_base)}}bisous {{application_filename}}{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        test_result_ok(
            build(&BUILD, &args, io_helper, dck_helper, download_helper, Some(&config)));

        // Check if temporary folder was created and remove
        let f = io_helper.files_delete.borrow();

        let mut not_found_dockerfile = true;
        let mut generate_dockerfile = String::new();

        for filename in f.keys() {
            if filename.ends_with("/Dockerfile") {
                not_found_dockerfile = false;
                generate_dockerfile = filename.to_string();
                assert_eq!(f.get(filename).unwrap(), "tutu bisous atom.deb");
            }
        }

        if not_found_dockerfile {
            panic!("The temporary Dockerfile in '/tmp/xxx/' folder not found!");
        }

        generate_dockerfile
    }

    #[test]
    fn build_application() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let io_helper = &TestInputOutputHelper::new();
        // Add application with dependencies

        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"atom.deb\"\nurl: \"toto\"\ndependencies:\n  - d1\n  - d2"));

        let dl_helper = &TestDownloadHelper::new(io_helper);

        let generate_dockerfile = build_with_args(&[String::from("atom")], io_helper, dck_helper, dl_helper, config);

        let downloads = dl_helper.dl.borrow();
        let dl = downloads.get(0).unwrap();

        assert_eq!(dl.output_filename, "dwn/atom.deb");
        assert_eq!(dl.url, "toto");

        let builds = dck_helper.builds.borrow();
        let atom_build = builds.get(0).unwrap();

        assert_eq!(atom_build.tag, "run-atom:latest");
        assert_eq!(generate_dockerfile, atom_build.dockerfile_name);
        assert!(generate_dockerfile.starts_with(&atom_build.base_dir));

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building atom...");
    }

    #[test]
    fn build_application_docker_build_fail() {
        let dck_helper = &TestContainerHelper::new();
        let io_helper = &TestInputOutputHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

        let args = [String::from("atom")];

        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{#if base}}dffdfd{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Create entrypoint
        match create_config_filename_path(&ENTRYPOINT_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from(ENTRYPOINT))
            },
            None => panic!("Unable to create entrypoint for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\"\ndependencies:\n  - d3"));

        dck_helper.builds_error.borrow_mut().insert(String::from("run-atom:latest"), true);

        let stderr = test_result_err(
            build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config)),
            CommandExitCode::DockerBuildFail);

        assert_eq!(stderr.get(0).unwrap(), "Build atom failed!");
        assert_eq!(stderr.get(1).unwrap(), "Cannot build application atom!");
    }

    #[test]
    fn build_application_download_fail() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let io_helper = &TestInputOutputHelper::new();
        // Add application with dependencies

        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"atom.deb\"\nurl: \"toto\"\ndependencies:\n  - d1\n  - d2"));

        let dl_helper = &TestDownloadHelper::new(io_helper);

        dl_helper.urls_error.borrow_mut().insert(String::from("toto"), true);

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{dockerfile_from}} {{#if (not dockerfile_base)}}bisous {{application_filename}}{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        let stderr = test_result_err(
            build(&BUILD, &[String::from("atom")], io_helper, dck_helper, dl_helper, Some(&config)),
            CommandExitCode::DockerBuildFail);

        assert_eq!(stderr.get(0).unwrap(), "Build atom failed!");
        assert_eq!(stderr.get(1).unwrap(), "Unable to download application 'atom'!");
    }

    #[test]
    fn build_application_download_already_done() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let io_helper = &TestInputOutputHelper::new();
        // Add application with dependencies

        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"atom.deb\"\nurl: \"toto\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("dwn/atom.deb"), String::from("Go, go, go !"));

        let dl_helper = &TestDownloadHelper::new(io_helper);

        let generate_dockerfile = build_with_args(&[String::from("atom")], io_helper, dck_helper, dl_helper, config);

        let downloads = dl_helper.dl.borrow();
        let dl = downloads.get(0).unwrap();

        assert_eq!(dl.output_filename, "dwn/atom.deb");
        assert_eq!(dl.url, "toto");
        assert_eq!(dl.update, false);

        assert_eq!(io_helper.files.borrow().get("dwn/atom.deb").unwrap(), "Go, go, go !");

        let builds = dck_helper.builds.borrow();
        let atom_build = builds.get(0).unwrap();

        assert_eq!(atom_build.tag, "run-atom:latest");
        assert_eq!(generate_dockerfile, atom_build.dockerfile_name);
        assert!(generate_dockerfile.starts_with(&atom_build.base_dir));

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building atom...");
    }

    #[test]
    fn build_application_skip_download() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let io_helper = &TestInputOutputHelper::new();
        // Add application with dependencies

        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"atom.deb\"\nurl: \"toto\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("dwn/atom.deb"), String::from("Go, go, go !"));

        let dl_helper = &TestDownloadHelper::new(io_helper);

        let downloads = dl_helper.update_dl_files.borrow_mut().insert(String::from("dwn/atom.deb"), true);

        let generate_dockerfile = build_with_args(&[String::from("-s"), String::from("atom")], io_helper, dck_helper, dl_helper, config);

        let downloads = dl_helper.dl.borrow();

        assert_eq!(downloads.len(), 0);

        assert_eq!(io_helper.files.borrow().get("dwn/atom.deb").unwrap(), "Go, go, go !");

        let builds = dck_helper.builds.borrow();
        let atom_build = builds.get(0).unwrap();

        assert_eq!(atom_build.tag, "run-atom:latest");
        assert_eq!(generate_dockerfile, atom_build.dockerfile_name);
        assert!(generate_dockerfile.starts_with(&atom_build.base_dir));

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building atom...");
    }

    #[test]
    fn build_application_with_force() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let io_helper = &TestInputOutputHelper::new();
        // Add application with dependencies

        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"atom.deb\"\nurl: \"toto\"\ndependencies:\n  - d1\n  - d2"));

        let dl_helper = &TestDownloadHelper::new(io_helper);

        let generate_dockerfile = build_with_args(&[String::from("-f"), String::from("atom")], io_helper, dck_helper, dl_helper, config);

        let downloads = dl_helper.dl.borrow();
        let dl = downloads.get(0).unwrap();

        assert_eq!(dl.output_filename, "dwn/atom.deb");
        assert_eq!(dl.url, "toto");

        let builds = dck_helper.builds.borrow();
        let atom_build = builds.get(0).unwrap();

        assert_eq!(atom_build.tag, "run-atom:latest");
        assert_eq!(generate_dockerfile, atom_build.dockerfile_name);
        assert!(generate_dockerfile.starts_with(&atom_build.base_dir));

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building atom...");

        let builds = dck_helper.builds.borrow();
        let app_build = builds.get(0).unwrap();

        assert_eq!(app_build.build_options.get(0).unwrap(), "--no-cache");
    }

    #[test]
    fn build_application_many_application() {
        let dck_helper = &TestContainerHelper::new();
        // Create configuration file
        let config = Config {
            download_dir: String::from("dwn"),
            applications_dir: String::from("app"),
            dockerfile: ConfigDocker {
                from: String::from("tata"),
                tag: String::from("tutu")
            },
            tmp_dir: None
        };

        let io_helper = &TestInputOutputHelper::new();
        // Add application with dependencies

        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"atom.deb\"\nurl: \"toto\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"filezilla.deb\"\nurl: \"titi\"\ndependencies:\n  - d1\n  - d2"));

        let dl_helper = &TestDownloadHelper::new(io_helper);

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{dockerfile_from}} {{#if (not dockerfile_base)}}bisous {{application_filename}}{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        test_result_ok(
            build(&BUILD, &[String::from("atom"), String::from("filezilla")], io_helper, dck_helper, dl_helper, Some(&config)));

        let downloads = dl_helper.dl.borrow();
        let dl = downloads.get(0).unwrap();

        assert_eq!(dl.output_filename, "dwn/atom.deb");
        assert_eq!(dl.url, "toto");

        let dl = downloads.get(1).unwrap();

        assert_eq!(dl.output_filename, "dwn/filezilla.deb");
        assert_eq!(dl.url, "titi");

        let builds = dck_helper.builds.borrow();
        let atom_build = builds.get(0).unwrap();

        assert_eq!(atom_build.tag, "run-atom:latest");

        let filezilla_build = builds.get(1).unwrap();

        assert_eq!(filezilla_build.tag, "run-filezilla:latest");

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building atom...");
        assert_eq!(stdout.get(1).unwrap(), "Building filezilla...");

    }

    // TODO These test need more better implementation of folder/file in test.
    // Create a real tree with hook when create, read, update, delete
    //  - test: build test with generate Dockerfile/entry.sh error caused by folder error
    //  - test: build test with delete folder error caused by folder error

    // TODO add switch helper in handlebars to allow install package

    // TODO build all
    // TODO build missing application

    // TODO check if ctrl+c on curl

    // TODO only one parameter in command (use struct)
    // TODO get_config_application() display error
    // TODO use CommandError
}
