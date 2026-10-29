use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let doc = fs::read(&args[1]);

    let path = String::from_utf8(doc.expect("rust: can't read the file parsed as input")).unwrap();
    nost_comrak::parse(path);
}
