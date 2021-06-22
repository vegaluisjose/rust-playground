use lexpr::Value;
use std::fmt;

pub trait ToValue {
    fn to_value(&self) -> Value;
}

#[derive(Default, Clone, Debug)]
pub struct Name {
    pub name: String,
    pub rename: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Net {
    pub name: Name,
}

// we could use a macro for Display impl?
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_value())
    }
}

impl fmt::Display for Net {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_value())
    }
}

impl Name {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        Name {
            name: name.as_ref().to_string(),
            rename: None,
        }
    }
    pub fn new_with_rename<S>(name: S, rename: S) -> Self
    where
        S: AsRef<str>,
    {
        Name {
            name: name.as_ref().to_string(),
            rename: Some(rename.as_ref().to_string()),
        }
    }
    pub fn rename(&self) -> Option<&String> {
        self.rename.as_ref()
    }
}

impl ToValue for Name {
    fn to_value(&self) -> Value {
        let name = Value::symbol(self.name.clone().into_boxed_str());
        if let Some(rename) = self.rename() {
            let keyword = String::from("rename");
            let keyword = Value::symbol(keyword.into_boxed_str());
            let rename = Value::string(rename.clone().into_boxed_str());
            Value::list(vec![keyword, name, rename])
        } else {
            name
        }
    }
}

impl ToValue for Net {
    fn to_value(&self) -> Value {
        let net = String::from("net");
        let net = Value::symbol(net.into_boxed_str());
        Value::list(vec![net, self.name.to_value()])
    }
}

fn main() {
    let name = Name::new_with_rename("CLOCK", "A[0]");
    println!("{}", name);
}
