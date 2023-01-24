use std::collections::LinkedList;
use std::fmt;
use std::io::{Error, ErrorKind};
use std::process;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Add,
    Sub,
    Mul,
    Div,
    OpenParentheses,
    CloseParentheses,
    Number(u32),
}

impl TokenKind {
    fn from_char(c: char) -> Result<Self, Error> {
        let tk = match c {
            '+' => TokenKind::Add,
            '-' => TokenKind::Sub,
            '*' => TokenKind::Mul,
            '/' => TokenKind::Div,
            '(' => TokenKind::OpenParentheses,
            ')' => TokenKind::CloseParentheses,
            _ => return Err(Error::new(ErrorKind::InvalidInput, "予期しない文字")),
        };
        Ok(tk)
    }

    fn from_num(n: u32) -> Result<Self, Error> {
        Ok(TokenKind::Number(n))
    }
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

#[derive(Debug)]
pub struct TokenLinkedList {
    pub list: LinkedList<TokenKind>,
}

impl TokenLinkedList {
    pub fn new() -> Self {
        TokenLinkedList {
            list: LinkedList::new(),
        }
    }

    pub fn from(s: String) -> Result<Self, std::io::Error> {
        TokenLinkedList::tokenize(s)
    }

    pub fn tokenize(s: String) -> Result<Self, std::io::Error> {
        let mut state = State::Start;
        let mut nest_count = 0;
        let mut tll = TokenLinkedList::new();
        let mut number = String::from("");

        for c in s.chars() {
            number.push(c);

            if number.parse::<u32>().is_err() && number.len() == 1 {
                number = String::from("");
            } else if number.parse::<u32>().is_err() && number.len() > 1 {
                tll.list.push_back(TokenKind::Number(
                    number[0..number.len() - 1].parse::<u32>().unwrap(),
                ));
                number = String::from("");
            }

            match c {
                '+' | '-' | '*' | '/' => {
                    if state != State::S2 {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("{}の位置が不適切です。", c),
                        ));
                    }
                    tll.list.push_back(TokenKind::from_char(c).unwrap());
                    state = State::S1;
                }
                '(' => {
                    if state != State::S1 && state != State::Start {
                        return Err(Error::new(ErrorKind::InvalidInput, "(の位置が不適切です。"));
                    }
                    tll.list.push_back(TokenKind::OpenParentheses);
                    state = State::S1;
                    nest_count += 1;
                }
                ')' => {
                    if state != State::S2 && state != State::End {
                        return Err(Error::new(ErrorKind::InvalidInput, ")の位置が不適切です。"));
                    }
                    tll.list.push_back(TokenKind::CloseParentheses);
                    state = State::S2;
                    nest_count -= 1;
                }
                '0'..='9' => {
                    state = State::S2;
                }
                ' ' => continue,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "パースできない文字です。",
                    ))
                }
            }
        }
        if number.parse::<u32>().is_ok() {
            tll.list
                .push_back(TokenKind::Number(number[0..].parse::<u32>().unwrap()));
        }

        if state == State::S1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "式が演算子で終了しています。",
            ));
        }
        if nest_count != 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "カッコが対応していません。",
            ));
        }
        Ok(tll)
    }

    pub fn print_token(&self) {
        for t in self.list.iter() {
            print!("{}", t);
        }
        println!("  ret");
    }

    pub fn consume(&mut self, tk: TokenKind) -> bool {
        if self.list.len() == 0 {
            return false;
        }
        let t = self.list.pop_front();
        if t == Some(tk) {
            return true;
        }
        self.list.push_front(t.unwrap());
        false
    }

    pub fn expect(&mut self, tk: TokenKind) -> Result<(), std::io::Error> {
        if self.list.len() == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "failed expect.",
            ));
        }
        let t = self.list.pop_front();
        if t == Some(tk) {
            return Ok(());
        }
        self.list.push_front(t.unwrap());
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "failed expect.",
        ))
    }

    pub fn expect_number(&mut self) -> Result<u32, std::io::Error> {
        if self.list.len() == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "failed expect.",
            ));
        }
        let t = self.list.pop_front();
        match t {
            Some(TokenKind::Number(n)) => return Ok(n),
            _ => {
                self.list.push_front(t.unwrap());
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "failed expect.",
                ));
            }
        }
    }
}
