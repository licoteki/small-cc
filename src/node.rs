use crate::token::TokenLinkedList;
use crate::token::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Number(u32),
}

#[derive(Debug)]
pub struct Node {
    node_kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn new(node_kind: NodeKind, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Node {
        Node { node_kind, lhs, rhs }
    }

    pub fn expr(t: &TokenLinkedList<TokenKind>) -> Option<Node> {
        let lhs = Node::mul(&t);
        match t.next().unwrap() {
            TokenLinkedList::NonEmpty { element, next } => {
                match element {
                    TokenKind::Add => {
                        return Some(Node::new(NodeKind::Add, Some(Box::new(lhs.unwrap())), Some(Box::new(Node::mul(&*next).unwrap()))));
                    },
                    TokenKind::Sub => {
                        return Some(Node::new(NodeKind::Sub, Some(Box::new(lhs.unwrap())), Some(Box::new(Node::mul(&*next).unwrap()))));
                    },
                    _ => lhs,
                }
            },
            _ => lhs,
        }
    }
    
    fn mul(t: &TokenLinkedList<TokenKind>) -> Option<Node> {
        let lhs = Node::primary(&t);
        match t.next().unwrap() {
            TokenLinkedList::NonEmpty { element, next } => {
                match element {
                    TokenKind::Mul => {
                        return Some(Node::new(NodeKind::Mul, Some(Box::new(lhs.unwrap())), Some(Box::new(Node::primary(&*next).unwrap()))));
                    },
                    TokenKind::Div => {
                        return Some(Node::new(NodeKind::Div, Some(Box::new(lhs.unwrap())), Some(Box::new(Node::primary(&*next).unwrap()))));
                    },
                    _ => lhs,
                }
            },
            _ => lhs,
        }
    }

    fn primary(t: &TokenLinkedList<TokenKind>) -> Option<Node> {
        match t {
            TokenLinkedList::NonEmpty { element, next } => {
                match element {
                    TokenKind::OpenParentheses => {
                        return Node::expr(&**next);
                    },
                    TokenKind::Number(n) => {
                        return Some(Node::new(NodeKind::Number(*n), None, None));
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }
}

