///
/// Module to build application.
///
/// Release under MIT License.
///
use std::path::Path;
use command::Command;
use command::CommandExitCode;
use super::super::io::InputOutputHelper;
use super::super::docker::ContainerHelper;
use super::super::config::get_config;
use super::super::config::dockerfile::{DOCKERFILE_BASE_FILENAME, DOCKERFILE_DEB_FILENAME};

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
/// Build base image
///
fn build_base() -> CommandExitCode {
    // 1 - Get base image name from dockerfile debian
    // 2 - If force, remove previous image
    // 3 - Get all dependencies from applications files
    // 4 - Build
    CommandExitCode::Todo
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
        build_base()
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
    min_args: 0,
    max_args: 0,
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
    use super::{DOCKERFILE_BASE_FILENAME, DOCKERFILE_DEB_FILENAME};
    use super::super::super::io::tests::TestInputOutputHelper;
    use super::super::super::docker::tests::TestContainerHelper;
    use super::super::super::config::get_config_filename;
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
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\n"))
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
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\n"))
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
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        // Add application with dependencies
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndependencies:\n  - d1\n  - d2"));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndependencies:\n  - d3"));

        let result = build(&BUILD, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::Ok);

        let builds = dck_helper.builds.borrow();
        let base_build = builds.get(0).unwrap();

        assert_eq!(base_build.build_options.get(0).unwrap(), "d1 d2 D3");
        assert_eq!(base_build.tag, "d-base-image:v1.0.0");
        assert_eq!(base_build.dockerfile_name, DOCKERFILE_BASE_FILENAME);
        assert_eq!(base_build.base_dir, "dwn");

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "Building base image...");
    }
}
