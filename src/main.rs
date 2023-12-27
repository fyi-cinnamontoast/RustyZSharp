use crate::lexer::Lexer;

mod lexer;

fn main() {
    let code = r#"func Main() {
    String hello = "Hello, World!"
    PrintL(hello)
}
    "#;
    let mut lexer = Lexer::new(code);
    lexer.parse();
    let tokens = lexer.result().unwrap();
    for token in tokens {
        eprintln!("{:?}", token);
    }
}
