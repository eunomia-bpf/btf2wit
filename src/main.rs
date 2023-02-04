use std::io::Write;

use anyhow::anyhow;
use btf::types::{Btf, BtfType};
use object::ElfFile;
fn main() -> anyhow::Result<()> {
    let file = std::fs::read("source.bpf.o")?;
    let elf: ElfFile = object::ElfFile::parse(&file[..]).map_err(|e| anyhow!("{}", e))?;
    let btf = Btf::load(&elf).map_err(|e| anyhow!("{}", e))?;
    let types = btf.types();

    let mut out_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("out.h")
        .map_err(|e| anyhow!("{}", e))?;
    for ty in types {
        if let BtfType::Struct(st) = ty {
            writeln!(out_file, "struct {} {{", st.name)?;
            for member in st.members.iter() {
                let real_type = &types[member.type_id as usize];
                if let BtfType::Array(array_info) = real_type {
                    // array_info.
                    let val_type = &types[array_info.val_type_id as usize];
                    writeln!(
                        out_file,
                        "    {} {}[{}];",
                        val_type.name(),
                        member.name,
                        array_info.nelems
                    )?;
                } else if let BtfType::Ptr(ptr_info) = real_type {
                    let val_type = &types[ptr_info.type_id as usize];
                    writeln!(
                        out_file,
                        "    {} *{};",
                        if let BtfType::Void = val_type {
                            "void"
                        } else {
                            val_type.name()
                        },
                        member.name
                    )?;
                } else {
                    writeln!(out_file, "    {} {};", real_type.name(), member.name)?;
                }
            }
            writeln!(out_file, "}};")?;
        }
    }
    return Ok(());
}
