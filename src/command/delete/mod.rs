///
/// Module to delete image.
///
/// Release under MIT License.
///
use std::path::Path;
use command::Command;
use command::CommandExitCode;
use io::InputOutputHelper;
use docker::ContainerHelper;
use config::{Config, get_config_application};
use process::RunCommandHelper;

///
/// Function to delete one image.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn delete_one(config: &Config, app: &str, io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper)  -> CommandExitCode {

    let mut application_filename = String::from(app);
    application_filename.push_str(".yml");

    let application_filename_path = Path::new(&config.applications_dir)
        .join(&application_filename);

    let application_filename_full_path = application_filename_path
        .to_str()
        .unwrap();

    match get_config_application(io_helper, &application_filename_full_path) {
        Ok(config_application) => {
            if dck_helper.remove_image(&config_application.image_name) {
                CommandExitCode::Ok
            } else {
                CommandExitCode::ContainerImageNotFound
            }
        },
        Err(_) => CommandExitCode::ApplicationFileNotFound
    }
}

///
/// Function to delete all images.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn delete_all(config: &Config, io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper)  -> CommandExitCode {
    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(mut list_applications_file) => {
            list_applications_file.sort();

            // 2 - We have list of application
            for filename in list_applications_file  {
                let application_name = Path::new(&filename)
                    .file_stem()
                    .unwrap()   // get OsStr
                    .to_str()
                    .unwrap();

                delete_one(&config, &application_name, io_helper, dck_helper);
            };

            CommandExitCode::Ok
        },
        Err(_) => CommandExitCode::CannotReadApplicationsFolder
    }
}

///
/// Function to implement delete D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn delete(command: &Command, args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, _run_command_helper: &RunCommandHelper,
    config: Option<&Config>) -> CommandExitCode {

    let config = config.unwrap();

    match args[0].as_ref() {
        "-h" | "--help" => {
            io_helper.println(command.usage);
            CommandExitCode::Ok
        },
        "-a" | "--all" => {
            delete_all(&config, io_helper, dck_helper)
        },
        app => {
            delete_one(&config, app, io_helper, dck_helper)
        }
    }
}

///
/// The `delete` command.
///
pub const DELETE: Command = Command {
    /// This command call by `check`.
    name: "delete",
    /// description.
    description: "Delete image",
    /// Short name.
    short_name: "rm",
    /// `check` command have no parameter.
    min_args: 1,
    max_args: 1,
    /// `check` command have no help.
    usage: "
    Usage:	d-sh delete APPLICATION

    Delete an image for a application

    Options:
      -a, --all                Build all image of application
",
    need_config_file: true,
    exec_cmd: delete
};

#[cfg(test)]
mod tests {
    use io::tests::TestInputOutputHelper;
    use docker::tests::TestContainerHelper;
    use config::{Config, ConfigDocker};
    use super::{DELETE, delete};
    use command::CommandExitCode;
    use process::tests::TestRunCommandHelper;

    #[test]
    fn delete_display_help() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

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

        let result = delete(&DELETE, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "\n    Usage:\td-sh delete APPLICATION\n\n    Delete an image for a application\n\n    Options:\n      -a, --all                Build all image of application\n");
    }

    #[test]
    fn delete_one_application_ok() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let args = [String::from("titi")];

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

        // Create application file atom
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

        // Create list of images returned by docker
        dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-titi:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-filezilla:latest"));

        let result = delete(&DELETE, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

        let dck_images = dck_helper.images.borrow();

        let list_image: Vec<String> = dck_images
            .iter()
            .filter(|i| *i == "run-titi:latest")
            .map(|l| l.to_string())
            .collect();

        assert_eq!(list_image.len(), 0);
    }

    #[test]
    fn delete_one_application_ko() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let args = [String::from("titi")];

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

        // Create application file atom
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

        // Create list of images returned by docker
        dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-titi:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-filezilla:latest"));

        let result = delete(&DELETE, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::ApplicationFileNotFound);
    }

    #[test]
    fn delete_one_application_all() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let args = [String::from("-a")];

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

        // Create application file atom
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

        // Create list of images returned by docker
        dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-titi:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-filezilla:latest"));

        let result = delete(&DELETE, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

        let dck_images = dck_helper.images.borrow();

        assert_eq!(dck_images.len(), 0);
    }
}
