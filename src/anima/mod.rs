use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use colored_truecolor::Colorize;

type AnimaFuture = Pin<Box<dyn Future<Output = Anima> + Send>>;
type AnimaHook = fn(Anima) -> AnimaFuture;
type AnimaGroups = Vec<AnimaHook>;

#[derive(Clone)]
pub struct Anima {
    pub failures: Rc<RefCell<u32>>,
    pub asserts: Rc<RefCell<u32>>,
    pub before_all_hooks: Vec<AnimaHook>,
    pub before_each_hooks: Vec<AnimaHook>,
    pub after_all_hooks: Vec<AnimaHook>,
    pub after_each_hooks: Vec<AnimaHook>,
    pub groups: AnimaGroups,
}

impl Anima {
    pub fn new(describe: &str) -> Self {
        println!("\n{describe}");
        Self {
            failures: Rc::new(RefCell::new(0)),
            asserts: Rc::new(RefCell::new(0)),
            before_all_hooks: vec![],
            before_each_hooks: vec![],
            after_all_hooks: vec![],
            after_each_hooks: vec![],
            groups: vec![],
        }
    }

    fn inc_failure(&self) {
        *self.failures.borrow_mut() += 1;
    }

    fn inc_assert(&self) {
        *self.asserts.borrow_mut() += 1;
    }

    pub fn ok<T: IntoIterator<Item = bool>>(&self, iter: T) -> &Self {
        for b in iter {
            if b {
                self.inc_assert();
            } else {
                self.inc_failure();
            }
        }
        self
    }

    pub async fn run(&self) {
        for hook in &self.before_all_hooks {
            let _ = hook(self.clone()).await;
        }

        for group in &self.groups {
            for hook in &self.before_each_hooks {
                let _ = hook(self.clone()).await;
            }

            let _ = group(self.clone()).await;

            for hook in &self.after_each_hooks {
                let _ = hook(self.clone()).await;
            }
        }

        for hook in &self.after_all_hooks {
            let _ = hook(self.clone()).await;
        }
        println!(
            "\n{} {}",
            "ASSERTS".white(),
            self.asserts.borrow().to_string().green()
        );
        println!(
            "{} {}\n",
            "FAILURE".white(),
            self.failures.borrow().to_string().red()
        );
    }

    pub fn ko<T: IntoIterator<Item = bool>>(&self, iter: T) -> &Self {
        for b in iter {
            if b.eq(&false) {
                self.inc_assert();
            } else {
                self.inc_failure();
            }
        }
        self
    }

    pub fn add_before_all_hooks(&mut self, f: AnimaHook) -> &mut Self {
        self.before_all_hooks.push(f);
        self
    }

    pub fn add_after_all_hooks(&mut self, f: AnimaHook) -> &mut Self {
        self.after_all_hooks.push(f);
        self
    }

    pub fn add_after_each_hooks(&mut self, f: AnimaHook) -> &mut Self {
        self.after_each_hooks.push(f);
        self
    }

    pub fn add_before_each_hooks(&mut self, f: AnimaHook) -> &mut Self {
        self.before_each_hooks.push(f);
        self
    }
    pub fn add_groups(&mut self, g: &mut AnimaGroups) -> &mut Self {
        self.groups.append(g);
        self
    }
}

#[cfg(test)]
mod test {
    use tokio::test;

    use crate::anima::Anima;

    #[test]
    pub async fn all() {
        Anima::new("anima")
            .ok(vec![true, true])
            .ko(vec![false, false])
            .run()
            .await
    }
}
