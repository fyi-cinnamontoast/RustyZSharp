use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use crate::lexer::Lexer;
use crate::program::Program;

mod lexer;
mod parser;
mod ast;
mod program;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file = "tests/hello-world.zs";
    let path = current_dir.join(file);

    let mut fs = File::open(path.clone()).expect(format!("Unable to open {}!", file).as_str());

    let mut code = String::new();
    fs.read_to_string(&mut code).expect("Unable to read the file!");

    let mut prog = Program::new(file);
    prog.exec(code.as_str());
}
