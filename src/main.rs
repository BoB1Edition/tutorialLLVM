use std::mem;
use std::io;
use std::io::Read;

#[allow(dead_code)]
enum Token {
    tok_eof = -1,

  // команды (ключевые слова)
  tok_def = -2, tok_extern = -3,

  // операнды (идентификаторы, числа)
  tok_identifier = -4, tok_number = -5,
}


static mut NumVal: f64 = 0.0; 

fn getchar() -> Option<char> {
    let input = io::stdin().bytes() 
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as char);
    
    return input;
}

fn gettok(data: String) {
    let mut LastChar = ' ';
    let mut IdentifierStr = String::new();
    while LastChar == ' ' {
        LastChar = getchar().unwrap();
    }
    if LastChar.is_alphabetic() {
        IdentifierStr = LastChar.to_string();
        LastChar = getchar().unwrap();
        while LastChar.is_alphanumeric() {
            IdentifierStr.push_str(&LastChar.to_string());
        }
        println!("Hello, world! {}", IdentifierStr);
    }
}

fn main() {
    println!("Hello, world!");
    let s = "def test()";
    gettok(s.to_string());
}
