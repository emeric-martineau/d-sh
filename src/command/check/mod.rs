///
/// Module to check build container.
///
/// Release under MIT License.
///
use std::path::Path;
use command::Command;
use super::super::io::InputOutputHelper;
use super::super::docker::ContainerHelper;
use super::super::common::get_config;
use super::super::common::get_config_application;

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn check(_command: &Command, _args: &[String], io_helper: &mut InputOutputHelper,
    dck_helper: &mut ContainerHelper) -> i32 {

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
                        0
                    } else {
                        for filename in error_filename {
                             io_helper.eprintln(&format!("The file {} have bad format!", &filename));
                        }

                        9
                    }
                },
                Err(_) => 8
            }
        },
        Err(_) => 7
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
    use super::super::super::docker::tests::TestContainerHelper;
    use super::super::super::common::get_config_filename;
    use super::CHECK;
    use super::check;

    #[test]
    fn check_if_image_found() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

        let args = [];

        // Create list of images returned by docker
        dck_helper.images.push(String::from("run-atom"));
        dck_helper.images.push(String::from("run-gitkraken"));
        dck_helper.images.push(String::from("run-filezilla"));

        // Create configuration file
        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.insert(cfg_file, String::from("---\ndownload_dir: \"dwn\"\napplications_dir: \"app\"\n"))
            },
            None => panic!("Unable to get config filename for test")
        };

        // Create application file atom
        io_helper.files.insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom\""));
        io_helper.files.insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla\""));

        let result = check(&CHECK, &args, io_helper, dck_helper);

        assert_eq!(result, 0);
    }

    #[test]
    fn check_if_image_not_found() {

    }
}
