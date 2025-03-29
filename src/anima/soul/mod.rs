use crossterm::{
    cursor::{MoveLeft, MoveRight},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::size,
};

use std::{
    fmt::Debug,
    io::{Stdout, stdout},
    process::ExitCode,
    thread::sleep,
    time::Duration,
};

use super::unit::Unit;

///
/// Print a skipped test message to the console
///
/// - `description` The test description
///
pub fn skip_output(description: &str) -> bool {
    if let Ok((x, _)) = size() {
        let mut out = stdout();
        let symbol = '~';
        let status = "skip";
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "{} {}{}{}{}{}\n",
                    symbol.yellow(),
                    description.to_lowercase().dark_grey(),
                    MoveRight(x - 8_u16 - description.len() as u16),
                    "[ ".blue(),
                    status.yellow(),
                    " ]".blue()
                )),
                ResetColor,
            )
            .is_ok()
        );
        true
    } else {
        println!("~ {description}");
        true
    }
}

///
/// Print a success message to the console
///
/// - `description` The test description
///
pub fn success_output(description: &str) -> bool {
    if let Ok((x, _)) = size() {
        let mut out: Stdout = stdout();
        let status: &str = "ok";
        let symbol: char = '*';
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "{} {}{}{}{}{}\n",
                    symbol.green().bold(),
                    description.to_lowercase().white().bold(),
                    MoveRight(x - 8_u16 - description.len() as u16),
                    "[ ".blue().bold(),
                    status.green().bold(),
                    " ]".blue().bold(),
                )),
                ResetColor,
            )
            .is_ok()
        );
        true
    } else {
        println!("* {description}");
        true
    }
}

///
/// Print a title to the console
///
/// - `title` The group title
///
pub fn title_output(title: &str) {
    if let Ok((x, _)) = size() {
        let mut out: Stdout = stdout();
        let symbol: char = '*';
        let status: &str = "::";
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "\n{} {}{}{}{}{}\n\n",
                    symbol.green().bold(),
                    title.to_lowercase().white().bold(),
                    MoveRight(x - 8_u16 - title.len() as u16),
                    "[ ".blue().bold(),
                    status.cyan().bold(),
                    " ]".blue().bold(),
                )),
                ResetColor,
            )
            .is_ok()
        );
    } else {
        println!("{title}");
    }
}

pub fn run_group(title: &str, tests: &mut [Unit]) {
    title_output(title);
    tests.iter_mut().for_each(|t| {
        let _ = t.run();
    });
}
///
/// Close the test suite
///
/// - `success` the failure eq zero
/// - `s` The sussess message
/// - `f` The failure message
///
pub fn results_output(success: bool, s: &str, f: &str) -> ExitCode {
    if let Ok((x, _)) = size() {
        let mut out: Stdout = stdout();
        let status: String = if success {
            "ok".green().to_string()
        } else {
            "ko".red().to_string()
        };
        let symbol: String = if success {
            "*".green().to_string()
        } else {
            "!".red().to_string()
        };
        let description: &str = if success { s } else { f };
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "{} {}{}{}{}{}\n\n",
                    symbol.bold(),
                    description.to_lowercase().white().bold(),
                    MoveRight(x - 8_u16 - description.len() as u16),
                    "[ ".blue().bold(),
                    status.bold(),
                    " ]".blue().bold(),
                )),
                ResetColor,
            )
            .is_ok()
        );
        if success {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    } else if success {
        println!("* {s}");
        ExitCode::SUCCESS
    } else {
        println!("* {f}");
        ExitCode::FAILURE
    }
}

///
/// Print a success ot failure message to the console
///
/// - `description` the test description
/// - `test`   the test result
///
pub fn check(description: &str, test: bool) -> bool {
    sleep(Duration::from_millis(250));
    if test {
        success_output(description)
    } else {
        failure_ouptut(description)
    }
}

///
///
/// Print a failure message
///
/// - `description` The test description
///
pub fn failure_ouptut(description: &str) -> bool {
    if let Ok((x, _)) = size() {
        let mut out: Stdout = stdout();
        let status: &str = "ko";
        let symbol: char = '!';
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "{} {}{}{}{}{}\n",
                    symbol.red(),
                    description.to_lowercase().white(),
                    MoveRight(x - 8_u16 - description.len() as u16),
                    "[ ".blue(),
                    status.red(),
                    " ]".blue()
                )),
                ResetColor,
            )
            .is_ok()
        );
        false
    } else {
        println!("! {description}");
        false
    }
}

