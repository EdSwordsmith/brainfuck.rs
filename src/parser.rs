use anyhow::Ok;

use crate::{ast::*, scanner::Token};
use std::slice::Iter;

fn parse_loop(tokens: &mut Iter<Token>) -> anyhow::Result<LoopNode> {
    let mut loop_node = LoopNode { nodes: Vec::new() };

    while let Some(token) = tokens.next() {
        match token {
            Token::EndLoop => { return Ok(loop_node); },
            _ => { loop_node.nodes.push(parse_token(tokens, *token)?); },
        }
    }

    Err(anyhow::Error::msg("Expected ] character."))
}

fn parse_token(tokens: &mut Iter<Token>, current: Token) -> anyhow::Result<Node> {
    match current {
        Token::IncValue(value) => Ok(Node::Operation(OperationNode::IncrementValue(value))),
        Token::IncPtr(value) => Ok(Node::Operation(OperationNode::IncrementPointer(value))),
        Token::BeginLoop => Ok(Node::Loop(parse_loop(tokens)?)),
        Token::EndLoop => Err(anyhow::Error::msg("Unexpected ] character.")),
        Token::Print => Ok(Node::Operation(OperationNode::Print)),
        Token::Read => Ok(Node::Operation(OperationNode::Read)),
    }
}

pub fn parse(tokens: Vec<Token>) -> anyhow::Result<ProgramNode> {
    let mut token_stream = tokens.iter();
    let mut nodes = Vec::new();

    while let Some(token) = token_stream.next() {
        nodes.push(parse_token(&mut token_stream, *token)?);
    }

    Ok(ProgramNode { nodes })
}
