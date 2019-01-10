///
/// Module to check build container.
///
/// Release under MIT License.
///
use std::path::Path;
use command::Command;
use command::CommandExitCode;
use io::InputOutputHelper;
use docker::ContainerHelper;
use config::{get_config_application, Config};
use process::RunCommandHelper;

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn check(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, _run_command_helper: &RunCommandHelper,
    config: Option<&Config>) -> CommandExitCode {

    let config = config.unwrap();

    // 1 - We have got configuration
    match io_helper.dir_list_file(&config.applications_dir, "*.yml") {
        Ok(list_applications_file) => {
            let mut error_filename: Vec<String> = Vec::new();

            // 2 - We have list of application
            for filename in list_applications_file  {
                // 3 - Now, we check if image exits
                match get_config_application(io_helper, &filename) {
                    Ok(config_application) => {
                        let status;
                        let images = dck_helper.list_image(&config_application.image_name);

                        if images.len() > 0 {
                            status = "Build done"
                        } else {
                            status = "Build need";
                        }

                        let application_name = Path::new(&filename)
                            .file_stem()
                            .unwrap()   // get OsStr
                            .to_str()
                            .unwrap();

                        io_helper.println(&format!(
                            "{:<with_first$}{:<with_first$}{:<width_second$}",
                            application_name,
                            &config_application.image_name,
                            &status,
                            with_first = 34,
                            width_second = 13));
                    },
                    Err(_) => error_filename.push(filename)
                };
            };

            if error_filename.len() == 0 {
                CommandExitCode::Ok
            } else {
                for filename in error_filename {
                     io_helper.eprintln(&format!("The file {} have bad format!", &filename));
                }

                CommandExitCode::BadApplicationFormat
            }
        },
        Err(_) => CommandExitCode::CannotReadApplicationsFolder
    }

}

///
/// The `check` command.
///
pub const CHECK: Command = Command {
    /// This command call by `check`.
    name: "check",
    /// description.
    description: "List missing container image",
    /// Short name.
    short_name: "chk",
    /// `check` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `check` command have no help.
    usage: "",
    need_config_file: true,
    exec_cmd: check
};

#[cfg(test)]
mod tests {
    use io::tests::TestInputOutputHelper;
    use io::tests::found_item;
    use docker::tests::TestContainerHelper;
    use config::{Config, ConfigDocker};
    use super::{CHECK, check};
    use command::CommandExitCode;
    use process::tests::TestRunCommandHelper;

    #[test]
    fn check_if_image_found_and_not_found() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let args = [];

        // Create list of images returned by docker
        dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-gitkraken:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-filezilla:latest"));

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

        let result = check(&CHECK, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

        let stdout = io_helper.stdout.borrow();

        found_item(&stdout, "atom                              run-atom:latest                   Build done   ");
        found_item(&stdout, "filezilla                         run-filezilla:latest              Build done   ");
        found_item(&stdout, "titi                              run-titi:latest                   Build need   ");
    }

    #[test]
    fn check_if_application_format_has_an_error() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let args = [];

        // Create list of images returned by docker
        dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));

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
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name2: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

        let result = check(&CHECK, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::BadApplicationFormat);
    }

    #[test]
    fn check_if_cannot_read_application_dir() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

        let args = [];

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

        io_helper.files_error.borrow_mut().insert(String::from("app"), true);

        let result = check(&CHECK, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::CannotReadApplicationsFolder);
    }
}
