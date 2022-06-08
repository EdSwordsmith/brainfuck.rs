use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::{fs::read_to_string, path::PathBuf};

use crate::ast::Visitor;

mod ast;
mod parser;
mod c_writer;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    input: PathBuf,
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = read_to_string(cli.input).with_context(|| format!("Error reading source file"))?;
    let ast = parser::parse(input)?;

    let mut writer = c_writer::CWriter { out: std::io::stdout() };
    writer.visit_node(&ast);

    Ok(())
}
