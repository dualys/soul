use crossterm::{
    cursor::{MoveLeft, MoveRight},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::size,
};
use std::{
    io::{Stdout, stdout},
    process::ExitCode,
};

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
                    description.white().bold(),
                    MoveRight(x - 8 as u16 - description.len() as u16),
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
            "ok".to_string()
        } else {
            "ko".to_string()
        };
        let symbol: char = if success { '*' } else { '!' };
        let description: &str = if success { s } else { f };
        assert!(
            execute!(
                out,
                MoveLeft(0),
                SetForegroundColor(Color::White),
                Print(format!(
                    "\n{} {}{}{}{}{}\n\n",
                    symbol.green().bold(),
                    description.white().bold(),
                    MoveRight(x - 8 as u16 - description.len() as u16),
                    "[ ".blue().bold(),
                    status.green().bold(),
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
    } else {
        if success {
            println!("* {s}");
            ExitCode::SUCCESS
        } else {
            println!("* {f}");
            ExitCode::FAILURE
        }
    }
}

///
/// Print a success ot failure message to the console
///
/// - `description` the test description
/// - `test`   the test result
///
pub fn check(description: &str, test: bool) -> bool {
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
                    description.white(),
                    MoveRight(x - 8 as u16 - description.len() as u16),
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

    /// Display the results
    fn run(&mut self) -> ExitCode;
}
