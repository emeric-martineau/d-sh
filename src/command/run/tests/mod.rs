///
/// Module to tests module run.
///
/// Release under MIT License.
///
use io::convert_path;
use io::tests::TestInputOutputHelper;
use docker::tests::TestContainerHelper;
use docker::tests::TestRunContainer;
use config::{Config, ConfigDocker};
use io::tests::found_item;
use super::{RUN, run};
use command::CommandExitCode;
use users::{get_current_uid, get_current_gid, get_current_username};
use download::tests::TestDownloadHelper;

#[test]
fn run_display_help() {
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
            tag: String::from("tutu")
        },
        tmp_dir: None
    };

    let result = run(&RUN, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::Ok);

    let stdout = io_helper.stdout.borrow();

    assert_eq!(stdout.get(0).unwrap(), "\n    Usage:\td-sh run [-i | --interactive] APPLICATION [APPLICATION ARGS]\n\n    Run an application\n\n    Options:\n      -i | --interactive       Run application in terminal\n");
}

#[test]
fn run_image_application_not_found() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("atom")];

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

    let result = run(&RUN, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::ApplicationFileNotFound);

    let stdout = io_helper.stdout.borrow();

    found_item(&stdout, "Running atom...");

    let stderr = io_helper.stderr.borrow();

    found_item(&stderr, "Application 'atom' not found.");
}

#[test]
fn run_image_not_found_not_interactive() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("atom")];

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

    let result = run(&RUN, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::ContainerImageNotFound);

    let stdout = io_helper.stdout.borrow();

    found_item(&stdout, "Running atom...");

    let stderr = io_helper.stderr.borrow();

    found_item(&stderr, "Image for program atom not found.");
    found_item(&stderr, "Build it before with:");
    found_item(&stderr, "  d-sh build atom");
}

#[test]
fn run_image_found_not_interactive() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("atom")];

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
    io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"/usr/bin/atom -f\"\ndownload_filename: \"\"\nurl: \"\""));

    // Create list of images returned by docker
    dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));

    let result = run(&RUN, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::Ok);

    let containers = dck_helper.containers.borrow();
    let atom_container = containers.get(0).unwrap();

    assert_eq!(atom_container.image_name, "run-atom:latest");
    assert_eq!(atom_container.cmd, "/usr/bin/atom -f");

    assert_eq!(atom_container.run_options.len(), 16);

    let username = get_current_username().unwrap();

    assert_eq!(atom_container.run_options.get(0).unwrap(), "-v");
    assert_eq!(atom_container.run_options.get(1).unwrap(), "/tmp/.X11-unix/:/tmp/.X11-unix/");
    assert_eq!(atom_container.run_options.get(2).unwrap(), "-v");
    assert_eq!(atom_container.run_options.get(3).unwrap(), "/dev/shm:/dev/shm");
    assert_eq!(atom_container.run_options.get(4).unwrap(), "-v");
    assert_eq!(atom_container.run_options.get(5).unwrap(), &format!("{}:/home/{}", convert_path("~/"), username));
    assert_eq!(atom_container.run_options.get(6).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(7).unwrap(), "DISPLAY");
    assert_eq!(atom_container.run_options.get(8).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(9).unwrap(), &format!("USERNAME_TO_RUN={}", username));
    assert_eq!(atom_container.run_options.get(10).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(11).unwrap(), &format!("USERNAME_TO_RUN_GID={}", get_current_gid()));
    assert_eq!(atom_container.run_options.get(12).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(13).unwrap(), &format!("USERNAME_TO_RUN_UID={}", get_current_uid()));
    assert_eq!(atom_container.run_options.get(14).unwrap(), "--rm");
    assert_eq!(atom_container.run_options.get(15).unwrap(), "-d");

    let stdout = io_helper.stdout.borrow();

    found_item(&stdout, "Running atom...");
    found_item(&stdout, "Create container");
}

#[test]
fn run_image_found_not_interactive_with_args() {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args = [String::from("atom"), String::from("arg1"), String::from("arg2")];

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
    io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from("---\nimage_name: \"run-atom:latest\"\ncmd_line: \"/usr/bin/atom -f\"\ndownload_filename: \"\"\nurl: \"\""));

    // Create list of images returned by docker
    dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));

    let result = run(&RUN, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::Ok);

    let containers = dck_helper.containers.borrow();
    let atom_container = containers.get(0).unwrap();

    assert_eq!(atom_container.cmd_options.len(), 2);

    assert_eq!(atom_container.cmd_options.get(0).unwrap(), "arg1");
    assert_eq!(atom_container.cmd_options.get(1).unwrap(), "arg2");
}

