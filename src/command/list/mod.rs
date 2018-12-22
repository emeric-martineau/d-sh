///
/// Module to list all application avaible.
///
/// Release under MIT License.
///
use std::path::Path;
use command::Command;
use command::CommandExitCode;
use super::super::io::InputOutputHelper;
use super::super::docker::ContainerHelper;
use super::super::config::get_config;

///
/// Function to implement list D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn list(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
    _dck_helper: &ContainerHelper) -> CommandExitCode {

    let config = get_config(io_helper).unwrap();

    // 1 - We have got configuration
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

                io_helper.println(&application_name);
            };

            CommandExitCode::Ok
        },
        Err(_) => CommandExitCode::CannotReadApplicationsFolder
    }
}

///
/// The `list` command.
///
pub const LIST: Command = Command {
    /// This command call by `check`.
    name: "list",
    /// description.
    description: "List all applications available",
    /// Short name.
    short_name: "ls",
    /// `check` command have no parameter.
    min_args: 0,
    max_args: 0,
    /// `check` command have no help.
    usage: "",
    need_config_file: true,
    exec_cmd: list
};

#[cfg(test)]
mod tests {
    use super::super::super::io::tests::TestInputOutputHelper;
    use super::super::super::io::tests::found_item;
    use super::super::super::docker::tests::TestContainerHelper;
    use super::super::super::config::get_config_filename;
    use super::LIST;
    use super::list;
    use command::CommandExitCode;

    #[test]
    fn list_all_applications() {
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

        // Create application file atom
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\"\ncmd_line: \"\""));

        let result = list(&LIST, &args, io_helper, dck_helper);

        assert_eq!(result, CommandExitCode::Ok);

        let stdout = io_helper.stdout.borrow();

        found_item(&stdout, "atom");
        found_item(&stdout, "filezilla");
        found_item(&stdout, "titi");
    }
}
