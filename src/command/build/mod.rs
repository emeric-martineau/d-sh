///
/// Module to build application.
///
/// Release under MIT License.
///
use std::path::PathBuf;
use std::env::temp_dir;
use std::error::Error;
use command::Command;
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
    dependencies: &str, data: &Value) -> Result<(), CommandExitCode> {
    let handlebars = Template::new();

    match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
        Some(dockerfile_name) => {
            if ! io_helper.file_exits(&dockerfile_name) {
                io_helper.eprintln(&format!("The file '{}' doesn't exits. Please run 'init' command first.", dockerfile_name));
                return Err(CommandExitCode::TemplateNotFound);
            }

            match io_helper.file_read_at_string(&dockerfile_name) {
                Ok(mut source_template) => {
                    match handlebars.render_template(&source_template, &data) {
                        Ok(content) => {
                            match io_helper.file_write(&output_filename, &content) {
                                Ok(_) => Ok(()),
                                Err(_) => {
                                    io_helper.eprintln("Unable to generate Dockerfile for build. Please check right!");
                                    Err(CommandExitCode::CannotGenerateDockerfile)
                                }
                            }
                        },
                        Err(err) => {
                            match err {
                                TemplateRenderError::TemplateError(err) => io_helper.eprintln(err.description()),
                                TemplateRenderError::RenderError(err) => io_helper.eprintln(err.description()),
                                TemplateRenderError::IOError(_, msg) => io_helper.eprintln(&msg)
                            }

                            io_helper.eprintln("Something is wrong in Dockerfile template!");
                            Err(CommandExitCode::DockerfileTemplateInvalid)
                        }
                    }
                },
                Err(_) => {
                    io_helper.eprintln("Unable to read Dockerfile template. Please check right!");
                    Err(CommandExitCode::CannotGenerateDockerfile)
                }
            }
        },
        None => {
            io_helper.eprintln("Unable to get your home dir!");
            Err(CommandExitCode::CannotGetHomeFolder)
        }
    }
}

///
/// Build one application.
///
///
fn build_some_application(io_helper: &InputOutputHelper, dck_helper: &ContainerHelper, tmp_dir: &PathBuf,
    options: &BuildOptions, config: &Config, applications: &Vec<&String>,
    dl_helper: &DownloadHelper) -> CommandExitCode {
    let mut app_build_fail = Vec::new();

    for app in applications {
        io_helper.println(&format!("Building {}...", app));

        if build_one_application(io_helper, dck_helper, &tmp_dir, &options, config, app,
            dl_helper).is_err() {
            app_build_fail.push(app);
        }
    }

    if app_build_fail.is_empty() {
        CommandExitCode::Ok
    } else {
        for app in app_build_fail {
            io_helper.eprintln(&format!("Build {} failed!", app));
        }
        // TODO
        CommandExitCode::Todo
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
    config: Option<&Config>) -> CommandExitCode {
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
                return CommandExitCode::Ok;
            },
            "-a" | "--all" => options.all = true,
            "-b" | "--base" => options.base = true,
            "-f" | "--force" => options.force = true,
            "-m" | "--missing" => options.missing = true,
            "-s" | "--skip-redownload" => options.skip_redownload = true,
            other => {
                io_helper.eprintln(&UNKOWN_OPTIONS_MESSAGE.replace("{}", other));
                return CommandExitCode::UnknowOption;
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
                result = CommandExitCode::Todo;
            } else if options.missing {
                // TODO
                result = CommandExitCode::Todo;
            } else {
                result = build_some_application(io_helper, dck_helper, &tmp_dir, &options, config,
                    &applications, dl_helper);
            }

            // Remove tmp folder
            remove_tmp_dir(io_helper, &tmp_dir);

            result
        },
        Err(_) => {
            io_helper.eprintln(&format!("Cannot create '{}' folder. Please check right!", &tmp_dir.to_str().unwrap()));
            CommandExitCode::CannotCreateFolder
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
    exec_cmd: build
};

#[cfg(test)]
mod tests {
    use super::{BUILD, build, UNKOWN_OPTIONS_MESSAGE};
    use config::dockerfile::{DOCKERFILE_BASE_FILENAME, ENTRYPOINT_FILENAME, ENTRYPOINT};
    use io::tests::TestInputOutputHelper;
    use docker::tests::TestContainerHelper;
    use config::{create_config_filename_path, Config, ConfigDocker};
    use command::CommandExitCode;
    use download::tests::TestDownloadHelper;

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

        let result = build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

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

        let result = build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config));

        assert_eq!(result, CommandExitCode::UnknowOption);

        let stderr = io_helper.stderr.borrow();

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

        let result = build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config));

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

        assert_eq!(result, CommandExitCode::Ok);
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

        let result = build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config));

        let stderr = io_helper.stderr.borrow();

        assert_eq!(stderr.get(0).unwrap(), &format!("The file '{}' doesn't exits. Please run 'init' command first.", dockerfile_name));

        assert_eq!(result, CommandExitCode::TemplateNotFound);
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

        let result = build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config));

        let stderr = io_helper.stderr.borrow();

        assert_eq!(stderr.get(0).unwrap(), &format!("The file '{}' doesn't exits. Please run 'init' command first.", entrypoint_name));

        assert_eq!(result, CommandExitCode::TemplateNotFound);
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

        let result = build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config));

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

        assert_eq!(result, CommandExitCode::Ok);
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

        let result = build(&BUILD, &args, io_helper, dck_helper, dl_helper, Some(&config));

        let stderr = io_helper.stderr.borrow();

        assert_eq!(stderr.get(0).unwrap(), "wrong name of closing helper");
        assert_eq!(stderr.get(1).unwrap(), "Something is wrong in Dockerfile template!");

        assert_eq!(result, CommandExitCode::DockerfileTemplateInvalid);
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

        let result = build(&BUILD, &args, io_helper, dck_helper, download_helper, Some(&config));

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

        assert_eq!(result, CommandExitCode::Ok);

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

        let downloads = dl_helper.cmds.borrow();
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

    // TODO These test need more better implementation of folder/file in test.
    // Create a real tree with hook when create, read, update, delete
    //  - test: build test with generate Dockerfile/entry.sh error caused by folder error
    //  - test: build test with delete folder error caused by folder error

    // TODO add switch helper in handlebars to allow install package

    // TODO build check if file is already download
    // TODO build download fail
    // TODO docker build fail
    // TODO only one parameter in command (use struct)
    // TODO build application with Force
    // TODO build many applications
    // TODO build all
    // TODO build missing application
    // TODO build application with skip redownload

    // TODO check if ctrl+c on curl

    // TODO rework error with macro to clear code
    // TODO get_config_application() display error
}
