///
/// Module to log.
///
/// Release under MIT License.
///
///

pub trait LoggerHelper {
    /// Print simple debug message.
    fn debug(&self, expr: &str);
    /// Print debug message with array.
    fn debug_with_array(&self, expr: &str, args: std::slice::Iter<&str>);
    /// Print debug message with one parameter
    fn debug_with_parameter(&self, expr: &str, args: &str);
    /// Print warn message.
    fn warn(&self, expr: &str);
    /// Print warn message with one parameter
    fn warn_with_parameter(&self, expr: &str, args: &str);
    /// Print err message.
    fn err(&self, expr: &str);
}

/// Default print on tty.
pub struct DefaultLoggerHelper;


impl LoggerHelper for DefaultLoggerHelper {
    fn debug(&self, expr: &str) {
        println!("[DEBUG]: {}", expr);
    }

    fn debug_with_array(&self, expr: &str, args: std::slice::Iter<&str>) {
        let mut debug_args = String::new();

        for x in args {
            debug_args.push_str(" ");
            debug_args.push_str(x);
        }

        let s = expr.replace("{}", &debug_args);

        self.debug(&s);
    }

    fn debug_with_parameter(&self, expr: &str, args: &str) {
        let s = expr.replace("{}",args);

        self.debug(&s);
    }

    fn warn(&self, expr: &str) {
        println!("[WARN]: {}", expr);
    }

    fn warn_with_parameter(&self, expr: &str, args: &str) {
        let s = expr.replace("{}",args);

        self.warn(&s);
    }

    fn err(&self, expr: &str) {
        println!("[ERROR]: {}", expr);
    }
}

/// Default print on tty.
pub struct EmptyLoggerHelper {
}

impl LoggerHelper for EmptyLoggerHelper {
    fn debug(&self, _expr: &str) {
    }

    fn debug_with_array(&self, _expr: &str, _args: std::slice::Iter<&str>) {
    }

    fn debug_with_parameter(&self, _expr: &str, _args: &str) {
    }

    fn warn(&self, _expr: &str) {
    }

    fn warn_with_parameter(&self, _expr: &str, _args: &str) {

    }

    fn err(&self, _expr: &str) {
    }
}