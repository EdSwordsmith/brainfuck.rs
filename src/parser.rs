use std::str::Chars;
use crate::ast::*;

fn parse_loop(chars: &mut Chars) -> anyhow::Result<LoopNode> {
    let mut loop_node = LoopNode { nodes: Vec::new() };

    while let Some(token) = chars.next() {
        if token == ']' {
            return Ok(loop_node);
        }

        if let Some(node) = parse_token(chars, token)? {
            loop_node.nodes.push(node);
        }
    }

    Err(anyhow::Error::msg("Expected ] character."))
}

fn parse_token(chars: &mut Chars, current: char) -> anyhow::Result<Option<Node>> {
    match current {
        '+' => Ok(Some(Node::Operation(OperationNode::IncrementValue))),
        '-' => Ok(Some(Node::Operation(OperationNode::DecrementValue))),
        '>' => Ok(Some(Node::Operation(OperationNode::IncrementPointer))),
        '<' => Ok(Some(Node::Operation(OperationNode::DecrementPointer))),
        '.' => Ok(Some(Node::Operation(OperationNode::Print))),
        ',' => Ok(Some(Node::Operation(OperationNode::Read))),
        '[' => Ok(Some(Node::Loop(parse_loop(chars)?))),
        ']' => Err(anyhow::Error::msg("Unexpected ] character.")),
        _ => Ok(None),
    }
}

pub fn parse(input: impl AsRef<str>) -> anyhow::Result<ProgramNode> {
    let mut chars = input.as_ref().chars();
    let mut nodes = Vec::new();

    while let Some(token) = chars.next() {
        if let Some(node) = parse_token(&mut chars, token)? {
            nodes.push(node);
        }
    }

    Ok(ProgramNode { nodes })
}
