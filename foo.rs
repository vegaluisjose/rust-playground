use std::fs::File;
use std::io::{BufWriter, Write};

fn write(prog: &str, path: &str) {
    let f = File::create(path).expect("Unable to create file");
    let mut buf = BufWriter::new(f);
    buf.write_all(prog.as_bytes()).expect("Unable to write data");
}

#[derive(PartialEq, Hash, Debug)]
enum Expr {
    Clock(String),
    Module(String, Box<Expr>),
    Main(String, Box<Expr>)
}

fn new_line() -> String { String::from("\n") }
fn indent(n: usize) -> String { format!("{s:>w$}", s=String::from(""), w=n) }

fn emit(expr: &Expr) -> String {
    use Expr::*;
    match expr {
        Clock(n) => format!("{}input {}: Clock", indent(4), n),
        Module(n, m) => format!("{}module {}:{}{}", indent(2), n, new_line(), emit(m)),
        Main(n, m) => format!("{}circuit {}:{}{}", indent(0), n, new_line(), emit(m))
    }
}

fn main() {
    let s = String::from("foo");
    let t = s.clone();
    let clock = Box::new(Expr::Clock(String::from("clk")));
    let module = Box::new(Expr::Module(s, clock));
    let top = Expr::Main(t, module);
    let prog = emit(&top);
    let path = "foo.fir";
    println!("{}", prog);
    write(&prog, &path);
}