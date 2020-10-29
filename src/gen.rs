use std::ops::Index;

use regex::Regex;

use crate::sig::{ArgTy, FunSig};

pub fn gen_js(libname: &str, pretty_libname: &str, funcs: &[FunSig]) -> String {
    let mut buffer = String::new();

    buffer.push_str(&format!("class {} {{\n", pretty_libname));

    // *** Gen constructor

    buffer.push_str("  constructor() {\n");
    buffer.push_str(&format!("    this.#soId = loadLib(\"{}\");\n", libname));
    buffer.push_str("  }\n\n");

    // *** Gen methods

    let snake_to_camel_sep_regex = Regex::new("_(.)").unwrap();
    for fun in funcs {
        // Gen camel case name from snake case
        let camel_case_name = {
            let cleaned_name = &fun.name[4..];

            let mut captures = snake_to_camel_sep_regex
                .captures_iter(cleaned_name)
                .map(|m| m.index(1).to_uppercase())
                .collect::<Vec<_>>();

            snake_to_camel_sep_regex
                .split(cleaned_name)
                .fold(String::new(), |mut acc, item| {
                    acc.push_str(item);
                    if !captures.is_empty() {
                        acc.push_str(&captures.remove(0));
                    }
                    acc
                })
        };

        // Gen arguments for signature
        let js_sig = (0..fun.args.len())
            .into_iter()
            .map(|i| format!("arg{}", i))
            .collect::<Vec<_>>()
            .join(", ");

        // Build the method
        buffer.push_str(&format!("  {}({}) {{\n", camel_case_name, js_sig));

        // Gen type checks
        for (i, arg) in fun.args.iter().enumerate() {
            let constructor_type = match arg {
                ArgTy::Buff => "String",
                ArgTy::Number => "Number",
            };

            buffer.push_str(&format!(
                "    if (arg{}.constructor !== {}) throw TypeError();\n",
                i, constructor_type
            ));
        }

        // Gen call
        buffer.push_str(&format!(
            "    return call(this.#soId, \"{}\", \"{}\", [\n",
            &fun.name, &fun.ret
        ));

        // Gen call arguments
        for (i, arg) in fun.args.iter().enumerate() {
            match arg {
                ArgTy::Buff => {
                    buffer.push_str(&format!("      [ \"len\", arg{}.length ],\n", i));
                    buffer.push_str(&format!("      [ \"buf\", arg{} ],\n", i));
                }
                ArgTy::Number => {
                    buffer.push_str(&format!("      [ \"f64\", arg{} ],\n", i));
                }
            }
        }

        buffer.push_str("    ]);\n");
        buffer.push_str("  }\n\n");
    }

    buffer.push_str("}");
    buffer
}
