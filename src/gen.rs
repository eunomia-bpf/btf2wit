/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
 *
 * Copyright (c) 2023, eunomia-bpf
 * All rights reserved.
 */

use crate::btf_types::BtfUtils;
use btf::types::{Btf, BtfType};
use std::io::Write;

pub fn generate_wit<'a>(btf: &Btf<'a>) -> anyhow::Result<Vec<u8>> {
    let mut out_buf = Vec::<u8>::new();
    let mut btf_util = BtfUtils::new(&btf);
    let types = btf.types();

    writeln!(out_buf, "default world host {{")?;

    for (idx, ty) in types.iter().enumerate() {
        match ty {
            BtfType::Struct(_) | BtfType::Union(_) | BtfType::Enum(_) | BtfType::Func(_) => write!(
                out_buf,
                "{}",
                btf_util.generate_top_level_string(idx as u32, 4)?
            )?,
            _ => {}
        }
    }
    writeln!(out_buf, "}}")?;
    return Ok(out_buf);
}
