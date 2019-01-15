///
/// Module to list all application avaible.
///
/// Release under MIT License.
///
use std::path::Path;
use command::Command;
use command::CommandExitCode;
use io::InputOutputHelper;
use docker::ContainerHelper;
use config::Config;
use download::DownloadHelper;

///
/// Function to implement list D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn list(_command: &Command, _args: &[String], io_helper: &InputOutputHelper,
    _dck_helper: &ContainerHelper, _dl_helper: &DownloadHelper,
    config: Option<&Config>) -> CommandExitCode {

    let config = config.unwrap();

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
    use io::tests::TestInputOutputHelper;
    use io::tests::found_item;
    use docker::tests::TestContainerHelper;
    use config::{Config, ConfigDocker};
    use super::{LIST, list};
    use command::CommandExitCode;
    use download::tests::TestDownloadHelper;

    #[test]
    fn list_all_applications() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let dl_helper = &TestDownloadHelper::new(io_helper);

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

        // Create application file atom
        io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
        io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

        let result = list(&LIST, &args, io_helper, dck_helper, dl_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

        let stdout = io_helper.stdout.borrow();

        found_item(&stdout, "atom");
        found_item(&stdout, "filezilla");
        found_item(&stdout, "titi");
    }
}
