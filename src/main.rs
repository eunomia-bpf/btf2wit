/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
 *
 * Copyright (c) 2023, eunomia-bpf
 * All rights reserved.
 */

use std::io::{stdout, Write};

use anyhow::anyhow;
use btf::types::Btf;
use btf_types::GenerateArgs;
use clap::Parser;
use gen::generate_wit;
use object::ElfFile;

pub mod btf_types;
pub mod gen;
#[cfg(test)]
pub mod tests;

fn pointer_size_test(s: &str) -> Result<usize, String> {
    let size: usize = s.parse().map_err(|_| format!("`{}` isn't a integer", s))?;
    match size {
        s @ (32 | 64) => Ok(s),
        s => Err(format!(
            "Invalid pointer size: {} (only 32 or 64 are accepted)",
            s
        )),
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    input_file: String,
    #[arg(short, long, value_name = "OUT_FILE")]
    output_file: Option<String>,
    #[arg(short = 'p', long, value_parser = pointer_size_test, default_value_t = 32)]
    pointer_size: usize,
    #[arg(short, long, default_value_t = String::from("host"), help = "Specify the default world name in the output wit file")]
    world_name: String,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let input_file =
        std::fs::read(args.input_file).map_err(|e| anyhow!("Failed to open input file: {}", e))?;
    let elf: ElfFile = object::ElfFile::parse(&input_file[..])
        .map_err(|e| anyhow!("Failed to parse input file as ELF: {}", e))?;
    let btf = Btf::load(&elf).map_err(|e| anyhow!("Failed to read BTF section: {}", e))?;
    let out_buf = generate_wit(
        &btf,
        GenerateArgs::default()
            .pointer_size(args.pointer_size)
            .world_name(args.world_name),
    )?;
    if let Some(out_file) = args.output_file {
        std::fs::write(out_file, out_buf).map_err(|e| anyhow!("Failed to write output: {}", e))?;
    } else {
        stdout()
            .write(&out_buf[..])
            .map_err(|e| anyhow!("Failed to write standard out: {}", e))?;
    };
    return Ok(());
}
