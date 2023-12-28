use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use crate::lexer::Lexer;

mod lexer;
mod parser;
mod ast;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file = "tests/hello-world.zs";
    let path = current_dir.join(file);
    let mut fs = File::open(path.clone()).expect(format!("Unable to open {}!", file).as_str());
    let mut code = String::new();
    fs.read_to_string(&mut code).expect("Unable to read the file!");
    let mut lexer = Lexer::new(code.as_str());
    lexer.parse();
    let Ok(tokens) = lexer.result() else {
        let errs = lexer.result().unwrap_err();
        let lines = code.split("\n");
        println!("--> {}", file);
        for error in errs {
            println!("{}", error);
        }
        exit(-1);
    };
    for token in tokens {
        eprintln!("{:?}", token);
    }
}
