use oma_debcontrol::{parse_str, BufParse, BufParseInput, Streaming};
use std::{
    fs::{read_to_string, File},
    io::Read as IoRead,
    path::PathBuf,
};

fn data_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("control")
}

#[test]
fn should_parse_control_file() {
    let input = read_to_string(data_file()).unwrap();
    let package_names = parse_str(&input)
        .unwrap()
        .into_iter()
        .flat_map(|paragraph| {
            paragraph
                .fields
                .into_iter()
                .find(|f| f.name == "Package")
                .map(|f| f.value)
                .into_iter()
        })
        .collect::<Vec<_>>();

    assert_eq!(
        package_names,
        vec![
            "gir1.2-ostree-1.0",
            "libostree-1-1",
            "libostree-dev",
            "libostree-doc",
            "ostree",
            "ostree-boot",
            "ostree-tests",
        ]
    );
}

struct FileWrapper(File);

impl BufParseInput for FileWrapper {
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        IoRead::read(&mut self.0, buf)
    }
}

#[test]
fn should_parse_control_file_streaming() {
    let read = FileWrapper(File::open(data_file()).unwrap());
    let mut parser = BufParse::new(read, 64);

    let mut package_names: Vec<String> = Vec::new();
    while let Some(result) = parser.try_next().unwrap() {
        match result {
            Streaming::Item(paragraph) => {
                let maybe_field = paragraph.fields.into_iter().find(|f| f.name == "Package");
                if let Some(field) = maybe_field {
                    package_names.push(field.value);
                }
            }
            Streaming::Incomplete => {
                parser.buffer().unwrap();
            }
        }
    }

    assert_eq!(
        package_names,
        vec![
            "gir1.2-ostree-1.0",
            "libostree-1-1",
            "libostree-dev",
            "libostree-doc",
            "ostree",
            "ostree-boot",
            "ostree-tests",
        ]
    );
}
