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

fn emit(expr: &Expr) -> String {
    use Expr::*;
    match expr {
        Module(n) => format!("  module {}:\n", n),
        Main(n, m) => format!("circuit {}:\n{}", n, emit(m))
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