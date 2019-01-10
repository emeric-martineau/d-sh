///
/// Module to delete image.
///
/// Release under MIT License.
///
use std::path::Path;
use users::{get_current_uid, get_current_gid, get_current_username};
use command::Command;
use command::CommandExitCode;
use io::{InputOutputHelper, convert_path};
use docker::ContainerHelper;
use config::{Config, ConfigApplication, get_config_application};
use process::RunCommandHelper;

///
/// Construct extra args of run
///
/// `interactive` true if command line interactive
/// `config_application` configuration of current application
///
/// returning vector of string
///
fn get_extra_args(interactive: bool, config_application: &ConfigApplication) -> Vec<String> {
    let mut extra_args: Vec<String> = vec![];

    if interactive || config_application.interactive.unwrap_or(false) {
        extra_args.push(String::from("-it"));
    } else {
        extra_args.push(String::from("-d"));
    }

    if config_application.ipc_host.unwrap_or(false) {
        extra_args.push(String::from("--ipc=host"));
    }

    extra_args
}

///
/// Construct run args of run
///
/// `extra_args` extra arguments
/// `username` username
///
/// returning vector of string
///
fn get_run_args(extra_args: &mut Vec<String>, username: String) -> Vec<String> {
    let mut run_opts: Vec<String> = vec![
        String::from("-v"),
        String::from("/tmp/.X11-unix/:/tmp/.X11-unix/"),
        String::from("-v"),
        String::from("/dev/shm:/dev/shm"),
        String::from("-v"),
        format!("{}:/home/{}", convert_path("~/"), username),
        String::from("-e"),
        String::from("DISPLAY"),
        String::from("-e"),
        format!("USERNAME_TO_RUN={}", username),
        String::from("-e"),
        format!("USERNAME_TO_RUN_GID={}", get_current_gid()),
        String::from("-e"),
        format!("USERNAME_TO_RUN_UID={}", get_current_uid()),
        String::from("--rm")];

    run_opts.append(extra_args);

    run_opts
}

///
/// Convert args to cmd_args
///
/// `args` command line args
///
/// returning vector of string
///
fn get_cmd_args(cmd_line_args: &Option<Vec<String>>, args: &[String]) -> Vec<String> {
    let mut cmd_args: Vec<String> = Vec::new();

    if cmd_line_args.is_some() {
        let cla = &cmd_line_args.as_ref().unwrap();

        for arg in *cla {
            cmd_args.push(arg.to_owned());
        }
    }

    for arg in args {
        cmd_args.push(arg.to_owned());
    }

    cmd_args
}

///
/// Function to run one image.
///
/// `app` name of application name.
///
/// returning exit code of D-SH.
///
fn run_application(config: &Config, app: &str, io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, args: &[String], interactive: bool)  -> CommandExitCode {

    io_helper.println(&format!("Running {}...", app));

    let mut application_filename = String::from(app);
    application_filename.push_str(".yml");

    let application_filename_path = Path::new(&config.applications_dir)
        .join(&application_filename);

    let application_filename_full_path = application_filename_path
        .to_str()
        .unwrap();

    match get_config_application(io_helper, &application_filename_full_path) {
        Ok(config_application) => {
            // Check if image exists
            let images = dck_helper.list_image(&config_application.image_name);

            if images.len() > 0 {
                io_helper.println("Create container");

                match get_current_username() {
                    Some(username) => {
                        let mut extra_args = get_extra_args(interactive, &config_application);
                        let run_opts = get_run_args(&mut extra_args, username);

                        let cmd_args = get_cmd_args(&config_application.cmd_line_args, args);

                        if dck_helper.run_container(&config_application.image_name, Some(&run_opts),
                            Some(&config_application.cmd_line), Some(&cmd_args)) {
                            CommandExitCode::Ok
                        } else {
                            CommandExitCode::ContainerRunError
                        }
                    },
                    None => {
                        io_helper.eprintln("Cannot get current user !");
                        CommandExitCode::CannotGetCurrentUser
                    }
                }
            } else {
                io_helper.eprintln(&format!("Image for program {} not found.", app));
                io_helper.eprintln("");
                io_helper.eprintln("Build it before with:");
                io_helper.eprintln(&format!("  d-sh build {}", app));

                CommandExitCode::ContainerImageNotFound
            }
        },
        Err(_) => {
            io_helper.eprintln(&format!("Application '{}' not found.", app));
            CommandExitCode::ApplicationFileNotFound
        }
    }
}

