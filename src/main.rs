mod tokenizer;

use std::fs;
use tokenizer::Lexer;

fn main() {

    let content = fs::read_to_string("mylang.lod").expect("No file exist");

    let mut lexer = Lexer::new(content);

    lexer.tokenize();    
    lexer.print_tokens();

}
