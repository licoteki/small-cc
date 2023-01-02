use std::env;
use std::process;
use std::fmt;

enum Token {
    Operator {
        character: char,
        next: Box<Token>,
    },
    Operand {
        number: i32,
        next: Box<Token>,
    },
    End,
}

impl Token {
    pub fn map<F: Copy + FnMut(Token) -> Token>(self, mut f:F) -> Token {
        match self {
            Token::Operator { character, next } => {
                Token::Operator { character: character, next: Box::new(f(*next)) }
            },
            Token::Operand { number, next } => {
                Token::Operand { number: number, next: Box::new(f(*next)) }
            },
            Token::End => Token::End,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Operator { character, next: _ } => write!(f, "{}", character),
            Token::Operand  { number, next: _ } => write!(f, "{}", number),
            Token::End => write!(f, "End"),
        }
    }
}

fn print_token(t: Token) -> Token{
    if let Token::End = t {
        println!("  ret");
        t
    } else if let Token::Operator { character, next: _ } = t {
        match character {
            '+' => print!("  add rax, "),
            '-' => print!("  sub rax, "),
             _  => (),
        }
        t.map(|e| print_token(e))
    } else if let Token::Operand { number, next: _ } = t {
        println!("{}", number);
        t.map(|e| print_token(e))
    } else {
        t.map(|e| print_token(e))
    }
}

fn lex(expression: String) -> String {
    expression.replace(" ", "")
}

fn tokenize(expression: String) -> Token {
    if expression.len() == 0 { return Token::End; }
    let (first, last) = expression.split_at(1);
    if first == " " { return tokenize(last.to_string()) };
    
    if "+-/*".contains(first) {
        return Token::Operator {
            character: first.chars().nth(0).unwrap(),
            next: Box::new(tokenize(last.to_string())),
        }
    }
    
    let mut number: String = "".to_string();
    for c in expression.chars() {
        if c.is_ascii_digit() {
           number.push(c); 
        } else {
            let (_, last) = expression.split_at(number.len());
            return Token::Operand {
                number: number.parse::<i32>().unwrap(),
                next: Box::new(tokenize(last.to_string())),
            }
        }
    }
    
    return Token::Operand {
        number: number.parse::<i32>().unwrap(),
        next: Box::new(Token::End),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { 
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }
    let expression = args[1].clone();
    let lexed = lex(expression);
    let token = tokenize(lexed);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    print!("  mov rax, ");
   
    print_token(token);
    process::exit(0);
}
