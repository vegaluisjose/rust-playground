use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let mut data = String::from("Luis, ");
    data.push('\n');
    data.push_str("new line");
    println!("string: {}", data);
    let f = File::create("foo.txt").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}