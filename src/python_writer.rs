use std::io::Write;

use crate::ast::*;

pub struct PythonWriter<T: Write> {
    out: T,
    ident: u32,
}

impl<T: Write> PythonWriter<T> {
    pub fn new(out: T) -> Self {
        PythonWriter { out: out, ident: 0 }
    }
}

impl<T: Write> Visitor<OperationNode> for PythonWriter<T> {
    fn visit_node(&mut self, node: &OperationNode) {
        match node {
            OperationNode::IncrementValue => self.out.write(b"inc_value()\n").unwrap(),
            OperationNode::DecrementValue => self.out.write(b"dec_value()\n").unwrap(),
            OperationNode::IncrementPointer => self.out.write(b"ptr += 1\n").unwrap(),
            OperationNode::DecrementPointer => self.out.write(b"ptr -= 1\n").unwrap(),
            OperationNode::Print => self.out.write(b"print(chr(tape[ptr]), end='')\n").unwrap(),
            OperationNode::Read => self.out.write(b"read_stdin()\n").unwrap(),
        };
    }
}

impl<T: Write> Visitor<LoopNode> for PythonWriter<T> {
    fn visit_node(&mut self, node: &LoopNode) {
        self.out.write(b"while should_loop():\n").unwrap();

        for n in node.nodes.iter() {
            self.visit_node(n);
        }

        if node.nodes.len() == 0 {
            for _ in 0..self.ident {
                self.out.write(b"    ").unwrap();
            }

            self.out.write(b"pass\n").unwrap();
        }
    }
}

impl<T: Write> Visitor<Node> for PythonWriter<T> {
    fn visit_node(&mut self, node: &Node) {
        for _ in 0..self.ident {
            self.out.write(b"    ").unwrap();
        }

        match node {
            Node::Operation(op_node) => self.visit_node(op_node),
            Node::Loop(loop_node) => self.visit_node(loop_node),
        };
    }
}

impl<T: Write> Visitor<ProgramNode> for PythonWriter<T> {
    fn visit_node(&mut self, node: &ProgramNode) {
        self.out.write(b"#!/usr/bin/python3\n").unwrap();
        self.out.write(b"tape = {}\n").unwrap();
        self.out.write(b"ptr = 0\n\n").unwrap();
        self.out.write(b"current_input = ''\n\n").unwrap();
        self.out.write(b"def inc_value():\n").unwrap();
        self.out.write(b"    if ptr not in tape:\n").unwrap();
        self.out.write(b"        tape[ptr] = 0\n").unwrap();
        self.out.write(b"    tape[ptr] += 1\n\n").unwrap();
        self.out.write(b"def dec_value():\n").unwrap();
        self.out.write(b"    if ptr not in tape:\n").unwrap();
        self.out.write(b"        tape[ptr] = 0\n").unwrap();
        self.out.write(b"    tape[ptr] -= 1\n\n").unwrap();
        self.out.write(b"def read_stdin():\n").unwrap();
        self.out.write(b"    global current_input\n").unwrap();
        self.out
            .write(b"    if len(current_input) == 0:\n")
            .unwrap();
        self.out
            .write(b"        current_input = input()\n\n")
            .unwrap();
        self.out.write(b"    if len(current_input) > 0:\n").unwrap();
        self.out
            .write(b"        tape[ptr] = ord(current_input[0])\n")
            .unwrap();
        self.out
            .write(b"        current_input = current_input[1:]\n\n")
            .unwrap();
        self.out.write(b"    else:\n").unwrap();
        self.out.write(b"        tape[ptr] = 0\n\n").unwrap();
        self.out.write(b"def should_loop():\n").unwrap();
        self.out.write(b"    if ptr not in tape:\n").unwrap();
        self.out.write(b"        return False\n").unwrap();
        self.out.write(b"    return tape[ptr] != 0\n\n").unwrap();

        for n in node.nodes.iter() {
            self.visit_node(n);
        }

        self.out.write(b"printf(\"\\n\");").unwrap();
        self.out.write(b"return 0;").unwrap();
        self.out.write(b"}\n").unwrap();
    }
}
