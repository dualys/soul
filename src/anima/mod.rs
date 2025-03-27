use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;

type AnimaFuture = Pin<Box<dyn Future<Output = Anima> + Send>>;
type AnimaHook = fn(Anima) -> AnimaFuture;
type It = fn() -> AnimaFuture;
type AnimaGroups = Vec<AnimaHook>;

#[derive(Clone)]
pub struct Anima {
    pub description: String,
    pub failures: Cell<u32>,
    pub asserts: Cell<u32>,
    pub it: Vec<It>,
    pub before_all_hooks: Vec<AnimaHook>,
    pub before_each_hooks: Vec<AnimaHook>,
    pub after_all_hooks: Vec<AnimaHook>,
    pub after_each_hooks: Vec<AnimaHook>,
    pub groups: AnimaGroups,
}

impl Anima {
    pub async fn new(describe: &str) -> Self {
        Self {
            failures: Cell::new(0),
            asserts: Cell::new(0),
            description: describe.to_string(),
            it: vec![],
            before_all_hooks: vec![],
            before_each_hooks: vec![],
            after_all_hooks: vec![],
            after_each_hooks: vec![],
            groups: vec![],
        }
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
    pub async fn run(&self) {}
}

#[cfg(test)]
mod test {
    #[tokio::test]
    pub async fn all() {}
}
