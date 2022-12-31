use std::env;
use std::process;
use std::mem;

struct TokenReserved {
    ope: char,
}

struct TokenNumber {
    value: i32,
}

trait TokenMarker {}

impl TokenMarker for TokenReserved {}
impl TokenMarker for TokenNumber {}

struct Token {
    current: Box<dyn TokenMarker>,
    next: Option<Box<dyn TokenMarker>>,
}

impl Token {
    fn tokenize(input: String) {
        if input.chars().nth(0).unwrap() == ' ' {}
        if "+-/*".find(input.chars().nth(0).unwrap()).unwrap() >= 0 {
            let current = Box::new(
                TokenReserved { ope: input.chars().nth(0).unwrap() }
            );
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { 
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    //let token = tokenize(args[1]);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    process::exit(0);
}
