/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
 *
 * Copyright (c) 2023, eunomia-bpf
 * All rights reserved.
 */

use anyhow::anyhow;
use btf::types::{Btf, BtfType};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone, Default)]
pub struct TypeAttribute {
    pub const_: bool,
    pub volatile: bool,
    pub restrict: bool,
}
// #[derive(Debug,Clone)]
// pub struct Edge<'a>{
//     to:u32,
//     ty:&'a BtfType<'a>

// }
#[derive(Clone, Debug)]
pub struct CollapsedArray {
    pub dim: Vec<u32>,
    pub val_type: u32,
}
#[allow(dead_code)]
pub struct BtfUtils<'a> {
    btf: &'a Btf<'a>,
    lookup_cache: HashMap<u32, (u32, TypeAttribute)>,
    array_cache: HashMap<u32, CollapsedArray>,
    name_cache: HashMap<u32, String>,
}
fn write_indent(str: &mut String, indent: usize) {
    for _ in 0..indent {
        str.push(' ');
    }
}
fn replace_underscores(s: impl Into<String>) -> String {
    let t: String = s.into();
    return t.replace("_", "-");
}
impl<'a> BtfUtils<'a> {
    pub fn new(btf: &'a Btf<'a>) -> Self {
        Self {
            btf,
            lookup_cache: Default::default(),
            array_cache: Default::default(),
            name_cache: Default::default(),
        }
    }
    pub fn generate_top_level_string(
        &mut self,
        type_id: u32,
        mut indent: usize,
    ) -> anyhow::Result<String> {
        let mut ret = String::new();
        let ty = self
            .btf
            .types()
            .get(type_id as usize)
            .ok_or_else(|| anyhow!("Invalid type: {}", type_id))?;
        match ty {
            BtfType::Struct(st) | BtfType::Union(st) => {
                write_indent(&mut ret, indent);
                writeln!(
                    ret,
                    "{} {} {{",
                    if st.is_struct { "record" } else { "union" },
                    replace_underscores(st.name)
                )?;
                for member in st.members.iter() {
                    indent += 4;
                    write_indent(&mut ret, indent);
                    writeln!(
                        ret,
                        "{:>4}: {},",
                        replace_underscores(member.name),
                        self.generate_string(member.type_id)?
                    )?;
                    indent -= 4;
                }

                write_indent(&mut ret, indent);
                writeln!(ret, "}}")?;
            }
            BtfType::Enum(enum_) => {
                write_indent(&mut ret, indent);
                writeln!(ret, "enum {} {{", replace_underscores(enum_.name))?;
                for value in enum_.values.iter() {
                    indent += 4;
                    write_indent(&mut ret, indent);
                    writeln!(ret, "{},", replace_underscores(value.name))?;
                    indent -= 4;
                }
                write_indent(&mut ret, indent);
                writeln!(ret, "}}")?;
            }
            BtfType::Typedef(typedef) => {
                write_indent(&mut ret, indent);
                writeln!(
                    ret,
                    "type {} = {}",
                    replace_underscores(typedef.name),
                    self.generate_string(typedef.type_id)?
                )?;
            }
            BtfType::Func(func) => {
                // Ignore anonymous functions
                if !func.name.is_empty() {
                    write_indent(&mut ret, indent);
                    writeln!(
                        ret,
                        "import {}: {} /* linkage: {} */",
                        func.name,
                        self.generate_string(func.proto_type_id)?,
                        func.kind
                    )?;
                }
            }
            tp => return Err(anyhow!("Unexpected top level type: {}", tp)),
        };
        return Ok(ret);
    }
    pub fn generate_string(&mut self, type_id: u32) -> anyhow::Result<String> {
        if let Some(v) = self.name_cache.get(&type_id) {
            return Ok(v.clone());
        }
        let ty = self
            .btf
            .types()
            .get(type_id as usize)
            .ok_or_else(|| anyhow!("Invalid type: {}", type_id))?;
        let mut ret = String::new();
        match ty {
            BtfType::Fwd(_) => {
                return Err(anyhow!("Forwards are not supported!"));
            }
            BtfType::Func(_) => return Err(anyhow!("FuncDef is not supported!")),

            BtfType::Var(_) => return Err(anyhow!("Var is not supported!")),
            BtfType::Datasec(_) => return Err(anyhow!("DataSec is not supported!")),
            BtfType::DeclTag(_) => return Err(anyhow!("DeclTag is not supported!")),
            BtfType::TypeTag(_) => return Err(anyhow!("TypeTag is not supported!")),
            BtfType::Typedef(typedef) => {
                return Err(anyhow!(
                    "Unexpected typedef {} at non-top level!",
                    typedef.name
                ));
            }
            BtfType::Void => {
                write!(ret, "()")?;
            }
            BtfType::Ptr(ptr) => {
                write!(
                    ret,
                    "u64 /* pointer to <{}> */",
                    self.generate_string(ptr.type_id)?
                )?;
                // return Err(anyhow!("Pointers are not supported!"));
            }
            BtfType::Int(int) => {
                match int.encoding {
                    btf::types::BtfIntEncoding::None
                    | btf::types::BtfIntEncoding::Char
                    | btf::types::BtfIntEncoding::Bool => write!(ret, "u")?,
                    btf::types::BtfIntEncoding::Signed => write!(ret, "s")?,
                };
                match int.bits {
                    bits @ (8 | 16 | 32 | 64) => {
                        write!(ret, "{}", bits)?;
                    }
                    b => {
                        return Err(anyhow!(
                            "Unsupported integer bits {} for type {}",
                            b,
                            type_id
                        ))
                    }
                }
            }

            BtfType::Array(_) => {
                let collapsed_arr = self.lookup_array(type_id)?;
                for _ in 0..collapsed_arr.dim.len() {
                    write!(ret, "list<")?;
                }
                write!(ret, "{}", self.generate_string(collapsed_arr.val_type)?)?;
                for _ in 0..collapsed_arr.dim.len() {
                    write!(ret, ">")?;
                }
            }

            BtfType::Volatile(v) => {
                write!(ret, "/* volatile */{}", self.generate_string(v.type_id)?)?
            }
            BtfType::Const(v) => write!(ret, "/* const */{}", self.generate_string(v.type_id)?)?,
            BtfType::Restrict(v) => {
                write!(ret, "/* restrict */{}", self.generate_string(v.type_id)?)?
            }
            BtfType::FuncProto(proto) => {
                write!(ret, "func (")?;
                for (i, arg) in proto.params.iter().enumerate() {
                    write!(
                        ret,
                        "{} {}",
                        if arg.name.is_empty() {
                            "".to_string()
                        } else {
                            format!("{}:", replace_underscores(arg.name))
                        },
                        self.generate_string(arg.type_id)?
                    )?;
                    if i != proto.params.len() - 1 {
                        write!(ret, ",")?;
                    }
                }
                write!(ret, ")")?;
                if proto.res_type_id != 0 {
                    write!(ret, " -> {}", self.generate_string(proto.res_type_id)?)?;
                }
            }
            BtfType::Float(float) => {
                write!(ret, "float")?;
                match float.sz {
                    bytes @ (4 | 8) => write!(ret, "{}", bytes * 8)?,
                    b => {
                        return Err(anyhow!(
                            "Unsupported float bytes {} for type {}",
                            b,
                            type_id
                        ))
                    }
                }
            }
            BtfType::Struct(st) | BtfType::Union(st) => {
                write!(ret, "{}", replace_underscores(st.name))?
            }
            BtfType::Enum(enum_) => {
                write!(ret, "{}", replace_underscores(enum_.name))?;
            }
        };
        self.name_cache.insert(type_id, ret.clone());
        return Ok(ret);
    }
    pub fn lookup_array(&mut self, type_id: u32) -> anyhow::Result<CollapsedArray> {
        if let Some(v) = self.array_cache.get(&type_id) {
            return Ok(v.clone());
        } else {
            let mut dim = vec![];
            let mut curr_ty = type_id;
            loop {
                let ty = self
                    .btf
                    .types()
                    .get(curr_ty as usize)
                    .ok_or_else(|| anyhow!("Invalid type during lookup: {}", curr_ty))?;
                if let BtfType::Array(arr) = ty {
                    dim.push(arr.nelems);
                    curr_ty = arr.val_type_id;
                } else {
                    break;
                }
            }
            let v = CollapsedArray {
                dim,
                val_type: curr_ty,
            };
            self.array_cache.insert(type_id, v.clone());
            return Ok(v);
        }
    }
    #[allow(dead_code)]
    pub fn lookup_attr_chain(&mut self, type_id: u32) -> anyhow::Result<(u32, TypeAttribute)> {
        if let Some(v) = self.lookup_cache.get(&type_id) {
            return Ok(v.clone());
        } else {
            let mut curr_ty = type_id;
            let mut attr = TypeAttribute::default();
            let tail_type = loop {
                if let Some(ty) = self.btf.types().get(curr_ty as usize) {
                    (curr_ty, attr) = match ty {
                        BtfType::Volatile(v) => (
                            v.type_id,
                            TypeAttribute {
                                volatile: true,
                                ..attr
                            },
                        ),
                        BtfType::Const(v) => (
                            v.type_id,
                            TypeAttribute {
                                const_: true,
                                ..attr
                            },
                        ),
                        BtfType::Restrict(v) => (
                            v.type_id,
                            TypeAttribute {
                                restrict: true,
                                ..attr
                            },
                        ),
                        _ => break (curr_ty),
                    };
                } else {
                    return Err(anyhow::anyhow!("Invalid type during lookup: {}", curr_ty));
                }
            };
            self.lookup_cache
                .insert(type_id, (tail_type as u32, attr.clone()));
            return Ok((tail_type as u32, attr));
        }
    }
}
