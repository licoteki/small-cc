use crate::token::TokenLinkedList;
use crate::token::TokenKind;

#[derive(Clone, Copy, PartialEq)]
enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Number(u32),
}

struct Node {
    node_kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn new(node_kind: NodeKind, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Node {
        Node { node_kind, lhs, rhs }
    }

    fn expr(t: &TokenLinkedList<TokenKind>) -> Node {
        unimplemented!();
    }
    
    fn mul(t: TokenLinkedList<TokenKind>) -> Option<Node> {
        let lhs = Node::primary(&t).unwrap();
        match t.next().unwrap() {
            TokenLinkedList::NonEmpty { element, next } => {
                match element {
                    TokenKind::Mul => {
                        return Some(Node::new(NodeKind::Mul, Some(Box::new(lhs)), Some(Box::new(Node::primary(&*next).unwrap()))));
                    },
                    TokenKind::Div => {
                        return Some(Node::new(NodeKind::Div, Some(Box::new(lhs)), Some(Box::new(Node::primary(&*next).unwrap()))));
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }

    fn primary(t: &TokenLinkedList<TokenKind>) -> Option<Node> {
        match t {
            TokenLinkedList::NonEmpty { element, next } => {
                match element {
                    TokenKind::OpenParentheses => {
                        return Some(Node::expr(&**next));
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

