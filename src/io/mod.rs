///
/// Module to print output.
///
/// Release under MIT License.
///

/// Trait to write one screen.
pub trait OutputWriter {
    /// Print a line with line-feed.
    fn println(&mut self, expr: &str);
}

/// Default print on tty.
pub struct DefaultOutputWriter;

impl OutputWriter for DefaultOutputWriter {
    fn println(&mut self, expr: &str) {
        println!("{}", expr);
    }
}

#[cfg(test)]
pub mod tests {
    use super::OutputWriter;

    /// Use this fonction for test.
    pub struct TestOutputWriter {
        pub stdout: Vec<String>
    }


    impl OutputWriter for TestOutputWriter {
        fn println(&mut self, expr: &str) {
            self.stdout.push(String::from(expr))
        }
    }

    impl TestOutputWriter {
        pub fn new() -> TestOutputWriter {
            TestOutputWriter {
                stdout: Vec::new()
            }
        }
    }
}
