use anyhow::anyhow;
use oma_debcontrol::{BufParse, Paragraph, Streaming};
use std::{
    env::args_os,
    ffi::OsString,
    fmt::Display,
    fs::File,
    io::{stdin, Read},
};

fn stringify<E: Display>(error: E) -> anyhow::Error {
    anyhow!("{}", error)
}

fn print_paragraph(p: &Paragraph) {
    for field in &p.fields {
        print!("{}:", field.name);
        for line in field.value.lines() {
            println!(" {}", line);
        }
    }
    println!();
}

fn get_input(arg: Option<OsString>) -> anyhow::Result<Box<dyn Read>> {
    let input: Box<dyn Read> = match arg {
        None => Box::new(stdin()),
        Some(filename) => Box::new(File::open(filename)?),
    };
    Ok(input)
}

fn main() -> anyhow::Result<()> {
    let arg = args_os().skip(1).next();
    let mut parse = BufParse::new(get_input(arg)?, 4096);

    while let Some(result) = parse.try_next().map_err(stringify)? {
        match result {
            Streaming::Item(paragraph) => print_paragraph(&paragraph),
            Streaming::Incomplete => parse.buffer()?,
        }
    }

    Ok(())
}
