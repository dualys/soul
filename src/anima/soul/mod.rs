pub const SUCCESS: &str = "ok";
pub const FAILURE: &str = "ko";
pub const DEFAULT_SLEEP_TIME: u64 = 0;

use crossterm::{
    cursor::{MoveLeft, MoveRight},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::size,
};

use std::{
    cell::Cell,
    fmt::Debug,
    io::{Stdout, stdout},
    ops::Add,
    process::ExitCode,
    thread::sleep,
    time::{Duration, Instant},
};

use super::unit::Unit;

///
/// Print a skipped test message to the console
///
/// - `description` The test description
///
pub fn skip_output(description: &str) -> bool {
    if let Ok((x, _)) = size() {
        let mut out: Stdout = stdout();
        let symbol: char = '~';
        let status: &str = SUCCESS;
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "{} {}{}{}{}{}\n",
                    symbol.green().bold(),
                    description.to_lowercase().white().bold(),
                    to_right(x, description),
                    "[ ".white().bold(),
                    status.green().bold(),
                    " ]".white().bold()
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
        let status: &str = SUCCESS;
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
                    to_right(x, description),
                    "[ ".white().bold(),
                    status.green().bold(),
                    " ]".white().bold(),
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
pub fn title_output(title: &str, status: &str) {
    if let Ok((x, _)) = size() {
        let mut out: Stdout = stdout();
        let symbol: char = '#';
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "\n{} {}{}{}{}{}\n\n",
                    symbol.green().bold(),
                    title.to_lowercase().white().bold(),
                    to_right(x, title),
                    "[ ".white().bold(),
                    status.green().bold(),
                    " ]".white().bold(),
                )),
                ResetColor,
            )
            .is_ok()
        );
    } else {
        println!("{title}");
    }
}

fn to_right(x: u16, description: &str) -> MoveRight {
    MoveRight(x - description.len().add(8) as u16)
}

///
/// Close the test suite
///
/// - `success` the failure eq zero
/// - `s` The sussess message
/// - `f` The failure message
///
pub fn results_output(success: bool, s: &str, f: &str, stats: &mut Unit) -> ExitCode {
    if let Ok((x, _)) = size() {
        let mut out: Stdout = stdout();
        let status: String = if success {
            SUCCESS.green().to_string()
        } else {
            FAILURE.red().to_string()
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
                    to_right(x, description),
                    "[ ".white().bold(),
                    status.bold(),
                    " ]".white().bold(),
                )),
                ResetColor,
            )
            .is_ok()
        );
        return if success {
            success_output(format!("asserts  {}", stats.get_assertions().get()).as_str());
            success_output(format!("failure  {}", stats.get_failures().get()).as_str());
            skip_output(format!("skipped  {}", stats.get_skipped().get()).as_str());

            title_output(
                format!("execution time {}s", stats.take().elapsed().as_secs()).as_str(),
                SUCCESS,
            );
            ExitCode::SUCCESS
        } else {
            failure_ouptut(format!("asserts  {}", stats.get_assertions().get()).as_str());
            failure_ouptut(format!("failure  {}", stats.get_failures().get()).as_str());
            skip_output(format!("skipped  {}", stats.get_skipped().get()).as_str());
            title_output(
                format!("execution time {} ms", stats.take().elapsed().as_millis()).as_str(),
                FAILURE,
            );
            ExitCode::FAILURE
        };
    }
    ExitCode::FAILURE
}

///
/// Print a success ot failure message to the console
///
/// - `description` the test description
/// - `test`   the test result
///
pub fn check(description: &str, test: bool, t: u64) -> bool {
    sleep(Duration::from_millis(t));
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
        let status: &str = FAILURE;
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
                    to_right(x, description),
                    "[ ".white(),
                    status.red(),
                    " ]".white(),
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

    fn take(&mut self) -> Instant;
    ///
    /// Define a sub-group of tests
    ///
    /// - `description` The group title
    /// - `it` The callback that runs the group
    ///
    fn subgroup(&mut self, description: &str, it: fn(&mut Self) -> &mut Self) -> &mut Self;
    fn get_assertions(&mut self) -> Cell<usize>;
    fn get_failures(&mut self) -> Cell<usize>;
    fn get_skipped(&mut self) -> Cell<usize>;

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

    fn set_sleep_time(&mut self, time: u64) -> &mut Self;

    /// Display the results
    fn run(&mut self) -> ExitCode;
}
