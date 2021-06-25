use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use derive_more::{Deref, DerefMut};

pub type Id = String;

#[derive(Clone, Debug, Eq)]
pub struct Foo {
    id: Id,
    val: u32,
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Set(HashSet<Foo>);

impl Set {
    pub fn new() -> Self {
        Set(HashSet::new())
    }
}

impl PartialEq for Foo {
    fn eq(&self, other: &Foo) -> bool {
        self.id == other.id
    }
}

impl Hash for Foo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<Id> for Foo {
    fn borrow(&self) -> &Id {
        &self.id
    }
}

impl Borrow<str> for Foo {
    fn borrow(&self) -> &str {
        &self.id.as_str()
    }
}

fn main() {
    let mut set = Set::new();
    set.insert(Foo {
        id: "a".to_string(),
        val: 2,
    });
    let key = "a".to_string();
    if let Some(foo) = set.get(&key) {
        println!("foo:{:?}", foo);
    }
}
