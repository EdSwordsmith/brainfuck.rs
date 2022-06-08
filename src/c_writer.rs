use std::io::Write;

use crate::ast::*;

pub struct CWriter<T: Write> {
    pub out: T,
}

impl<T: Write> Visitor<OperationNode> for CWriter<T> {
    fn visit_node(&mut self, node: &OperationNode) {
        match node {
            OperationNode::IncrementValue => self.out.write(b"++*ptr;").unwrap(),
            OperationNode::DecrementValue => self.out.write(b"--*ptr;").unwrap(),
            OperationNode::IncrementPointer => self.out.write(b"++ptr;").unwrap(),
            OperationNode::DecrementPointer => self.out.write(b"--ptr;").unwrap(),
            OperationNode::Print => self.out.write(b"putchar(*ptr);").unwrap(),
            OperationNode::Read => self.out.write(b"*ptr = getchar();").unwrap(),
        };
    }
}

impl<T: Write> Visitor<LoopNode> for CWriter<T> {
    fn visit_node(&mut self, node: &LoopNode) {
        self.out.write(b"while (*ptr) {").unwrap();

        for n in node.nodes.iter() {
            self.visit_node(n);
        }

        self.out.write(b"}").unwrap();
    }
}

impl<T: Write> Visitor<Node> for CWriter<T> {
    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Operation(op_node) => self.visit_node(op_node),
            Node::Loop(loop_node) => self.visit_node(loop_node),
        };
    }
}

impl<T: Write> Visitor<ProgramNode> for CWriter<T> {
    fn visit_node(&mut self, node: &ProgramNode) {
        self.out.write(b"#include <stdio.h>\n").unwrap();
        self.out.write(b"char tape[30000] = {0};").unwrap();
        self.out.write(b"char *ptr = tape;").unwrap();
        self.out.write(b"int main() {").unwrap();

        for n in node.nodes.iter() {
            self.visit_node(n);
        }

        self.out.write(b"printf(\"\\n\");").unwrap();
        self.out.write(b"return 0;").unwrap();
        self.out.write(b"}\n").unwrap();
    }
}
