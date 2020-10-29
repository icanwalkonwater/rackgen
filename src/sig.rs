use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct FunSig {
    pub name: String,
    pub ret: ArgTy,
    pub args: Vec<ArgTy>,
}

#[derive(Debug, Copy, Clone)]
pub enum ArgTy {
    Buff,
    Number,
}

impl Display for ArgTy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgTy::Buff => f.write_str("buf"),
            ArgTy::Number => f.write_str("f64"),
        }
    }
}

pub fn parse_sig(name: &str, sig: &str) -> anyhow::Result<FunSig> {
    let ret = sig.chars().take_while(|&c| c != ';').collect::<String>();
    let args = sig[ret.len() + 1..].split(" ").collect::<Vec<_>>();

    Ok(FunSig {
        name: name.to_string(),
        ret: parse_ty(&ret),
        args: args.iter().map(|a| parse_ty(a)).collect(),
    })
}

fn parse_ty(ty: &str) -> ArgTy {
    match ty {
        "buf" => ArgTy::Buff,
        _ => ArgTy::Number,
    }
}
