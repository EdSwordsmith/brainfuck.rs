use anyhow::{Context, Ok, Result};
use asm_writer::AsmWriter;
use ast::ProgramNode;
use c_writer::CWriter;
use clap::Parser;
use interpreter::Interpreter;
use python_writer::PythonWriter;
use std::{fs, io::Write, path::PathBuf};

use crate::ast::Visitor;

mod ast;
mod c_writer;
mod parser;
mod python_writer;
mod interpreter;
mod asm_writer;
mod scanner;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    input: PathBuf,
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input =
        fs::read_to_string(cli.input).with_context(|| format!("Error reading source file"))?;

    let tokens = scanner::tokenize(input);
    let ast = parser::parse(tokens)?;

    let out: Box<dyn Write> = if let Some(output) = &cli.output {
        Box::new(fs::File::create(output)?)
    } else {
        Box::new(std::io::stdout())
    };

    let mut visitor: Box<dyn Visitor<ProgramNode>> = if let Some(output) = cli.output {
        if let Some(extension) = output.extension() {
            if extension.eq("c") || extension.eq("cpp") {
                Box::new(CWriter::new(out))
            } else if extension.eq("py") {
                Box::new(PythonWriter::new(out))
            } else if extension.eq("asm") {
                Box::new(AsmWriter::new(out))
            } else {
                return Err(anyhow::Error::msg("Unsupported target type"));
            }
        } else {
            unreachable!();
        }
    } else {
        Box::new(Interpreter::new())
    };

    visitor.visit_node(&ast);

    Ok(())
}
