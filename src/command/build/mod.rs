///
/// Module to build application.
///
/// Release under MIT License.
///
use std::path::{Path, PathBuf};
use std::fs::File;
use std::env::temp_dir;
use command::Command;
use command::CommandExitCode;
use super::super::io::InputOutputHelper;
use super::super::docker::ContainerHelper;
use super::super::config::{get_config, Config, create_config_filename_path, get_config_application};
use super::super::config::dockerfile::DOCKERFILE_BASE_FILENAME;
use handlebars::Handlebars;
use rand::Rng;

// TODO add optionnal parameter in config.yml for tmp_dir

///
/// Option for build command.
///
struct BuildOptions {
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
/// Generate template of dockerfile.
///
fn generate_dockerfile(config: &Config, io_helper: &InputOutputHelper, output_filename: String) -> Result<bool, CommandExitCode> {
    let handlebars = Handlebars::new();

    let data = json!({
        "dockerfile_from": config.dockerfile.from.to_owned(),
        "dockerfile_base": true
    });

    match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
        Some(dockerfile_name) => {
            match io_helper.file_read_at_string(&dockerfile_name) {
                Ok(mut source_template) => {
                    match handlebars.render_template(&source_template, &data) {
                        Ok(content) => {
                            match io_helper.file_write(&output_filename, &content) {
                                Ok(_) => Ok(true),
                                Err(_) => {
                                    io_helper.eprintln("Unable to generate Dockerfile for build. Please check right!");
                                    Err(CommandExitCode::CannotGenerateDockerfile)
                                }
                            }
                        },
                        Err(err) => {
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
/// Remove folder.
///
fn remove_temp_dir(io_helper: &InputOutputHelper, tmp_dir: &PathBuf) -> CommandExitCode {
    match io_helper.remove_dir_all(tmp_dir.to_str().unwrap()) {
        Ok(_) => CommandExitCode::Ok,
        Err(_) => CommandExitCode::CannotDeleteTemporaryFolder
    }
}

///
/// Get list of dependencies.
///
fn get_dependencies(io_helper: &InputOutputHelper) -> Result<Vec<String>, CommandExitCode> {
    let config = get_config(io_helper).unwrap();

    // 1 - We have got configuration
    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(mut list_applications_file) => {
            list_applications_file.sort();
            let mut dependencies: Vec<String> = Vec::new();

            // 2 - We have list of application
            for filename in list_applications_file  {
                let application_name = Path::new(&filename)
                    .file_stem()
                    .unwrap()   // get OsStr
                    .to_str()
                    .unwrap();

                    match get_config_application(io_helper, &filename) {
                        Ok(config_application) => {
                            if let Some(d) = config_application.dependencies {
                                dependencies.extend(d.iter().cloned());
                            }
                        },
                        Err(_) => {
                            // Non blocking error
                            io_helper.eprintln(&format!("Cannot read list of dependencies of '{}' application, please check right or file format!", &filename))
                        }
                    };
            };

            Ok(dependencies)
        },
        Err(_) => Err(CommandExitCode::CannotReadApplicationsFolder)
    }
}

///
/// Build base image.
///
fn build_base(io_helper: &InputOutputHelper) -> CommandExitCode {
    // 1 - Create tmp folder for build
    let mut tmp_dir = temp_dir();
    tmp_dir.push(random_string());

    match io_helper.create_dir_all(tmp_dir.to_str().unwrap()) {
        Ok(_) => {
            let config = get_config(io_helper).unwrap();
            let mut dockerfile_name = tmp_dir.to_owned();
            dockerfile_name.push("Dockerfile");

            // 2 - Generate Dockerfile
            match generate_dockerfile(&config, io_helper, dockerfile_name.to_str().unwrap().to_string()) {
                Ok(_) => {
                    // TODO 3 - If force, remove previous image
                    // 4 - Get all dependencies from applications files
                    if let Ok(dependencies) = get_dependencies(io_helper) {
                        // TODO 5 - create an hardlink (std::fs::hard_link) or copy entrypoint
                        // TODO 6 - Build
                    }

                    // 7 - Remove tmp folder
                    remove_temp_dir(io_helper, &tmp_dir)
                },
                Err(err) => err
            }
        },
        Err(_) => {
            io_helper.eprintln(&format!("Cannot create '{}' folder. Please check right!", &tmp_dir.to_str().unwrap()));
            CommandExitCode::CannotCreateFolder
        }
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
    dck_helper: &ContainerHelper) -> CommandExitCode {
    let mut options: BuildOptions = BuildOptions {
        all: false,
        base: false,
        force: false,
        missing: false,
        skip_redownload: false
    };

    for argument in args {
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
            },
        }
    }

    if options.base {
        build_base(io_helper)
    } else {
        CommandExitCode::Todo
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
    use super::BUILD;
    use super::build;
    use super::UNKOWN_OPTIONS_MESSAGE;
    use super::super::super::config::dockerfile::{DOCKERFILE_BASE_FILENAME, ENTRYPOINT};
    use super::super::super::io::tests::TestInputOutputHelper;
    use super::super::super::docker::tests::TestContainerHelper;
    use super::super::super::config::{get_config_filename, create_config_filename_path};
    use command::CommandExitCode;

    #[test]
    fn build_display_help() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [String::from("-h")];

        // Create configuration file
        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\ndockerfile:\n  from: \"tata\"\n  tag: \"tutu\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        let result = build(&BUILD, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::Ok);

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "\n    Usage:	d-sh build [OPTIONS] PROGRAM1 PROGRAM2 ...\n\n    Build an image for a program\n\n    Options:\n      -a, --all                Build all image of program\n      -b, --base               Build base image\n      -f, --force              Remove existing image before build\n      -m, --missing            Build only missing image\n      -s, --skip-redownload    If binary is present, don't check if new version is available");
    }

    #[test]
    fn build_unknow_option() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [String::from("--dghhfhdgfhdgf")];

        // Create configuration file
        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\ndockerfile:\n  from: \"tata\"\n  tag: \"tutu\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        let result = build(&BUILD, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::UnknowOption);

        let stderr = io_helper.stderr.borrow();

        assert_eq!(stderr.get(0).unwrap(), &UNKOWN_OPTIONS_MESSAGE.replace("{}", &args[0]));
    }

    #[test]
    fn build_base_short_option() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [String::from("-b")];

        // Create configuration file
        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\ndockerfile:\n  from: \"tata\"\n  tag: \"tutu\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        // Create dockerfile
        match create_config_filename_path(&DOCKERFILE_BASE_FILENAME) {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("{{dockerfile_from}} {{#if dockerfile_base}}coucou{{/if}}"))
            },
            None => panic!("Unable to create dockerfile for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndependencies:\n  - d3"));

        let result = build(&BUILD, &args, io_helper, dck_helper);

        // Check if temporary folder was created and remove
        let f = io_helper.files_delete.borrow();

        let mut not_found_dockerfile = true;
        let mut not_found_entrypoint = true;

        for filename in f.keys() {
            if filename.ends_with("/Dockerfile") {
                not_found_dockerfile = false;
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

        assert_eq!(base_build.build_options.get(0).unwrap(), "d1 d2 D3");
        assert_eq!(base_build.tag, "d-base-image:v1.0.0");
        assert_eq!(base_build.dockerfile_name, DOCKERFILE_BASE_FILENAME);
        assert_eq!(base_build.base_dir, "dwn");

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building base image...");

        assert_eq!(result, CommandExitCode::Ok);
    }

    // TODO test: build test with generate Dockerfile error cause template bad
    // TODO test: build test with generate Dockerfile/entry.sh error cause folder error
    // TODO test: build test with delete folder error cause folder error
    // TODO test: build base with -f option
    // TODO test: build with application bad file format
}
