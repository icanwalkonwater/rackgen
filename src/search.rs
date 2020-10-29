use std::collections::HashSet;
use std::ffi::{OsStr, CStr};
use std::fs::File;
use std::os::unix::ffi::OsStrExt;

use elfkit::section::SectionContent;
use elfkit::types::SymbolType;
use sharedlib::{Lib, Data, Symbol};
use std::os::raw::c_char;
use crate::sig::{FunSig, parse_sig};

const SYM_SECTION_NAME: &str = ".dynsym";

pub fn search_api_funcs(mut file: File) -> anyhow::Result<Vec<String>> {
    let mut elf = elfkit::Elf::from_reader(&mut file).unwrap();

    let section_index = elf
        .sections
        .iter()
        .position(|section| OsStr::from_bytes(&section.name) == SYM_SECTION_NAME)
        .ok_or(anyhow::Error::msg("Can't find symbol table !"))?;

    elf.load(section_index, &mut file)
        .map_err(|_| anyhow::Error::msg("Failed to load section"))?;

    let section = &elf.sections[section_index];

    let mut api_sigs = HashSet::new();
    let mut api_funcs = Vec::new();

    if let SectionContent::Symbols(symbols) = &section.content {
        for sym in symbols {
            let name = String::from_utf8_lossy(&sym.name).into_owned();

            if name.starts_with("api_") {
                if name.ends_with("_sig") {
                    api_sigs.insert(name);
                } else if let SymbolType::FUNC = sym.stype {
                    api_funcs.push(name);
                }
            }
        }
    } else {
        return Err(anyhow::Error::msg("What the fuck"));
    }

    let api_funcs = api_funcs
        .into_iter()
        .filter(|name| api_sigs.contains(&format!("{}_sig", name)))
        .collect::<Vec<_>>();

    Ok(api_funcs)
}

pub fn search_api_fun_sig(lib: &str, funcs: &[String]) -> anyhow::Result<Vec<FunSig>> {
    let mut res = Vec::new();

    unsafe {
        let lib = Lib::new(lib).expect("Failed to load lib");

        for func in funcs {
            let symbol_sig = format!("{}_sig", func);
            let sig: Data<*const c_char> = lib.find_data(symbol_sig).expect("Failed to find sig");
            let str = CStr::from_ptr(*sig.get()).to_string_lossy();

            res.push(parse_sig(func, &str)?);
        }
    }

    Ok(res)
}
