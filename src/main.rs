use std::io::{stdout, Write};

use anyhow::anyhow;
use btf::types::Btf;
use clap::Parser;
use gen::generate_wit;
use object::ElfFile;

pub mod btf_types;
pub mod gen;
#[cfg(test)]
pub mod tests;
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    input_file: String,
    #[arg(short, long, value_name = "OUT_FILE")]
    output_file: Option<String>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let input_file =
        std::fs::read(args.input_file).map_err(|e| anyhow!("Failed to open input file: {}", e))?;
    let elf: ElfFile = object::ElfFile::parse(&input_file[..])
        .map_err(|e| anyhow!("Failed to parse input file as ELF: {}", e))?;
    let btf = Btf::load(&elf).map_err(|e| anyhow!("Failed to read BTF section: {}", e))?;
    let out_buf = generate_wit(&btf)?;
    if let Some(out_file) = args.output_file {
        std::fs::write(out_file, out_buf).map_err(|e| anyhow!("Failed to write output: {}", e))?;
    } else {
        stdout()
            .write(&out_buf[..])
            .map_err(|e| anyhow!("Failed to write standard out: {}", e))?;
    };
    return Ok(());
}
