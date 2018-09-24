///
/// Module to init config file.
///
/// Release under MIT License.
///
use command::Command;
use super::super::io::InputOutputHelper;
use super::super::config::get_config_filename;
use super::super::docker::ContainerHelper;

/// Default directory of downloading applictions.
const DOWNLOAD_DIR: &str = "~/.d-sh/download";
/// Default directory to store applications.
const APPLICATIONS_DIR: &str = "~/.d-sh/applications";

///
/// Function to implement check D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn init(_command: &Command, _args: &[String], io_helper: &mut InputOutputHelper,
    _dck_helper: &mut ContainerHelper) -> i32 {
    let mut exit_code = 0;

    match get_config_filename() {
        Some(config_file) => {
            if io_helper.file_exits(&config_file) {
                io_helper.eprintln(&format!("The file '{}' exits. Please remove it (or rename) and rerun this command.", config_file));
                exit_code = 3;
            } else {
                io_helper.print(&format!("Enter the path of download directory (default: {}): ", DOWNLOAD_DIR));
                let download_dir = io_helper.read_line();
                let download_dir = download_dir.trim();

                io_helper.print(&format!("Enter the path of applications directory (default: {}): ", APPLICATIONS_DIR));
                let applications_dir = io_helper.read_line();
                let applications_dir = applications_dir.trim();

                let data = format!("---\ndownload_dir: \"{}\"\napplications_dir: \"{}\"\n", download_dir, applications_dir);
                // TODO create folder
                io_helper
                    .file_write(&config_file, &data)
                    .expect(&format!("Unable to write file '{}'", config_file));
            }
        },
        None => {
            io_helper.eprintln("Unable to get your home dir!");
            exit_code = 2;
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
    use super::init;
    use super::INIT;
    use super::super::super::docker::tests::TestContainerHelper;

    #[test]
    fn unable_to_create_configfile_if_exists() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

        let args = [];

        match get_config_filename() {
            Some(cfg_file) => {
                // Create file
                io_helper.files.insert(cfg_file, String::from("toto"))
            },
            None => panic!("Unable to get config filename for test")
        };

        let result = init(&INIT, &args, io_helper, dck_helper);

        assert_eq!(result, 3);
    }

    #[test]
    fn create_configfile_if_exists() {
        let io_helper = &mut TestInputOutputHelper::new();
        let dck_helper = &mut TestContainerHelper::new();

        io_helper.stdin.push(String::from("toto"));
        io_helper.stdin.push(String::from("titi"));

        let args = [];

        let result = init(&INIT, &args, io_helper, dck_helper);

        assert_eq!(result, 0);

        match get_config_filename() {
            Some(cfg_file) => {
                let v = io_helper.files.get(&cfg_file);

                match v {
                    Some(c) => {
                        println!("{}", c);
                        assert_eq!(c, &format!("---\ndownload_dir: \"toto\"\napplications_dir: \"titi\"\n"))
                    },
                    None => panic!("The config file was not created")
                };
            },
            None => panic!("Unable to get config filename for test")
        };
    }
}