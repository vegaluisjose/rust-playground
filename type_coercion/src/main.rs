use derive_more::From;

#[derive(Clone, Debug)]
pub struct Foo;

#[derive(Clone, Debug)]
pub struct Bar;

// using [derive(From)] from derive_more save us
// implementing the following
//
// From<T> for Val {
//     fn from(input: T) -> Self {
//         Val::T(T)
//     }
// }
//
// Where T can be Foo or Bar
#[derive(Clone, Debug, From)]
pub enum Val {
    Foo(Foo),
    Bar(Bar),
}


#[derive(Clone, Debug)]
pub struct Rec {
    val: Val,
}

impl Rec {
    pub fn new() -> Self {
        let val = Val::from(Foo);
        Rec { val }
    }
    // type coercion (conversion) part
    // val can be Foo or Bar and it will
    // get converted to Val
    pub fn set_val<V>(&mut self, val: V)
    where
        V: Into<Val>
    {
        self.val = val.into();
    }
}


fn main() {
    let mut rec = Rec::new();
    println!("{:?}", &rec);
    rec.set_val(Bar);
    println!("{:?}", &rec);
    rec.set_val(Foo);
    println!("{:?}", &rec);
}
