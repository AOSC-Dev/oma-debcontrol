use anyhow::{anyhow, Context};
use debcontrol::{parse_complete, Paragraph};
use json::JsonValue;
use std::{env::args_os, fs::read_to_string, io::stdout, path::PathBuf};

fn paragraph_to_json(paragraph: Paragraph) -> JsonValue {
    let mut obj = JsonValue::new_object();
    for field in paragraph.fields {
        obj.insert(field.name, JsonValue::String(field.value))
            .unwrap();
    }
    obj
}

fn main() -> anyhow::Result<()> {
    let filename: PathBuf = args_os()
        .skip(1)
        .next()
        .context("specify a file name")?
        .into();

    let input = read_to_string(filename)?;
    let json: JsonValue = parse_complete(&input)
        .map_err(|x| anyhow!("{}", x))?
        .into_iter()
        .map(paragraph_to_json)
        .collect::<Vec<_>>()
        .into();

    json.write_pretty(&mut stdout(), 2)?;
    Ok(())
}
