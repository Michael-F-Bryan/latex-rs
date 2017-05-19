# LaTeX-rs

[![Build Status](https://travis-ci.org/Michael-F-Bryan/latex-rs.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/latex-rs)
[![Crates.io](https://img.shields.io/crates/l/latex.svg)](https://crates.io/crates/latex)
[![Docs.rs](https://docs.rs/latex/badge.svg)](https://docs.rs/latex/)
[![latex-rs on Crates.io](https://img.shields.io/crates/v/latex.svg)](https://crates.io/crates/latex)

An ergonomic library for programatically generating LaTeX documents and reports.

This originally came from a desire to create an alternate renderer for [mdbook]
which saves to PDF. I quickly found that I needed a library for programatically
generating LaTeX documents from an AST and because there wasn't anything out
there which suited my use case, I made my own.


## Getting Started

There are some fairly detailed [examples][examples] (plus
[here][equation-examples] and [here][list-examples]) in the documentation,
although this is what your typical "Hello World" would look like:

```rust
use latex::{Document, DocumentClass, Element};

let mut doc = Document::new(DocumentClass::Article);
doc.push("Hello World");
```


## Contributing

This crate is still very young so pull requests and issues are welcome! If
you there's something you want then create an [issue] and I'll try to implement
it.

[issue]: https://github.com/Michael-F-Bryan/latex-rs/issues/new
[mdbook]: https://github.com/azerupi/mdBook
[examples]: https://docs.rs/latex#examples
[equation-examples]: https://docs.rs/latex/struct.Equation.html#examples
[list-examples]: https://docs.rs/latex/struct.List.html#examples
