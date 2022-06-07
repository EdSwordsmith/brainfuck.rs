use std::str::Chars;

#[derive(Debug)]
pub enum OperationType {
    IncrementValue,
    DecrementValue,
    IncrementPointer,
    DecrementPointer,
    Print,
    Read,
}

#[derive(Debug)]
pub enum Node {
    Operation(OperationType),
    Loop(Vec<Node>),
}

fn parse_loop(chars: &mut Chars) -> Node {
    let mut nodes = Vec::new();

    while let Some(token) = chars.next() {
        if token == ']' {
            return Node::Loop(nodes);
        }

        if let Some(node) = parse_token(chars, token) {
            nodes.push(node);
        }
    }

    // TODO: replace with Result to return error
    unreachable!();
}

fn parse_token(chars: &mut Chars, current: char) -> Option<Node> {
    match current {
        '+' => Some(Node::Operation(OperationType::IncrementValue)),
        '-' => Some(Node::Operation(OperationType::DecrementValue)),
        '>' => Some(Node::Operation(OperationType::IncrementPointer)),
        '<' => Some(Node::Operation(OperationType::DecrementPointer)),
        '.' => Some(Node::Operation(OperationType::Print)),
        ',' => Some(Node::Operation(OperationType::Read)),
        '[' => Some(parse_loop(chars)),
        _ => None,
    }
}

pub fn parse(input: impl AsRef<str>) -> Vec<Node> {
    let mut chars = input.as_ref().chars();
    let mut nodes = Vec::new();

    while let Some(token) = chars.next() {
        if let Some(node) = parse_token(&mut chars, token) {
            nodes.push(node);
        }
    }

    nodes
}
