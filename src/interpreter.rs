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
            OperationNode::IncrementValue(value) => { 
                if *value > 0 {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(*value as u8);
                } else {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub((- *value) as u8);
                }
            },
            OperationNode::IncrementPointer(value) => { self.ptr = (self.ptr as isize + value) as usize; },
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
