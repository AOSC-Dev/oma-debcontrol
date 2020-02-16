# `debcontrol` — parse Debian control files

[![pipeline status](https://gitlab.com/fkrull/debcontrol-rs/badges/master/pipeline.svg)](https://gitlab.com/fkrull/debcontrol-rs/-/commits/master)
[![crates.io version](https://img.shields.io/crates/v/debcontrol.svg)](https://crates.io/crates/debcontrol)
[![docs.rs version](https://docs.rs/debcontrol/badge.svg)](https://docs.rs/debcontrol)

A Rust crate for parsing [Debian control files].

[Debian control files]: https://www.debian.org/doc/debian-policy/ch-controlfields.html

## Usage
Parse a complete control file:

```rust
use debcontrol::{Paragraph, parse_str};

let paragraphs: Vec<Paragraph> = parse_str("
a-field: with a value
another-field: with a...
 ...continuation

# a comment
this-is: another paragraph
")?;
```

See the [documentation] for more examples and reference documentation.

[documentation]: https://docs.rs/debcontrol

## Developing

### Releases
* bump the version in `Cargo.toml`
* tag the commit as `v<VERSION>`, e.g. `v0.1.0`
