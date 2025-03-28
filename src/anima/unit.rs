use std::{cell::Cell, process::ExitCode};

use crate::anima::soul::{Testing, check};

use super::soul::results_output;

pub struct Unit {
    asserts: Cell<usize>,
    failures: Cell<usize>,
}

impl Testing for Unit {
    fn ok(&mut self, description: &str, data: Vec<bool>) -> &mut Self {
        for t in &data {
            if check(description, t.clone().eq(&true)).eq(&true) {
                self.asserts.set(self.asserts.get() + 1);
            } else {
                self.failures.set(self.failures.get() + 1);
            }
        }
        self
    }

    fn ko(&mut self, description: &str, data: Vec<bool>) -> &mut Self {
        for t in &data {
            if check(description, t.clone().eq(&false)).eq(&true) {
                self.asserts.set(self.asserts.get() + 1);
            } else {
                self.failures.set(self.failures.get() + 1);
            }
        }
        self
    }

    fn new() -> Self {
        Self {
            asserts: Cell::new(0),
            failures: Cell::new(0),
        }
    }

    fn run(&mut self) -> ExitCode {
        results_output(
            self.failures.get().eq(&0),
            "No errors has been fouded",
            "Errors has been founded",
        )
    }

    fn eq<T: Eq>(&mut self, description: &str, data: Vec<T>, to: T) -> &mut Self {
        for test in &data {
            check(description, test.eq(&to));
        }
        self
    }
}

#[cfg(test)]
mod test {
    use std::{env::consts::OS, process::ExitCode};

    use crate::anima::{soul::Testing, unit::Unit};
    #[test]
    pub fn success() -> ExitCode {
        Unit::new()
            .ok("All must match true", vec![true, true, true])
            .ko("All must match false", vec![false, false, false])
            .eq("Linux must be running", vec![OS, OS, OS], "linux")
            .run()
    }
}
