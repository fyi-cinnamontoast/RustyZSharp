use crate::program::Program;
use crate::util::{Raw, raw};

mod lexer;
mod parser;
mod ast;
mod program;
mod environment;
mod util;

fn main() {
    let file = "tests/hello-world.zs";

    let mut prog = Program::new(file);
    prog.exec(r#"
global String hello = "Hello, World!"

func Main() {
    Printl(hello)
}
"#);
    // let res = prog.call("Main");
}
