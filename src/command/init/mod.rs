///
/// Module to init config file.
///
/// Release under MIT License.
///
use command::Command;
use command::CommandExitCode;
use std::path::Path;
use std::collections::HashMap;
use super::super::io::InputOutputHelper;
use super::super::config::get_config_filename;
use super::super::config::create_config_filename_path;
use super::super::config::dockerfile::{DOCKERFILE_BASE_FILENAME, DOCKERFILE_BASE,
    DOCKERFILE_DEB_FILENAME, DOCKERFILE_DEB, DOCKERFILE_PACKAGE_FILENAME, DOCKERFILE_PACKAGE,
    DOCKERFILE_TGZ_FILENAME, DOCKERFILE_TGZ, ENTRYPOINT_FILENAME, ENTRYPOINT};
use super::super::docker::ContainerHelper;

/// Default directory of downloading applictions.
const DOWNLOAD_DIR: &str = "~/.d-sh/download";
/// Default directory to store applications.
const APPLICATIONS_DIR: &str = "~/.d-sh/applications";

fn create_dockerfile(io_helper: &InputOutputHelper)  -> CommandExitCode {
    let dockerfile_list: HashMap<&str, &str> = [
        (DOCKERFILE_BASE_FILENAME, DOCKERFILE_BASE),
        (DOCKERFILE_DEB_FILENAME, DOCKERFILE_DEB),
        (DOCKERFILE_PACKAGE_FILENAME, DOCKERFILE_PACKAGE),
        (DOCKERFILE_TGZ_FILENAME, DOCKERFILE_TGZ),
        (ENTRYPOINT_FILENAME, ENTRYPOINT)]
        .iter().cloned().collect();

    // Create all docker file
    for (k, v) in &dockerfile_list {
        match create_config_filename_path(&k) {
            Some(dockerfile_name) => {
                if io_helper.file_write(&dockerfile_name, &v).is_err() {
                    io_helper.eprintln(&format!("Unable to write file '{}'", k));
                    return CommandExitCode::CannotWriteConfigFile;
                }
            },
            None => {
                io_helper.eprintln("Unable to get your home dir!");
                return CommandExitCode::CannotGetHomeFolder;
            }
        }
    }

    CommandExitCode::Ok
}

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn init(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
    _dck_helper: &ContainerHelper) -> CommandExitCode {
    let exit_code;

    match get_config_filename() {
        Some(config_file) => {
            if io_helper.file_exits(&config_file) {
                io_helper.eprintln(&format!("The file '{}' exits. Please remove it (or rename) and rerun this command.", config_file));
                exit_code = CommandExitCode::ConfigFileExits;
            } else {
                io_helper.print(&format!("Enter the path of download directory (default: {}): ", DOWNLOAD_DIR));
                let download_dir = io_helper.read_line();
                let mut download_dir = download_dir.trim();

                io_helper.print(&format!("Enter the path of applications directory (default: {}): ", APPLICATIONS_DIR));
                let applications_dir = io_helper.read_line();
                let mut applications_dir = applications_dir.trim();

                // Default value
                if download_dir.is_empty() {
                    download_dir = DOWNLOAD_DIR;
                }

                if applications_dir.is_empty() {
                    applications_dir = APPLICATIONS_DIR;
                }

                let data = format!("---\ndownload_dir: \"{}\"\napplications_dir: \"{}\"\n", download_dir, applications_dir);

                // Create folder
                let path = Path::new(&config_file);

                if let Some(parent) = path.parent() {
                    // No parent ? Not a error.
                    if io_helper.create_dir_all(parent.to_str().unwrap()).is_err() {
                        io_helper.eprintln(&format!("Cannot create folder '{}'!", parent.display()));
                        return CommandExitCode::CannotCreateFolderForConfigFile;
                    }
                }

                if io_helper.file_write(&config_file, &data).is_err() {
                    io_helper.eprintln(&format!("Unable to write file '{}'", config_file));
                    exit_code = CommandExitCode::CannotWriteConfigFile;
                } else {
                    exit_code = create_dockerfile(io_helper);
                }
            }
        },
        None => {
            io_helper.eprintln("Unable to get your home dir!");
            exit_code = CommandExitCode::CannotGetHomeFolder;
        }
    }

    exit_code
}

