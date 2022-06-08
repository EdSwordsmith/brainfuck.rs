use anyhow::{Context, Ok, Result};
use ast::ProgramNode;
use clap::Parser;
use std::{fs, io::Write, path::PathBuf};

use crate::ast::Visitor;

mod ast;
mod c_writer;
mod parser;
mod python_writer;

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
    let ast = parser::parse(input)?;
    let out: Box<dyn Write> = if let Some(output) = &cli.output {
        Box::new(fs::File::create(output)?)
    } else {
        Box::new(std::io::stdout())
    };

    let mut visitor: Box<dyn Visitor<ProgramNode>> = if let Some(output) = cli.output {
        if let Some(extension) = output.extension() {
            if extension.eq("c") || extension.eq("cpp") {
                Box::new(c_writer::CWriter { out })
            } else if extension.eq("py") {
                Box::new(python_writer::PythonWriter { out, ident: 0 })
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    } else {
        Box::new(c_writer::CWriter { out })
    };

    visitor.visit_node(&ast);

    Ok(())
}
