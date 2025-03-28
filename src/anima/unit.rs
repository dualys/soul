use std::{cell::Cell, process::ExitCode};

use crate::anima::soul::{Testing, check};

use super::soul::{results_output, title_output};

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

    fn eq<T: PartialEq>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self {
        for test in &data {
            check(description, test.eq(&expected));
        }
        self
    }

    fn ne<T: PartialEq>(&mut self, description: &str, data: Vec<T>, expected: T) -> &mut Self {
        for test in &data {
            check(description, test.ne(&expected));
        }
        self
    }

    fn group(&mut self, description: &str, it: fn(&mut Self) -> &mut Self) -> &mut Self {
        title_output(description);
        it(self)
    }
}

#[cfg(test)]
mod test {
    use std::{env::consts::OS, process::ExitCode};

    use crate::anima::{soul::Testing, unit::Unit};

    fn is_thales_verified(ab: f64, ad: f64, ac: f64, ae: f64) -> bool {
        (ab / ad - ac / ae).abs() < f64::EPSILON
    }
    fn is_median_equals_half_hypotenuse(hypotenuse: f64, median: f64) -> bool {
        (median - hypotenuse / 2.0).abs() < f64::EPSILON
    }

    fn circle_inscribed_radius(a: f64, b: f64, c: f64) -> f64 {
        let s = (a + b + c) / 2.0;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area / s
    }

    #[test]
    pub fn success() -> ExitCode {
        Unit::new()
            .group("Only linux must be fouded", |u| {
                u.ok("Os must be linux", vec![OS == "linux"])
                    .eq("Os const must be equal to linux", vec![OS], "linux")
                    .ne("Os don't be equals to windows", vec![OS], "windows")
                    .ne("Os don't be equals to macos", vec![OS], "macos")
                    .ko("Windows must be not fouded", vec![OS == "windows"])
                    .ko("Macos must be not fouded", vec![OS == "macos"])
            })
            .group("Check the Pythagorean Theorem", |u| {
                let a = 3;
                let b = 4;
                let c = 5;

                u.ok("Triangle is right-angled", vec![a * a + b * b == c * c])
                    .eq(
                        "The hypotenuse squared must equal the sum of squares of the other sides",
                        vec![c * c],
                        a * a + b * b,
                    )
                    .ne(
                        "It must not be a wrong relation",
                        vec![a * a + b * b],
                        a * a + a * b,
                    )
                    .ko(
                        "Wrong triangle relation should fail",
                        vec![6 * 6 + 8 * 8 == 11 * 11],
                    )
            })
            .group("Check the Theorem of Thales", |u| {
                let ab = 8.0;
                let ad = 4.0;
                let ac = 10.0;
                let ae = 5.0;

                let thales_ok = is_thales_verified(ab, ad, ac, ae);
                let thales_ko = is_thales_verified(8.0, 3.0, 10.0, 6.0);

                u.ok(
                    "Thales relation must be verified for parallel lines",
                    vec![thales_ok],
                )
                .ko(
                    "Thales relation must not be verified for non-proportional segments",
                    vec![thales_ko],
                )
            })
            .group("Check the median theorem in a right triangle", |u| {
                let hypotenuse = 10.0;
                let median = 5.0;

                let verified = is_median_equals_half_hypotenuse(hypotenuse, median);
                let wrong = is_median_equals_half_hypotenuse(hypotenuse, 4.5);

                u.ok(
                    "In a right triangle, median from right angle must equal half the hypotenuse",
                    vec![verified],
                )
                .ko(
                    "If the median is not equal to half the hypotenuse, the property fails",
                    vec![wrong],
                )
            })
            .group("Check the inradius theorem", |u| {
                let a = 5.0;
                let b = 5.0;
                let c = 6.0;

                let r = circle_inscribed_radius(a, b, c);
                let expected_r = 1.5;
                u.ok(
                    "Inradius is consistent with area / semi-perimeter",
                    vec![(r - expected_r).abs() < 1e-6],
                )
            })
            .run()
    }
}
