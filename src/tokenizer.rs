use core::panic;
use std::fmt;
use regex::Regex;

enum TokenType{
    NUMBER,
    STRING,
    OPERATOR,
    SPECIAL,
    VARIABLE
}

impl fmt::Display for TokenType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            TokenType::NUMBER => write!(f, "NUMBER()"),
            TokenType::STRING => write!(f, "STRING()"),
            TokenType::OPERATOR => write!(f, "OPERATOR()"),
            TokenType::SPECIAL => write!(f, "SPECIAL()"),
            TokenType::VARIABLE => write!(f, "VARIABLE()")
        }
    }

}

pub struct Token{
    token_type: TokenType,
    value: String
}

pub struct Lexer{
    content: String,
    content_len: u32, 
    tokens: Vec<Token>,
    counter: u32,
}

impl Lexer{

    pub fn new(content: String) -> Lexer {
        let content_len = content.len() as u32;
        Lexer { content, content_len, tokens: vec![], counter: 0 }
    }

    pub fn print_tokens(&self){

        for token in self.tokens.iter(){
            println!("{} : {}",token.token_type,token.value);
        }

    }

    pub fn tokenize(& mut self){

        
        let num_re = Regex::new(r"[0-9]").unwrap();
        let variable_reg = Regex::new(r"^[a-zA-Z]*[a-zA-Z_]").unwrap();

        while self.counter != self.content_len{
            
            let chr = self.content.chars().nth(self.counter as usize).unwrap();

            if chr == '\n' || chr == ' '{
                self.counter += 1;
                continue;
            }

            else if "+-*/".contains(chr){
                self.tokens.push(Token{token_type: TokenType::OPERATOR, value: chr.to_string()});
            }
            
            else if "(){},;=:".contains(chr){
                self.tokens.push(Token{token_type: TokenType::SPECIAL, value: chr.to_string()});
            }

            else if chr == '\'' || chr == '"'{
                self.tokens.push(Token{token_type: TokenType::STRING, value: chr.to_string()});
                self.counter += 1;
                
                let mut string = String::from("");
                
                while true{
                    
                    let string_char = self.content.chars().nth(self.counter as usize).unwrap();
                    if string_char == '\'' || string_char == '"'{
                        break;
                    }

                    string.push(string_char);
                    self.counter += 1;
                }

                self.tokens.push(Token{token_type: TokenType::STRING, value: string});
                self.tokens.push(Token{token_type: TokenType::STRING, value: chr.to_string()});
                self.counter += 1;
                continue;
            }

            else if variable_reg.is_match(&chr.to_string()) {
                
                let mut variable = String::from("");
                
                while true{
                    let var_char = self.content.chars().nth(self.counter as usize).unwrap();
                    if !variable_reg.is_match(&var_char.to_string()){
                        break;
                    }
                    variable.push(var_char.clone());
                    self.counter += 1;
                
                }
                self.tokens.push(Token { token_type: TokenType::VARIABLE, value: variable });
            }

            else if num_re.is_match(&chr.to_string()){

                let mut nummber = String::from("");

                while true{
                    let num_chr = self.content.chars().nth(self.counter as usize).unwrap();
                    
                    if !num_re.is_match(&num_chr.to_string()){
                        break;
                    }
                    
                    nummber.push(num_chr.clone());
                    self.counter += 1;
                }
                self.tokens.push(Token { token_type: TokenType::NUMBER, value: nummber });
            
            }
            
            else {
                panic!("unexpected token recieved");
            }

            self.counter += 1;
        }
    }

}