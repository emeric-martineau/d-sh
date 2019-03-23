use super::{init, INIT};
use command::tests::{test_result_err, test_result_ok};
use command::{CommandExitCode, CommandParameter};
use config::{create_config_filename_path, get_config_filename};
use docker::tests::TestContainerHelper;
use download::tests::TestDownloadHelper;
///
/// Module to tests module init.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;
use std::collections::HashMap;
use std::path::Path;
use log::{EmptyLoggerHelper};

#[test]
fn unable_to_create_configfile_if_exists() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    let args = [];

    match get_config_filename() {
        Some(cfg_file) => {
            // Create file
            io_helper
                .files
                .borrow_mut()
                .insert(cfg_file, String::from("toto"))
        }
        None => panic!("Unable to get config filename for test"),
    };

    let cmd_param = CommandParameter {
        command: &INIT,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        log_helper: log_helper,
        config: None,
    };

    let stderr = test_result_err(init(cmd_param), CommandExitCode::ConfigFileExits);

    assert_eq!("The file '/home/emeric/.d-sh/config.yml' exits. Please remove it (or rename) and rerun this command.", stderr.get(0).unwrap());
}

#[test]
fn create_configfile_if_not_exists() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    io_helper.stdin.borrow_mut().push(String::from("toto"));
    io_helper.stdin.borrow_mut().push(String::from("titi"));
    io_helper.stdin.borrow_mut().push(String::from("tata"));
    io_helper.stdin.borrow_mut().push(String::from("tutu"));

    let args = [];

    let cmd_param = CommandParameter {
        command: &INIT,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        log_helper: log_helper,
        config: None,
    };

    test_result_ok(init(cmd_param));

    match get_config_filename() {
        Some(cfg_file) => {
            let f = io_helper.files.borrow_mut();
            let v = f.get(&cfg_file);

            match v {
                Some(c) => assert_eq!(c, &format!("---\ndownload_dir: \"toto\"\napplications_dir: \"titi\"\ndockerfile:\n  from: \"tata\"\n  tag: \"tutu\"\n")),
                None => panic!("The config file was not created")
            };
        }
        None => panic!("Unable to get config filename for test"),
    };

    let f = io_helper.files.borrow_mut();

    let dockerfile_list: HashMap<&str, &str> = [
        (super::DOCKERFILE_BASE_FILENAME, super::DOCKERFILE_BASE),
        (super::ENTRYPOINT_FILENAME, super::ENTRYPOINT),
    ]
    .iter()
    .cloned()
    .collect();

    // Create all docker file
    for (filename, content) in &dockerfile_list {
        match create_config_filename_path(filename) {
            Some(dockerfile_name) => {
                let v = f.get(&dockerfile_name);

                match v {
                    Some(c) => assert_eq!(c, content),
                    None => panic!(format!("The dockerfile {} file was not created", filename)),
                };
            }
            None => panic!("Unable to get your home dir!"),
        }
    }
}

#[test]
fn create_configfile_but_cannot_write() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    io_helper.stdin.borrow_mut().push(String::from("toto"));
    io_helper.stdin.borrow_mut().push(String::from("titi"));
    io_helper.stdin.borrow_mut().push(String::from("tata"));
    io_helper.stdin.borrow_mut().push(String::from("tutu"));

    let args = [];

    match get_config_filename() {
        Some(cfg_file) => {
            io_helper.files_error.borrow_mut().insert(cfg_file, true);
        }
        None => panic!("Unable to get config filename for test"),
    };

    let cmd_param = CommandParameter {
        command: &INIT,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        log_helper: log_helper,
        config: None,
    };

    let stderr = test_result_err(init(cmd_param), CommandExitCode::CannotWriteConfigFile);

    assert!(stderr.get(0).unwrap().starts_with("Unable to write file '"));
    assert_eq!("Cannot write", stderr.get(1).unwrap())
}

#[test]
fn create_configfile_but_cannot_create_parent_folder() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);
    let log_helper = &EmptyLoggerHelper{};

    io_helper.stdin.borrow_mut().push(String::from("toto"));
    io_helper.stdin.borrow_mut().push(String::from("titi"));
    io_helper.stdin.borrow_mut().push(String::from("tata"));
    io_helper.stdin.borrow_mut().push(String::from("tutu"));

    let args = [];

    match get_config_filename() {
        Some(cfg_file) => {
            let path = Path::new(&cfg_file);

            if let Some(parent) = path.parent() {
                io_helper
                    .files_error
                    .borrow_mut()
                    .insert(String::from(parent.to_str().unwrap()), true);
            }
        }
        None => panic!("Unable to get config filename for test"),
    };

    let cmd_param = CommandParameter {
        command: &INIT,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        log_helper: log_helper,
        config: None,
    };

    let stderr = test_result_err(
        init(cmd_param),
        CommandExitCode::CannotCreateFolderForConfigFile,
    );

    assert!(stderr.get(0).unwrap().starts_with("Cannot create folder '"));
    assert_eq!("Cannot write", stderr.get(1).unwrap())
}
