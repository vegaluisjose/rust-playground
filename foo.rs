use std::fs::File;
use std::io::{BufWriter, Write};

fn write(prog: &str, path: &str) {
    let f = File::create(path).expect("Unable to create file");
    let mut buf = BufWriter::new(f);
    buf.write_all(prog.as_bytes()).expect("Unable to write data");
}

#[derive(PartialEq, Hash, Debug)]
enum Expr {
    Clock { name: String },
    Module { name: String, port: Vec<Box<Expr>> },
}

// fn my_func(s: &str, n: i8) -> String { format!("{}-->{}\n", s, n) }

// fn main() {
//     let x = String::from("hello");
//     let v = vec![1, 2, 3];
//     let mut y = String::new();
//     for i in v { y.push_str(&my_func(&x, i)) }
//     println!("{}", y);
// }

fn new_line() -> String { String::from("\n") }
fn indent(n: usize) -> String { format!("{s:>w$}", s=String::from(""), w=n) }

fn emit(expr: &Expr) -> String {
    use Expr::*;
    match expr {
        Clock { name } => format!("{}input {}: Clock{}", indent(4), name, new_line()),
        Module { name, port } => {
            let mut p = String::new();
            for i in port { p.push_str(&emit(&i)) }
            format!("{}module {}:{}{}", indent(2), name, new_line(), p)
        },
    }
}

fn main() {
    let clock = vec![Box::new(Expr::Clock { name: "clk".into() })];
    let module = Box::new(Expr::Module { name: "foo".into(), port: clock });
    let prog = emit(&module);
    // let top = Expr::Main(t, module);
    // let prog = emit(&top);
    let path = "foo.fir";
    println!("{}", prog);
    write(&prog, &path);
}