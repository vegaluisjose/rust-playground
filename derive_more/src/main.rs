use derive_more::{Deref, DerefMut};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use serde_sexpr::to_string;

#[derive(Clone, Debug)]
pub struct Foo;

#[derive(Default, Clone, Debug, Deref, DerefMut)]
pub struct Bar(Vec<Foo>);

impl Serialize for Foo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("foo")
    }
}

impl Serialize for Bar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.len() + 1;
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element("net")?;
        for e in self.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

fn main() {
    let mut v = Bar::default();
    v.push(Foo);
    v.push(Foo);
    println!("debug: {:?}", v);
    println!("sexpr: {}", to_string(&v).unwrap());
}
