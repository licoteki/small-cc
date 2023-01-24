use crate::token::{TokenKind, TokenLinkedList};

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
        Node {
            node_kind,
            lhs,
            rhs,
        }
    }

    pub fn expr(t: &mut TokenLinkedList) -> Option<Node> {
        let mut node = Node::mul(t);
        loop {
            if t.consume(TokenKind::Add) {
                node = Some(Node::new(
                    NodeKind::Add,
                    Some(Box::new(node.unwrap())),
                    Some(Box::new(Node::mul(t).unwrap())),
                ));
            } else if t.consume(TokenKind::Sub) {
                node = Some(Node::new(
                    NodeKind::Sub,
                    Some(Box::new(node.unwrap())),
                    Some(Box::new(Node::mul(t).unwrap())),
                ));
            } else {
                return node;
            }
        }
    }

    fn mul(t: &mut TokenLinkedList) -> Option<Node> {
        let mut node = Node::primary(t);
        loop {
            if t.consume(TokenKind::Mul) {
                node = Some(Node::new(
                    NodeKind::Mul,
                    Some(Box::new(node.unwrap())),
                    Some(Box::new(Node::primary(t).unwrap())),
                ));
            } else if t.consume(TokenKind::Div) {
                node = Some(Node::new(
                    NodeKind::Div,
                    Some(Box::new(node.unwrap())),
                    Some(Box::new(Node::primary(t).unwrap())),
                ));
            } else {
                return node;
            }
        }
    }

    fn primary(t: &mut TokenLinkedList) -> Option<Node> {
        if t.consume(TokenKind::OpenParentheses) {
            let e = Node::expr(t);
            t.expect(TokenKind::CloseParentheses);
            return e;
        }

        return Some(Node::new(
            NodeKind::Number(t.expect_number().unwrap()),
            None,
            None,
        ));
    }

    pub fn compile(n: Node) {
        match n.node_kind {
            NodeKind::Number(n) => println!("PUSH {}", n),
            NodeKind::Add => {
                Node::compile(*n.lhs.unwrap());
                Node::compile(*n.rhs.unwrap());
                println!("ADD");
            }
            NodeKind::Sub => {
                Node::compile(*n.lhs.unwrap());
                Node::compile(*n.rhs.unwrap());
                println!("Sub");
            }
            NodeKind::Mul => {
                Node::compile(*n.lhs.unwrap());
                Node::compile(*n.rhs.unwrap());
                println!("Mul");
            }
            NodeKind::Div => {
                Node::compile(*n.lhs.unwrap());
                Node::compile(*n.rhs.unwrap());
                println!("Div");
            }
        }
    }
}
