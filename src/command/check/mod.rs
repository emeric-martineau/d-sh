///
/// Module to check build container.
///
/// Release under MIT License.
///
use std::path::Path;
use command::Command;
use command::CommandExitCode;
use super::super::io::InputOutputHelper;
use super::super::docker::ContainerHelper;
use super::super::config::get_config;
use super::super::config::get_config_application;

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn check(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper) -> CommandExitCode {

    match get_config(io_helper) {
        Ok(config) => {
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
        },
        Err(_) => CommandExitCode::CannotReadConfigFile
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
    use super::super::super::io::tests::TestInputOutputHelper;
    use super::super::super::io::tests::found_item;
    use super::super::super::docker::tests::TestContainerHelper;
    use super::super::super::config::get_config_filename;
    use super::CHECK;
    use super::check;
    use command::CommandExitCode;

    #[test]
    fn check_if_image_found_and_not_found() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [];

        // Create list of images returned by docker
        dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-gitkraken:latest"));
        dck_helper.images.borrow_mut().push(String::from("run-filezilla:latest"));

        // Create configuration file
        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        // Create application file atom
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\""));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\""));
        io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\""));

        let result = check(&CHECK, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::Ok);

        let stdout = io_helper.stdout.borrow();

        found_item(&stdout, "atom                              run-atom:latest                   Build done   ");
        found_item(&stdout, "filezilla                         run-filezilla:latest              Build done   ");
        found_item(&stdout, "titi                              run-titi:latest                   Build need   ");
    }

    #[test]
    fn check_if_config_file_not_found() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [];

        let result = check(&CHECK, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::CannotReadConfigFile);
    }

    #[test]
    fn check_if_application_format_has_an_error() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [];

        // Create list of images returned by docker
        dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));

        // Create configuration file
        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        // Create application file atom
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name2: \"run-atom:latest\""));

        let result = check(&CHECK, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::BadApplicationFormat);
    }

    #[test]
    fn check_if_cannot_read_application_dir() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();

        let args = [];

        // Create configuration file
        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.borrow_mut().insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        io_helper.files_error.borrow_mut().insert(String::from("app"), true);

        let result = check(&CHECK, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::CannotReadApplicationsFolder);
    }
}
