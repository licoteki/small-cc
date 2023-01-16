use std::process;
use std::env;
use std::fmt;

#[derive(Debug, Clone)]
enum TokenKind {
    Add,
    Sub,
    Mul,
    Div,
    OpenParentheses,
    CloseParentheses,
    Number(i32),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Add => write!(f, "  add rax, "),
            TokenKind::Sub => write!(f, "  sub rax, "),
            TokenKind::Number(n) => write!(f, "{}\n", n),
            _ => Ok(()),
        }
    }
}

#[derive(PartialEq)]
enum State {
    Start,
    S1, // expect number || '('
    S2, // expect operator || ')'
    End,
}

#[derive(Debug, Clone)]
enum TokenLinkedList<TokenKind> {
    Empty,
    NonEmpty {
        element: TokenKind,
        next: Box<TokenLinkedList<TokenKind>>,
    },
}

struct TokenLinkedListIterator<'a, TokenKind: 'a> {
    unvisited: Vec<&'a TokenKind>
}

impl<'a, TokenKind: 'a> TokenLinkedListIterator<'a, TokenKind> {
    fn push_next(&mut self, mut token: &'a TokenLinkedList<TokenKind>) {
        while let TokenLinkedList::NonEmpty { ref element, ref next } = *token {
            self.unvisited.push(element);
            token = &(**next);
        }
    }
}

impl<TokenKind> TokenLinkedList<TokenKind> {
    fn iter(&self) -> TokenLinkedListIterator<TokenKind> {
        let mut iter = TokenLinkedListIterator { unvisited: Vec::new() };
        iter.push_next(self);
        iter
    }
}

impl<'a, TokenKind: 'a> IntoIterator for &'a TokenLinkedList<TokenKind> {
    type Item = &'a TokenKind;
    type IntoIter = TokenLinkedListIterator<'a, TokenKind>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, TokenKind> Iterator for TokenLinkedListIterator<'a, TokenKind> {
    type Item = &'a TokenKind;

    fn next(&mut self) -> Option<&'a TokenKind> {
        self.unvisited.reverse();
        let next = self.unvisited.pop();
        self.unvisited.reverse();
        next
    } 
}

impl TokenLinkedList<TokenKind> {
    fn tokenize(s: String, state: State, nest_count: i32) -> TokenLinkedList<TokenKind> {
        if s.len() == 0 {
            if nest_count != 0 { 
                eprintln!("'('と')'の数が一致しません");
                process::exit(1);
            }
            return TokenLinkedList::Empty;
        }

        let (first, last) = s.split_at(1);

        match first {
            "(" => {
                if state != State::S1 && state != State::Start {
                    eprintln!("'('の位置が不正です");
                    process::exit(1);
                }
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::OpenParentheses,
                    next: Box::new(TokenLinkedList::tokenize(last.to_string(), State::S1, nest_count+1)),
                }
            },
            ")" => {
                if state != State::S2 && state != State::End {
                    eprintln!("')'の位置が不正です");
                    process::exit(1);
                }
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::CloseParentheses,
                    next: Box::new(TokenLinkedList::tokenize(last.to_string(), State::S2, nest_count-1)),
                }
            },
            "+" => {
                if state != State::S2 {
                    eprintln!("'+'の位置が不正です");
                    process::exit(1);
                }
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::Add,
                    next: Box::new(TokenLinkedList::tokenize(last.to_string(), State::S1, nest_count)),
                }
            },
            "-" => {
                if state != State::S2 {
                    eprintln!("'-'の位置が不正です");
                    process::exit(1);
                }
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::Sub,
                    next: Box::new(TokenLinkedList::tokenize(last.to_string(), State::S1, nest_count)),
                }
            },
            "*" => {
                if state != State::S2 {
                    eprintln!("'*'の位置が不正です");
                    process::exit(1);
                }
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::Mul,
                    next: Box::new(TokenLinkedList::tokenize(last.to_string(), State::S1, nest_count)),
                }
            },
            "/" => {
                if state != State::S2 {
                    eprintln!("'/'の位置が不正です");
                    process::exit(1);
                }
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::Div,
                    next: Box::new(TokenLinkedList::tokenize(last.to_string(), State::S1, nest_count)),
                }
            },
            _ => (),
        }

        for (i, c) in s.chars().enumerate() {
            if c.to_string().parse::<i32>().is_ok() && s.len() == i + 1 {
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::Number(s.parse::<i32>().unwrap()),
                    next: Box::new(TokenLinkedList::tokenize("".to_string(), State::S2, nest_count)),
                }                
            } else if c.to_string().parse::<i32>().is_err() {
                return TokenLinkedList::NonEmpty {
                    element: TokenKind::Number(s[0..i].parse::<i32>().unwrap()),
                    next: Box::new(TokenLinkedList::tokenize(s[i..].to_string(), State::S2, nest_count)),
                }
            }; 
        }
        return TokenLinkedList::Empty;
    }

}

enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(i32),
    End,
}

struct Node {
    node_kind: NodeKind,
    lhs_rhs: Option<Box<(Node, Node)>>,
}

impl Node {
    fn new(node_kind: NodeKind, lhs_rhs: Option<Box<(Node, Node)>>) -> Node {
        Node {
            node_kind: node_kind,
            lhs_rhs: lhs_rhs,
        } 
    }

//    fn expr(t: Token) -> Node {
//        let node = mul(t);
//        match t {
//            Token::Next { token_kind, next } => {
//                match token_kind {
//                    TokenKind::Add => {
//                        return Node::new(NodeKind::Add, Some(Box::new((node, mul(*next)))));
//                    },
//                    TokenKind::Sub => {
//                        return Node::new(NodeKind::Sub, Some(Box::new((node, mul(*next)))));
//                    },
//                    _ => return node,
//                }
//            },
//            _ => return Node::new(NodeKind::End, None),
//        }
//
//    }
}

fn print_token(t: TokenLinkedList<TokenKind>) -> TokenLinkedList<TokenKind> {
    match t {
        TokenLinkedList::Empty => {
            println!("  ret");
            return TokenLinkedList::Empty;
        },
        TokenLinkedList::NonEmpty { ref element, next } => {
            print!("{}", element);
            return print_token(*next)
        },
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 { 
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }
    let expression = args[1].clone();
    let token = TokenLinkedList::tokenize(expression, State::Start, 0);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    print!("  mov rax, ");
   
    print_token(token);
    process::exit(0);
}
