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
    if "+-*/".contains(expression.chars().nth(0).unwrap()) {
        eprintln!("{}\n{}式が演算子で開始されています", expression, "^");
        process::exit(1);
    }
    if "+-*/".contains(expression.chars().nth(expression.len() - 1).unwrap()) {
        eprintln!("{}\n{}{}式が演算子で終了しています", expression, " ".repeat(expression.len() - 1), "^");
        process::exit(1);
    }
    for (i, c) in expression.chars().enumerate() {
        match "0123456789+-*/ ".contains(c) {
            false => {
                eprintln!("{}\n{}{}トークナイズできない文字が存在します。", expression, " ".repeat(i), "^");
                process::exit(1);
            },
            _ => (),
        }
    }
    expression.replace(" ", "")
}

fn tokenize(expression: String) -> Token {
    if expression.len() == 0 { return Token::End; }
    let (first, last) = expression.split_at(1);
    
    if "+-/*".contains(first) {
        return Token::Operator {
            character: first.chars().nth(0).unwrap(),
            next: Box::new(tokenize(last.to_string())),
        }
    }
    
    let mut number: String = "".to_string();
    
    for (i, c) in expression.chars().enumerate() {
        if "+-/*".contains(c) {
            let (_, last) = expression.split_at(number.len());
            if "+-/*".contains(last.chars().nth(1).unwrap()) {
                eprintln!("{}\n{}{}演算子が連続しています", expression, " ".repeat(i + 1), "^");
                process::exit(1);
            }
            return Token::Operand {
                number: number.parse::<i32>().unwrap(),
                next: Box::new(tokenize(last.to_string())),
            }
        }
        number.push(c); 
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
