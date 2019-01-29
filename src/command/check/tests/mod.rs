///
/// Module to tests module check.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;
use io::tests::found_item;
use docker::tests::TestContainerHelper;
use config::{Config, ConfigDocker};
use super::{CHECK, check};
use command::CommandExitCode;
use download::tests::TestDownloadHelper;

#[test]
fn check_if_image_found_and_not_found() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

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

    let result = check(&CHECK, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::Ok);

    let stdout = io_helper.stdout.borrow();

    found_item(&stdout, "atom                              run-atom:latest                   Build done   ");
    found_item(&stdout, "filezilla                         run-filezilla:latest              Build done   ");
    found_item(&stdout, "titi                              run-titi:latest                   Build need   ");
}

#[test]
fn check_if_application_format_has_an_error() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

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

    let result = check(&CHECK, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::BadApplicationFormat);
}

#[test]
fn check_if_cannot_read_application_dir() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

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

    let result = check(&CHECK, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::CannotReadApplicationsFolder);
}
