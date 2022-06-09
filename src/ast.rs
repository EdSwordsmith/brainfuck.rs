#[derive(Debug)]
pub enum OperationNode {
    IncrementValue(i16),
    IncrementPointer(isize),
    Print,
    Read,
}

#[derive(Debug)]
pub struct LoopNode {
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
    Operation(OperationNode),
    Loop(LoopNode),
}

#[derive(Debug)]
pub struct ProgramNode {
    pub nodes: Vec<Node>,
}

pub trait Visitor<T> {
    fn visit_node(&mut self, _node: &T) {}
}
