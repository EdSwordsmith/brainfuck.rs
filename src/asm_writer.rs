use std::io::Write;

use crate::ast::*;

pub struct AsmWriter<T: Write> {
    out: T,
    lbl: usize,
}

impl<T: Write> AsmWriter<T> {
    pub fn new(out: T) -> Self {
        AsmWriter { out, lbl: 0 }
    }
}

impl<T: Write> Visitor<OperationNode> for AsmWriter<T> {
    fn visit_node(&mut self, node: &OperationNode) {
        match node {
            OperationNode::IncrementValue(value) => self.out.write_fmt(format_args!("    add byte[rbx], {}\n", value)).unwrap(),
            OperationNode::IncrementPointer(value) => self.out.write_fmt(format_args!("    add rbx, {}\n", value)).unwrap(),
            OperationNode::Print => {self.out.write(b"    call write\n").unwrap();},
            OperationNode::Read => { self.out.write(b"    call read\n").unwrap();},
        };
    }
}

impl<T: Write> Visitor<LoopNode> for AsmWriter<T> {
    fn visit_node(&mut self, node: &LoopNode) {
        self.lbl += 1;
        let start_lbl = self.lbl;
        self.lbl += 1;
        let end_lbl = self.lbl;

        self.out.write_fmt(format_args!(".L{}:\n", start_lbl)).unwrap();
        self.out.write(b"    cmp byte[rbx], 0\n").unwrap();
        self.out.write_fmt(format_args!("    jz .L{}\n", end_lbl)).unwrap();

        for n in node.nodes.iter() {
            self.visit_node(n);
        }

        self.out.write_fmt(format_args!("    jmp .L{}\n", start_lbl)).unwrap();
        self.out.write_fmt(format_args!(".L{}:\n", end_lbl)).unwrap();
    }
}

impl<T: Write> Visitor<Node> for AsmWriter<T> {
    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Operation(op_node) => self.visit_node(op_node),
            Node::Loop(loop_node) => self.visit_node(loop_node),
        };
    }
}

impl<T: Write> Visitor<ProgramNode> for AsmWriter<T> {
    fn visit_node(&mut self, node: &ProgramNode) {
        self.out.write(b"global _start\n").unwrap();

        self.out.write(b"section .text\n").unwrap();

        self.out.write(b"write:\n").unwrap();
        self.out.write(b"    mov rax, 0x2000004\n").unwrap();
        self.out.write(b"    mov rdi, 1\n").unwrap();
        self.out.write(b"    mov rsi, rbx\n").unwrap();
        self.out.write(b"    mov rdx, 1\n").unwrap();
        self.out.write(b"    syscall\n").unwrap();
        self.out.write(b"    ret\n").unwrap();

        self.out.write(b"read:\n").unwrap();
        self.out.write(b"    mov rax, 0x2000003\n").unwrap();
        self.out.write(b"    mov rdi, 0\n").unwrap();
        self.out.write(b"    mov rsi, rbx\n").unwrap();
        self.out.write(b"    mov rdx, 1\n").unwrap();
        self.out.write(b"    syscall\n").unwrap();
        self.out.write(b"    ret\n").unwrap();

        self.out.write(b"_start:\n").unwrap();
        self.out.write(b"    mov rbx, tape\n").unwrap();

        for n in node.nodes.iter() {
            self.visit_node(n);
        }

        self.out.write(b"   mov rax, 0x2000001\n").unwrap();
        self.out.write(b"   xor rdi, rdi\n").unwrap();
        self.out.write(b"   syscall\n").unwrap(); 

        self.out.write(b"section .bss\n").unwrap();
        self.out.write(b"tape:\n").unwrap();
        self.out.write(b"   resb 30000\n").unwrap();
    }
}
