use debcontrol::parse_str;

static INPUT: &str = include_str!("control");

#[test]
fn should_parse_control_file() {
    let package_names = parse_str(INPUT)
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
