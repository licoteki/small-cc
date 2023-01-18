use std::process;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenKind {
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
pub enum State {
    Start,
    S1, // expect number || '('
    S2, // expect operator || ')'
    End,
}

#[derive(Debug, Clone)]
pub enum TokenLinkedList<TokenKind> {
    Empty,
    NonEmpty {
        element: TokenKind,
        next: Box<TokenLinkedList<TokenKind>>,
    },
}

pub struct TokenLinkedListIterator<'a, TokenKind: 'a> {
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
    pub fn new(s: String) -> Self {
        Self::tokenize(s, State::Start, 0)
    }

    fn tokenize(s: String, state: State, nest_count: i32) -> Self {
        if s.len() == 0 {
            if nest_count != 0 {
                eprintln!("'('と')'の数が一致しません");
                process::exit(1);
            }
            
            return TokenLinkedList::Empty;
        }

        let (first, last) = s.split_at(1);

        if last.len() == 0 && "+-*/(".contains(first) {
            eprintln!("式の終端文字が不正です");
            process::exit(1);
        }

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

    pub fn print_token(&self) {
        match self {
            TokenLinkedList::Empty => {
                println!("  ret");
            },
            TokenLinkedList::NonEmpty { ref element, next } => {
                print!("{}", element);
                next.print_token();
            },
        }
    }
}

