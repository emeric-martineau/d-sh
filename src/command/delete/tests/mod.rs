use super::{delete, DELETE};
use command::tests::{test_result_err, test_result_ok};
use command::{CommandExitCode, CommandParameter};
use config::{Config, ConfigDocker};
use docker::tests::TestContainerHelper;
use download::tests::TestDownloadHelper;
///
/// Module to tests module delete.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;

#[test]
fn delete_display_help() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("-h")];

    // Create configuration file
    let config = Config {
        download_dir: String::from("dwn"),
        applications_dir: String::from("app"),
        dockerfile: ConfigDocker {
            from: String::from("tata"),
            tag: String::from("tutu"),
        },
        tmp_dir: None,
    };

    let cmd_param = CommandParameter {
        command: &DELETE,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        config: Some(&config),
    };

    test_result_ok(delete(cmd_param));

    let stdout = io_helper.stdout.borrow();

    assert_eq!(stdout.get(0).unwrap(), "\n    Usage:\td-sh delete APPLICATION\n\n    Delete an image for a application\n\n    Options:\n      -a, --all                Build all image of application\n");
}

#[test]
fn delete_one_application_ok() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("titi")];

    // Create configuration file
    let config = Config {
        download_dir: String::from("dwn"),
        applications_dir: String::from("app"),
        dockerfile: ConfigDocker {
            from: String::from("tata"),
            tag: String::from("tutu"),
        },
        tmp_dir: None,
    };

    // Create application file atom
    io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
    io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
    io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

    // Create list of images returned by docker
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-atom:latest"));
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-titi:latest"));
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-filezilla:latest"));

    let cmd_param = CommandParameter {
        command: &DELETE,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        config: Some(&config),
    };

    test_result_ok(delete(cmd_param));

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
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("titi")];

    // Create configuration file
    let config = Config {
        download_dir: String::from("dwn"),
        applications_dir: String::from("app"),
        dockerfile: ConfigDocker {
            from: String::from("tata"),
            tag: String::from("tutu"),
        },
        tmp_dir: None,
    };

    // Create application file atom
    io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
    io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

    // Create list of images returned by docker
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-atom:latest"));
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-titi:latest"));
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-filezilla:latest"));

    let cmd_param = CommandParameter {
        command: &DELETE,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        config: Some(&config),
    };

    let stderr = test_result_err(delete(cmd_param), CommandExitCode::ApplicationFileNotFound);

    assert_eq!("Not found", stderr.get(0).unwrap());
}

#[test]
fn delete_one_application_all() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("-a")];

    // Create configuration file
    let config = Config {
        download_dir: String::from("dwn"),
        applications_dir: String::from("app"),
        dockerfile: ConfigDocker {
            from: String::from("tata"),
            tag: String::from("tutu"),
        },
        tmp_dir: None,
    };

    // Create application file atom
    io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
    io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
    io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

    // Create list of images returned by docker
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-atom:latest"));
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-titi:latest"));
    dck_helper
        .images
        .borrow_mut()
        .push(String::from("run-filezilla:latest"));

    let cmd_param = CommandParameter {
        command: &DELETE,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        config: Some(&config),
    };

    test_result_ok(delete(cmd_param));

    let dck_images = dck_helper.images.borrow();

    assert_eq!(dck_images.len(), 0);
}
