mod cloudcft;
use cloudcft::code_generator::CodeGenerator;
use cloudcft::parser::{Parser,AstNode};
use cloudcft::lexer::Lexer;
fn main() {
    let mut code=CodeGenerator::new();
    let mut lexer=Lexer::new("num a = 1;");
    let mut parser=Parser::new(lexer);
    let s=code.generate_code(parser.parse());
    println!("{}",s);

}
