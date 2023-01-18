enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(i32),
}

struct Node {
    node_kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn new(node_kind: NodeKind, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Node {
        Node {
            node_kind: node_kind,
            lhs: lhs,
            rhs: rhs,
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

