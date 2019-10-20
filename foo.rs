use std::fs::File;
use std::io::{BufWriter, Write};


// io write file
fn emit(prog: String, path: String) {
    let f = File::create(path).expect("Unable to create file");
    let mut buf = BufWriter::new(f);
    buf.write_all(prog.as_bytes()).expect("Unable to write data");
}

fn main() {
    let path = String::from("foo.txt");
    let mut data = String::from("Luis");
    data.push('\n');
    data.push_str("new line");
    println!("string: {}", data);
    emit(data, path);
}