///
/// Function to implement delete D-SH command.
///
/// `args` parameter is command line arguments of D-SH.
///
/// returning exit code of D-SH.
///
fn run(command: &Command, args: &[String], io_helper: &InputOutputHelper,
    dck_helper: &ContainerHelper, _run_command_helper: &RunCommandHelper,
    config: Option<&Config>) -> CommandExitCode {

    let config = config.unwrap();

    match args[0].as_ref() {
        "-h" | "--help" => {
            io_helper.println(command.usage);
            CommandExitCode::Ok
        },
        "-i" | "--interactive" => {
            if args.len() > 1 {
                run_application(&config, &args[1], io_helper, dck_helper, &args[2..], true)
            } else {
                io_helper.eprintln("You must specify an application !");

                CommandExitCode::ApplicationNameMissing
            }
        },
        app => {
            run_application(&config, &app, io_helper, dck_helper, &args[1..], false)
        }
    }
}

///
/// The `run` command.
///
pub const RUN: Command = Command {
    /// This command call by `check`.
    name: "run",
    /// description.
    description: "Run container",
    /// Short name.
    short_name: "",
    /// `check` command have no parameter.
    min_args: 1,
    max_args: 2,
    /// `check` command have no help.
    usage: "
    Usage:	d-sh run [-i | --interactive] APPLICATION [APPLICATION ARGS]

    Run an application

    Options:
      -i | --interactive       Run application in terminal
",
    need_config_file: true,
    exec_cmd: run
};

#[cfg(test)]
mod tests {
    use io::convert_path;
    use io::tests::TestInputOutputHelper;
    use docker::tests::TestContainerHelper;
    use docker::tests::TestRunContainer;
    use config::{Config, ConfigDocker};
    use io::tests::found_item;
    use super::{RUN, run};
    use command::CommandExitCode;
    use users::{get_current_uid, get_current_gid, get_current_username};
    use process::tests::TestRunCommandHelper;

    #[test]
    fn run_display_help() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

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

        let result = run(&RUN, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

        let stdout = io_helper.stdout.borrow();

        assert_eq!(stdout.get(0).unwrap(), "\n    Usage:\td-sh run [-i | --interactive] APPLICATION [APPLICATION ARGS]\n\n    Run an application\n\n    Options:\n      -i | --interactive       Run application in terminal\n");
    }

    #[test]
    fn run_image_application_not_found() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

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

        let result = run(&RUN, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::ApplicationFileNotFound);

        let stdout = io_helper.stdout.borrow();

        found_item(&stdout, "Running atom...");

        let stderr = io_helper.stderr.borrow();

        found_item(&stderr, "Application 'atom' not found.");
    }

    #[test]
    fn run_image_not_found_not_interactive() {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

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

        let result = run(&RUN, &args, io_helper, dck_helper, run_command_helper, Some(&config));

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
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

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

        let result = run(&RUN, &args, io_helper, dck_helper, run_command_helper, Some(&config));

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
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

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

        let result = run(&RUN, &args, io_helper, dck_helper, run_command_helper, Some(&config));

        assert_eq!(result, CommandExitCode::Ok);

        let containers = dck_helper.containers.borrow();
        let atom_container = containers.get(0).unwrap();

        assert_eq!(atom_container.cmd_options.len(), 2);

        assert_eq!(atom_container.cmd_options.get(0).unwrap(), "arg1");
        assert_eq!(atom_container.cmd_options.get(1).unwrap(), "arg2");
    }

    fn run_image_found_interactive(opt: &str, application_config_content: &str, args_len: usize) -> TestRunContainer {
        let io_helper = &TestInputOutputHelper::new();
        let dck_helper = &TestContainerHelper::new();
        let run_command_helper = &TestRunCommandHelper::new();

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

        let result = run(&RUN, &args, io_helper, dck_helper, run_command_helper, Some(&config));

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
}
