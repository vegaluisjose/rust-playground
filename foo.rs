// use std::fs::File;
// use std::io::{BufWriter, Write};


// io write file
// fn emit(prog: String, path: String) {
//     let f = File::create(path).expect("Unable to create file");
//     let mut buf = BufWriter::new(f);
//     buf.write_all(prog.as_bytes()).expect("Unable to write data");
// }

#[derive(PartialEq, Hash, Debug)]
enum Expr {
    Module(String),
    Main(String, Box<Expr>)
}

fn eval(expr: Expr) -> String {
    use Expr::*;
    match expr {
        Module(n) => format!("{}", n),
        Main(n, _) => format!("{}", n)
    }
}

fn main() {
    // let path = String::from("foo.txt");
    // let mut data = String::from("Luis");
    // data.push('\n');
    // data.push_str("new line");
    // println!("string: {}", data);
    // emit(data, path);
    // let tmp = eval(Expr::Clock(String::from("Luis")));
    let p = String::from("mod");
    let q = String::from("mod");
    let m = Box::new(Expr::Module(p));
    let x = Expr::Main(String::from(q), m);
    let t = eval(x);
    println!("{}", t);
    // println!("{}", tmp);
}