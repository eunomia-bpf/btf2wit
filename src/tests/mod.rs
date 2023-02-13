/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
 *
 * Copyright (c) 2023, eunomia-bpf
 * All rights reserved.
 */

use btf::types::Btf;
use object::ElfFile;

use crate::{btf_types::GenerateArgs, gen::generate_wit};

#[test]
fn test_nested_array() {
    let elf_bytes = include_bytes!("nested-array.bpf.o");
    let elf: ElfFile = object::ElfFile::parse(elf_bytes).unwrap();
    let btf = Btf::load(&elf).unwrap();
    let out = generate_wit(&btf, GenerateArgs::default().pointer_size(64)).unwrap();
    let expected_out = include_bytes!("nested-array-out.txt");
    assert_eq!(out, expected_out);
}