fn run_image_found_interactive(opt: &str, application_config_content: &str, args_len: usize) -> TestRunContainer {
    let io_helper: &TestInputOutputHelper = &TestInputOutputHelper::new();
    let dck_helper: &TestContainerHelper = &TestContainerHelper::new();
    let dl_helper: &TestDownloadHelper = &TestDownloadHelper::new(io_helper);

    let args;

    if opt.is_empty() {
        args = vec![String::from("atom")];
    } else {
        args = vec![String::from(opt), String::from("atom")];
    }

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
    io_helper.files.borrow_mut().insert(String::from("app/atom.yml"), String::from(application_config_content));

    // Create list of images returned by docker
    dck_helper.images.borrow_mut().push(String::from("run-atom:latest"));

    let result = run(&RUN, &args, io_helper, dck_helper, dl_helper, Some(&config));

    assert_eq!(result, CommandExitCode::Ok);

    let containers = dck_helper.containers.borrow();
    let atom_container = containers.get(0).unwrap();

    assert_eq!(atom_container.image_name, "run-atom:latest");
    assert_eq!(atom_container.cmd, "/usr/bin/atom -f");

    assert_eq!(atom_container.run_options.len(), 16 + args_len);

    let username = get_current_username().unwrap();

    assert_eq!(atom_container.run_options.get(0).unwrap(), "-v");
    assert_eq!(atom_container.run_options.get(1).unwrap(), "/tmp/.X11-unix/:/tmp/.X11-unix/");
    assert_eq!(atom_container.run_options.get(2).unwrap(), "-v");
    assert_eq!(atom_container.run_options.get(3).unwrap(), "/dev/shm:/dev/shm");
    assert_eq!(atom_container.run_options.get(4).unwrap(), "-v");
    assert_eq!(atom_container.run_options.get(5).unwrap(), &format!("{}:/home/{}", convert_path("~/"), username));
    assert_eq!(atom_container.run_options.get(6).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(7).unwrap(), "DISPLAY");
    assert_eq!(atom_container.run_options.get(8).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(9).unwrap(), &format!("USERNAME_TO_RUN={}", username));
    assert_eq!(atom_container.run_options.get(10).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(11).unwrap(), &format!("USERNAME_TO_RUN_GID={}", get_current_gid()));
    assert_eq!(atom_container.run_options.get(12).unwrap(), "-e");
    assert_eq!(atom_container.run_options.get(13).unwrap(), &format!("USERNAME_TO_RUN_UID={}", get_current_uid()));
    assert_eq!(atom_container.run_options.get(14).unwrap(), "--rm");
    assert_eq!(atom_container.run_options.get(15).unwrap(), "-it");

    let stdout = io_helper.stdout.borrow();

    found_item(&stdout, "Running atom...");
    found_item(&stdout, "Create container");

    atom_container.clone()
}

#[test]
fn run_image_found_interactive_by_command_line_short_opts() {
    run_image_found_interactive("-i", "---\nimage_name: \"run-atom:latest\"\ncmd_line: \"/usr/bin/atom -f\"\ndownload_filename: \"\"\nurl: \"\"", 0);
}

#[test]
fn run_image_found_interactive_by_command_line_long_opts() {
    run_image_found_interactive("--interactive", "---\nimage_name: \"run-atom:latest\"\ncmd_line: \"/usr/bin/atom -f\"\ndownload_filename: \"\"\nurl: \"\"", 0);
}

#[test]
fn run_image_found_interactive_by_config() {
    run_image_found_interactive("", "---\nimage_name: \"run-atom:latest\"\ncmd_line: \"/usr/bin/atom -f\"\ninteractive: true\ndownload_filename: \"\"\nurl: \"\"", 0);
}

#[test]
fn run_image_found_interactive_by_config_with_ipc() {
    let atom_container = run_image_found_interactive("",
        "---\nimage_name: \"run-atom:latest\"\ncmd_line: \"/usr/bin/atom -f\"\ninteractive: true\nipc_host: true\ndownload_filename: \"\"\nurl: \"\"", 1);

    assert_eq!(atom_container.run_options.get(16).unwrap(), "--ipc=host");
}

#[test]
fn run_image_found_interactive_by_config_and_with_argument() {
    let atom_container = run_image_found_interactive("",
        "---\nimage_name: \"run-atom:latest\"\ncmd_line: \"/usr/bin/atom -f\"\ninteractive: true\ncmd_line_args:\n - truc\n - bidule\ndownload_filename: \"\"\nurl: \"\"", 0);

    assert_eq!(atom_container.cmd_options.len(), 2);
    assert_eq!(atom_container.cmd_options.get(0).unwrap(), "truc");
    assert_eq!(atom_container.cmd_options.get(1).unwrap(), "bidule");
}