/// Represent a testing object
pub trait Testing {
    /// Init a new test suite
    fn new() -> Self;
    ///
    /// Check if all boolean in data are equals to true
    ///
    /// - `description` The test description
    /// - `data` The data to check
    ///
    fn ok(&mut self, description: &str, data: Vec<bool>) -> &mut Self;

    ///    
    /// Check if all boolean in data are equals to false
    ///
    /// - `description` The test description
    /// - `data` The data to check
    ///
    fn ko(&mut self, description: &str, data: Vec<bool>) -> &mut Self;

    ///
    /// Check if data equals to the expected value
    ///
    /// - `description` The test description
    /// - `data` The data to check
    /// - `expected` The expected value
    ///
    fn eq<T: PartialEq>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self;

    ///
    /// Check if data are not equals to the expected value
    ///
    /// - `description` The test description
    /// - `data` The data to check
    /// - `expected` The expected value
    ///
    fn ne<T: PartialEq>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self;

    ///
    /// Map test cases in a group
    ///
    /// - `description` the unit description
    /// - `it` The callback to excecute
    ///
    fn group(&mut self, description: &str, it: fn(&mut Self) -> &mut Self) -> &mut Self;

    ///
    /// Check if a single value is equal to exected
    ///
    /// - `description` The test description
    /// - `value` A value to validate
    /// - `expected` The expected value
    ///
    fn is<T: PartialEq>(&mut self, description: &str, value: T, expected: T) -> &mut Self;

    ///
    /// Check if a single value is true
    ///
    /// - `description` The test description
    /// - `value` A value to validate
    /// - `expected` The expected value
    ///
    fn not<T: PartialEq>(&mut self, description: &str, value: T, expected: T) -> &mut Self;

    ///
    /// Check if length of the data is not equal to expected
    ///
    /// - `description` The test description
    /// - `data` The data to check
    /// - `expected` Expected number of elements
    ///
    fn len<T: PartialEq>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self;

    ///
    /// check if data is full
    ///
    /// - `description` the test description
    /// - `min` the min value
    /// - `max` the max value
    /// - `current` the current value
    ///
    fn full(&mut self, description: &str, min: usize, max: usize, current: usize) -> &mut Self;

    ///
    /// Check if length of the data i greater than to expected
    ///
    /// - `description` The test description
    /// - `data` The data to check
    /// - `expected` Expected number of elements
    ///
    fn gt<T: PartialOrd>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self;

    ///
    /// Check if length of the data is lower than to expected
    ///
    /// - `description` The test description
    /// - `data` The data to check
    /// - `expected` Expected number of elements
    ///
    fn lt<T: PartialOrd>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self;

    ///
    /// Check if length of the data is greater equal than to expected
    ///
    /// - `description` The test description
    /// - `data` The data to check
    /// - `expected` Expected number of elements
    ///
    fn ge<T: PartialOrd>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self;

    ///
    /// check if a string if empty
    ///
    /// - `description` the test description
    /// - `data` the value to check
    ///  
    fn empty(&mut self, description: &str, data: String) -> &mut Self;

    ///
    /// check if data is betwwen minb an max
    ///
    /// - `description` the test description
    /// - `min` the min value
    /// - `max` the max value
    /// - `current` the data to check
    ///
    fn between<T: PartialOrd>(
        &mut self,
        description: &str,
        min: T,
        max: T,
        current: T,
    ) -> &mut Self;

    ///
    /// Check if a function returns an error
    ///
    /// - `description` The test description
    /// - `f` The function that should return a Result::Err
    ///
    fn throws<E: Debug, F: FnOnce() -> Result<(), E>>(
        &mut self,
        description: &str,
        f: F,
    ) -> &mut Self;

    ///
    /// Execute a test and display its duration
    ///
    /// - `description` The test description
    /// - `f` The function to execute, must return a boolean indicating success
    ///
    fn timed<F: FnOnce() -> bool>(&mut self, description: &str, f: F) -> &mut Self;

    ///
    /// Define a sub-group of tests
    ///
    /// - `description` The group title
    /// - `it` The callback that runs the group
    ///
    fn subgroup(&mut self, description: &str, it: fn(&mut Self) -> &mut Self) -> &mut Self;

    ///
    /// - `description` The test description
    ///
    fn skip(&mut self, description: &str) -> &mut Self;
    ///
    /// Check if length of the data is lower equal than to expected
    ///
    /// - `description` The test description
    /// - `data` The data to check
    /// - `expected` Expected number of elements
    ///
    fn le<T: PartialOrd>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self;

    /// Display the results
    fn run(&mut self) -> ExitCode;
}
