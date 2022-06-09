use std::io::Read;

use crate::ast::*;

pub struct Interpreter {
    tape: [u8; 30000],
    ptr: usize,
}

impl Interpreter {
    pub fn new() -> Self { Interpreter { tape: [0; 30000], ptr: 0 } }
}

impl Visitor<OperationNode> for Interpreter {
    fn visit_node(&mut self, node: &OperationNode) {
        match node {
            OperationNode::IncrementValue => { self.tape[self.ptr] += 1; },
            OperationNode::DecrementValue => { self.tape[self.ptr] -= 1; },
            OperationNode::IncrementPointer => { self.ptr += 1; },
            OperationNode::DecrementPointer => { self.ptr -= 1; },
            OperationNode::Print => print!("{}", self.tape[self.ptr] as char),
            OperationNode::Read => {
                let mut stdin = std::io::stdin();
                let mut buffer = [0u8; 1];
                stdin.read_exact(&mut buffer).unwrap();
                self.tape[self.ptr] = buffer[0];
            },
        };
    }
}

impl Visitor<LoopNode> for Interpreter {
    fn visit_node(&mut self, node: &LoopNode) {
        while self.tape[self.ptr] != 0 {
            for n in node.nodes.iter() {
                self.visit_node(n);
            }
        }
    }
}

impl Visitor<Node> for Interpreter {
    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Operation(op_node) => self.visit_node(op_node),
            Node::Loop(loop_node) => self.visit_node(loop_node),
        };
    }
}

impl Visitor<ProgramNode> for Interpreter {
    fn visit_node(&mut self, node: &ProgramNode) {
        for n in node.nodes.iter() {
            self.visit_node(n);
        }
    }
}