///
/// The `check` command.
///
pub const INIT: Command = Command {
    /// This command call by `check`.
    name: "init",
    /// description.
    description: "Initialize config file if not exists",
    /// Short name.
    short_name: "it",
    /// `check` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `check` command have no help.
    usage: "",
    need_config_file: false,
    exec_cmd: init
};

#[cfg(test)]
mod tests {
    use super::super::super::io::tests::TestInputOutputHelper;
    use super::get_config_filename;
    use super::create_config_filename_path;
    use super::init;
    use super::INIT;
    use super::super::super::docker::tests::TestContainerHelper;
    use super::Path;
    use super::HashMap;
    use command::CommandExitCode;

    #[test]
    fn unable_to_create_configfile_if_exists() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [];

        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("toto"))
            },
            None => panic!("Unable to get config filename for test")
        };

        let result = init(&INIT, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::ConfigFileExits);
    }

    #[test]
    fn create_configfile_if_not_exists() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        io_helper.stdin.borrow_mut().push(String::from("toto"));
        io_helper.stdin.borrow_mut().push(String::from("titi"));

        let args = [];

        let result = init(&INIT, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::Ok);

        match get_config_filename() {
            Some(cfg_file) => {
                let f = io_helper.files.borrow_mut();
                let v = f.get(&cfg_file);

                match v {
                    Some(c) => assert_eq!(c, &format!("---\ndownload_dir: \"toto\"\napplications_dir: \"titi\"\n")),
                    None => panic!("The config file was not created")
                };
            },
            None => panic!("Unable to get config filename for test")
        };

        let f = io_helper.files.borrow_mut();

        let dockerfile_list: HashMap<&str, &str> = [
            (super::DOCKERFILE_BASE_FILENAME, super::DOCKERFILE_BASE),
            (super::DOCKERFILE_DEB_FILENAME, super::DOCKERFILE_DEB),
            (super::DOCKERFILE_PACKAGE_FILENAME, super::DOCKERFILE_PACKAGE),
            (super::DOCKERFILE_TGZ_FILENAME, super::DOCKERFILE_TGZ),
            (super::ENTRYPOINT_FILENAME, super::ENTRYPOINT)]
            .iter().cloned().collect();

        // Create all docker file
        for (filename, content) in &dockerfile_list {
            match create_config_filename_path(filename) {
                Some(dockerfile_name) => {
                    let v = f.get(&dockerfile_name);

                    match v {
                        Some(c) => assert_eq!(c, content),
                        None => panic!(format!("The dockerfile {} file was not created", filename))
                    };

                },
                None => panic!("Unable to get your home dir!")
            }
        }
    }

    #[test]
    fn create_configfile_but_cannot_write() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        io_helper.stdin.borrow_mut().push(String::from("toto"));
        io_helper.stdin.borrow_mut().push(String::from("titi"));

        let args = [];

        match get_config_filename() {
            Some(cfg_file) => {
                io_helper.files_error.borrow_mut().insert(cfg_file, true);
            },
            None => panic!("Unable to get config filename for test")
        };

        let result = init(&INIT, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::CannotWriteConfigFile);
    }

    #[test]
    fn create_configfile_but_cannot_create_parent_folder() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        io_helper.stdin.borrow_mut().push(String::from("toto"));
        io_helper.stdin.borrow_mut().push(String::from("titi"));

        let args = [];

        match get_config_filename() {
            Some(cfg_file) => {
                let path = Path::new(&cfg_file);

                if let Some(parent) = path.parent() {
                    io_helper.files_error.borrow_mut().insert(String::from(parent.to_str().unwrap()), true);
                }
            },
            None => panic!("Unable to get config filename for test")
        };

        let result = init(&INIT, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::CannotCreateFolderForConfigFile);
    }
}
