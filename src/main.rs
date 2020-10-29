use std::fs::File;
use std::path::Path;

use clap::{App, Arg};

mod search;
mod gen;
mod sig;

fn main() -> anyhow::Result<()> {
    let app = App::new("rackgen")
        .arg(
            Arg::with_name("input")
                .help("The library to process")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("class")
                .help("The pretty name of the class to generate.")
                .takes_value(true)
                .short("c")
                .long("class")
                .multiple(false)
                .required(false),
        );

    let matches = app.get_matches();

    if let Some(input_file) = matches.value_of("input") {
        let funcs = search::search_api_funcs(File::open(input_file)?)?;
        let sigs = search::search_api_fun_sig(input_file, &funcs)?;

        let input_file = Path::new(input_file);
        let libname = input_file
            .file_name()
            .ok_or(anyhow::Error::msg("F u"))?
            .to_string_lossy()
            .into_owned();

        let pretty_class = matches
            .value_of("class")
            .map(|s| s.to_owned())
            .unwrap_or_else(|| {
                // Strip extension
                let ext_len = input_file.extension().map_or(0, |e| e.len());
                String::from(&libname[..libname.len() - ext_len-1])
            });

        println!("{}", gen::gen_js(&libname, &pretty_class, &sigs));
    }

    Ok(())
}
