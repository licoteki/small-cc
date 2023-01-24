use std::env;
use std::fmt;
use std::process;

mod node;
mod token;

use token::State;
use token::TokenLinkedList;

use node::Node;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }
    let expression = args[1].clone();
    let mut token = TokenLinkedList::from(expression).unwrap();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    print!("  mov rax, ");

    token.print_token();

    let node = Node::expr(&mut token);

    println!("{:?}", &node);
    Node::compile(node.unwrap());
    process::exit(0);
}
