///
/// Module to print output.
///
/// Release under MIT License.
///
use std::io;
use std::io::Error;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;

/// Trait to write one screen.
pub trait InputOutputHelper {
    /// Print a line with line-feed.
    fn println(&mut self, expr: &str);
    /// Print a string.
    fn print(&mut self, expr: &str);
    /// Print a line with line-feed on stderr.
    fn eprintln(&mut self, expr: &str);
    /// Read a line
    fn read_line(&mut self) -> String;
    /// Write a file.
    fn file_write(&mut self, path: &str, contents: &str) -> Result<(), Error>;
    /// Check if file exits
    fn file_exits(&mut self, filename: &str) -> bool;
}

/// Default print on tty.
pub struct DefaultInputOutputHelper;

impl InputOutputHelper for DefaultInputOutputHelper {
    fn println(&mut self, expr: &str) {
        println!("{}", expr);
    }

    fn print(&mut self, expr: &str) {
        print!("{}", expr);
        io::stdout().flush().unwrap();
    }

    fn eprintln(&mut self, expr: &str) {
        eprintln!("{}", expr);
    }

    fn read_line(&mut self) -> String {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input
            }
            Err(error) => panic!("error: {}", error)
        }
    }

    fn file_write(&mut self, path: &str, contents: &str) -> Result<(), Error> {
        match fs::write(path, contents) {
            Ok(f) => Ok(f),
            Err(e) => Err(e)
        }
    }

    fn file_exits(&mut self, filename: &str) -> bool {
        Path::new(filename).exists()
    }
}

#[cfg(test)]
pub mod tests {
    use super::InputOutputHelper;
    use std::collections::HashMap;
    use std::io::Error;

    /// Use this fonction for test.
    pub struct TestInputOutputHelper {
        pub stdout: Vec<String>,
        pub stderr: Vec<String>,
        pub stdin: Vec<String>,
        pub files: HashMap<String, String>
    }


    impl InputOutputHelper for TestInputOutputHelper {
        fn println(&mut self, expr: &str) {
            self.stdout.push(String::from(expr));
        }

        fn print(&mut self, expr: &str) {
            self.stdout.push(String::from(expr));
        }

        fn eprintln(&mut self, expr: &str) {
            self.stderr.push(String::from(expr));
        }

        fn read_line(&mut self) -> String {
            self.stdin.remove(0)
        }

        fn file_write(&mut self, path: &str, contents: &str) -> Result<(), Error> {
            self.files.insert(String::from(path), String::from(contents));
            Ok(())
        }

        fn file_exits(&mut self, filename: &str) -> bool {
            self.files.contains_key(filename)
        }
    }

    impl TestInputOutputHelper {
        pub fn new() -> TestInputOutputHelper {
            TestInputOutputHelper {
                stdout: Vec::new(),
                stderr: Vec::new(),
                stdin: Vec::new(),
                files: HashMap::new()
            }
        }
    }
}
