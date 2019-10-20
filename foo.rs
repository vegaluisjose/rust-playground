use std::fs::File;
use std::io::{BufWriter, Write};

fn write(prog: String, path: String) {
    let f = File::create(path).expect("Unable to create file");
    let mut buf = BufWriter::new(f);
    buf.write_all(prog.as_bytes()).expect("Unable to write data");
}

#[derive(PartialEq, Hash, Debug)]
enum Expr {
    Module(String),
    Main(String, Box<Expr>)
}

fn new_line() -> String { String::from("\n") }

fn indent(n: usize, s: String) -> String {
    let i = format!("{s:>w$}", s=String::from(""), w=n);
    let is = format!("{}{}", i, s);
    is
}

fn emit(expr: &Expr) -> String {
    use Expr::*;
    match expr {
        Module(n) => indent(2, format!("module {}:", n)),
        Main(n, m) => indent(0, format!("circuit {}:{}{}", n, new_line(), emit(m)))
    }
}

fn main() {
    let s = String::from("foo");
    let t = s.clone();
    let m = Box::new(Expr::Module(s));
    let x = Expr::Main(t, m);
    let prog = emit(&x);
    let path = String::from("foo.fir");
    println!("{}", prog);
    write(prog, path);
}