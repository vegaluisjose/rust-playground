use std::fs::File;
use std::io::{BufWriter, Write};

fn write(prog: &str, path: &str) {
    let f = File::create(path).expect("Unable to create file");
    let mut buf = BufWriter::new(f);
    buf.write_all(prog.as_bytes()).expect("Unable to write data");
}

#[derive(PartialEq, Hash, Debug)]
#[allow(dead_code)]
enum Expr {
    Clock { name: String },
    Reset { name: String },
    ResetLow { name: String },
    Module { name: String, io: Vec<Box<Expr>> },
    Main { name: String, module: Vec<Box<Expr>> },
}

fn new_line() -> String { String::from("\n") }
fn indent(n: usize) -> String { format!("{s:>w$}", s=String::from(""), w=n) }

fn emit(expr: &Expr) -> String {
    use Expr::*;
    match expr {
        Clock { name } => {
            format!("{}input {}: Clock{}", indent(4), name, new_line())
        },
        Reset { name } => {
            format!("{}input {}: UInt<1>{}", indent(4), name, new_line())
        },
        ResetLow { name } => {
            format!("{}input {}: UInt<1>{}", indent(4), name, new_line())
        },
        Module { name, io } => {
            let mut p = String::new();
            for i in io { p.push_str(&emit(&i)) }
            format!("{}module {}:{}{}", indent(2), name, new_line(), p)
        },
        Main { name, module } => {
            let mut m = String::new();
            for i in module { m.push_str(&emit(&i)) }
            format!("{}circuit {}:{}{}", indent(0), name, new_line(), m)
        },
    }
}

fn main() {
    let mut io = Vec::new();
    let mut module = Vec::new();
    io.push(Box::new(Expr::Clock { name: "clk".into() }));
    io.push(Box::new(Expr::Reset { name: "rst".into() }));
    module.push(Box::new(Expr::Module { name: "foo".into(), io: io }));
    let top = Box::new(Expr::Main { name: "foo".into(), module: module });
    let prog = emit(&top);
    let path = "foo.fir";
    println!("{}", prog);
    write(&prog, &path);
}