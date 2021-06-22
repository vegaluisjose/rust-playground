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

#[derive(Default, Clone, Debug)]
pub struct PortRef {
    pub name: String,
    pub instance: Option<String>,
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

impl fmt::Display for PortRef {
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
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn rename(&self) -> Option<&String> {
        self.rename.as_ref()
    }
}

impl PortRef {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str>,
    {
        PortRef {
            name: name.as_ref().to_string(),
            instance: None,
        }
    }
    pub fn new_with_instance<S>(name: S, instance: S) -> Self
    where
        S: AsRef<str>,
    {
        PortRef {
            name: name.as_ref().to_string(),
            instance: Some(instance.as_ref().to_string()),
        }
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn instance(&self) -> Option<&String> {
        self.instance.as_ref()
    }
}

fn symbol_from_str<S: AsRef<str>>(val: S) -> Value {
    let v = val.as_ref().to_string();
    Value::symbol(v.into_boxed_str())
}

impl ToValue for Name {
    fn to_value(&self) -> Value {
        let name = symbol_from_str(self.name());
        if let Some(v) = self.rename() {
            let rename = symbol_from_str("rename");
            let value = Value::from(v.clone());
            Value::list(vec![rename, name, value])
        } else {
            name
        }
    }
}

impl ToValue for Net {
    fn to_value(&self) -> Value {
        let net = symbol_from_str("net");
        Value::list(vec![net, self.name.to_value()])
    }
}

impl ToValue for PortRef {
    fn to_value(&self) -> Value {
        let p = symbol_from_str("portref");
        let v = symbol_from_str(self.name());
        if let Some(i) = self.instance() {
            let ir = Value::list(vec![symbol_from_str("instanceref"), symbol_from_str(i)]);
            Value::list(vec![p, v, ir])
        } else {
            Value::list(vec![p, v])
        }
    }
}

pub fn main() {
    println!("try unit tests, cargo test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let res = Name::new("CLOCK");
        let exp = String::from("CLOCK");
        assert_eq!(res.to_string(), exp)
    }

    #[test]
    fn test_name_with_rename() {
        let res = Name::new_with_rename("CLOCK", "A[0]");
        let exp = String::from(r#"(rename CLOCK "A[0]")"#);
        assert_eq!(res.to_string(), exp)
    }

    #[test]
    fn test_portref() {
        let res = PortRef::new("CE");
        let exp = String::from("(portref CE)");
        assert_eq!(res.to_string(), exp)
    }
}
