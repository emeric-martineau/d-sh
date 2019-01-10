///
/// Module to manage process.
///
/// Release under MIT License.
///


pub trait RunCommandHelper {
    /// Run command and return true if success.
    fn run(&self, cmd: &String, args: &Vec<String>) -> bool;
}

/// Default run process
pub struct DefaultRunCommandHelper;

impl RunCommandHelper for DefaultRunCommandHelper {
    fn run(&self, cmd: &String, args: &Vec<String>) -> bool {
        println!("{:?}", "coucou");
        false
    }
}

#[cfg(test)]
pub mod tests {
    use super::RunCommandHelper;
    use std::cell::RefCell;

    /// When run a container
    pub struct TestRunCommand {
        pub cmd: String,
        pub args: Vec<String>
    }

    pub struct TestRunCommandHelper {
        pub cmds:  RefCell<Vec<TestRunCommand>>
    }

    impl RunCommandHelper for TestRunCommandHelper {
        fn run(&self, cmd: &String, args: &Vec<String>) -> bool {
            println!("{:?}", "coucou");
            false
        }
    }

    impl TestRunCommandHelper {
        pub fn new() -> TestRunCommandHelper {
            TestRunCommandHelper {
                cmds: RefCell::new(Vec::new())
            }
        }
    }
}
