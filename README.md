# debcontrol -- parse Debian control files

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

## License
Copyright (c) 2020 Felix Krull

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.