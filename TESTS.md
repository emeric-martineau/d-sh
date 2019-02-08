# How write tests ?

Each module have sub-module `tests`.

To do tests more easier, D-SH use Helper :
 - TestInputOutputHelper: to print on screen or in file,
 - TestDownloadHelper: to download file,
 - TestContainerHelper: to run docker command.

## Create my first test

We will create a new command in `src/command/new`:

```
use command::{Command, CommandError, CommandParameter};

#[cfg(test)]
mod tests;

fn new_command(cmd_param: CommandParameter) -> Result<(), CommandError> {
  cmd_param.io_helper.println(&"New command ok!");
  Ok(())
}

pub const NEW_COMMAND: Command = Command {
    name: "new",
    description: "Create new application",
    short_name: "n",
    min_args: 0,
    max_args: 0,
    usage: "Blabalbal",
    need_config_file: true,
    exec_cmd: new_command
};
```

We will create a test in `src/command/tests`:
```
use io::tests::TestInputOutputHelper;
use docker::tests::TestContainerHelper;
use download::tests::TestDownloadHelper;
use config::{Config, ConfigDocker};
use command::tests::test_result_ok;
use command::CommandParameter;
use super::{new_command, NEW_COMMAND};

#[test]
fn run_new() {
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

    let cmd_param = CommandParameter {
        command: &NEW_COMMAND,
        args: &args,
        io_helper: io_helper,
        dck_helper: dck_helper,
        dl_helper: dl_helper,
        config: Some(&config)
    };

    test_result_ok(new_command(cmd_param));

    let stdout = io_helper.stdout.borrow();

    assert_eq!(stdout.get(0).unwrap(), "New command ok!");
}
```
