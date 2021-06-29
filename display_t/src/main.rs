use derive_more::Display;
use std::fmt;

#[derive(Clone, Debug, Display)]
#[display(fmt = "name:{} val:{}", name, val)]
pub struct Foo<T: fmt::Display> {
    name: String,
    val: T,
}

#[derive(Clone, Debug, Display)]
#[display(fmt = "{}", _0)]
pub struct Bar(u32);

fn main() {
    let data = Foo { name:"some".to_string(), val: Bar(4) };
    println!("{}", data);    
}
