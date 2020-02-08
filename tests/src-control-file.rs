use debcontrol::{next_paragraph, Error, Paragraph};

static INPUT: &str = include_str!("control");

struct It<'a> {
    input: &'a str,
}

impl<'a> Iterator for It<'a> {
    type Item = Result<Paragraph<'a>, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match next_paragraph(self.input) {
            Ok((rest, Some(paragraph))) => {
                self.input = rest;
                Some(Ok(paragraph))
            }
            Ok((rest, None)) => {
                self.input = rest;
                None
            }
            Err(err) => Some(Err(err)),
        }
    }
}

#[test]
fn should_parse_control_file() {
    let mut iterator = It { input: INPUT };

    let package_names = iterator
        .take_while(Result::is_ok)
        .map(Result::unwrap)
        .flat_map(|paragraph: Paragraph| {
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
