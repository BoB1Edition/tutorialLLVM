use std::mem;
use std::io;

#[allow(dead_code)]
enum Token {
    tok_eof = -1,

  // команды (ключевые слова)
  tok_def = -2, tok_extern = -3,

  // операнды (идентификаторы, числа)
  tok_identifier = -4, tok_number = -5,
}

static mut guess: String = String::new();
static mut NumVal: f64 = 0.0; 

fn getchar() -> char {
    let buf: char;
    let r = io::stdin().read(&mut buf);
    return '\n';
}

fn gettok() {
    let mut LastChar = ' ';
    while LastChar == ' ' {
        LastChar = getchar();
    }
    println!("Hello, world! {}", mem::size_of_val(&LastChar));
}

fn main() {
    println!("Hello, world!");
    gettok();
}
