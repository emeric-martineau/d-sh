use super::{list, LIST};
use command::tests::test_result_ok;
use command::CommandParameter;
use config::{Config, ConfigDocker};
use docker::tests::TestContainerHelper;
use download::tests::TestDownloadHelper;
use io::tests::found_item;
///
/// Module to tests module list.
///
/// Release under MIT License.
///
use io::tests::TestInputOutputHelper;

#[test]
fn list_all_applications() {
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
            tag: String::from("tutu"),
        },
        tmp_dir: None,
    };

    // Create application file atom
    io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
    io_helper.files.borrow_mut().insert(String::from("app/filezilla.yml"), String::from("---\nimage_name: \"run-filezilla:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));
    io_helper.files.borrow_mut().insert(String::from("app/titi.yml"), String::from("---\nimage_name: \"run-titi:latest\"\ncmd_line: \"\"\ndownload_filename: \"\"\nurl: \"\""));

    let cmd_param = CommandParameter {
        command: &LIST,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        config: Some(&config),
    };

    test_result_ok(list(cmd_param));

    let stdout = io_helper.stdout.borrow();

    found_item(&stdout, "atom");
    found_item(&stdout, "filezilla");
    found_item(&stdout, "titi");
}
