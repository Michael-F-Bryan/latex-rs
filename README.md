# LaTeX-rs

[![Build Status](https://travis-ci.org/Michael-F-Bryan/latex-rs.svg?branch=master)](https://travis-ci.org/Michael-F-Bryan/latex-rs)
[![Crates.io](https://img.shields.io/crates/l/latex.svg)](https://crates.io/crates/latex)
[![Docs.rs](https://docs.rs/latex/badge.svg)](https://docs.rs/latex/)
[![latex-rs on Crates.io](https://img.shields.io/crates/v/latex.svg)](https://crates.io/crates/latex)
[![Build
status](https://ci.appveyor.com/api/projects/status/eca0h1nrk6nq3xwo?svg=true)](https://ci.appveyor.com/project/Michael-F-Bryan/latex-rs)


An ergonomic library for programatically generating LaTeX documents and reports.

This originally came from a desire to create an alternate renderer for [mdbook]
which saves to PDF. I quickly found that I needed a library for programatically
generating LaTeX documents from an AST and because there wasn't anything out
there which suited my use case, I made my own.


## Getting Started

Most of the types used to construct a `Document` have examples showing how they
can be used and roughly what they'll generate, however for a more in-depth
example check out the [complex example] in the `examples/` directory.

This is what your typical "Hello World" would look like:

```rust
use latex::{Document, DocumentClass, Element};

let mut doc = Document::new(DocumentClass::Article);
doc.push("Hello World");
```


## Features

The crate is still incomplete, but the following features are available when
generating your `LaTeX` documents:

- [x] Preamble
- [x] Sections
- [x] Paragraphs
- [x] Align environment and Equations
- [x] Lists (both numbered and not)
- [x] Table of contents, title page, and the `\clearpage` command
- [ ] Figures
- [ ] Tables
- [ ] Appendices
- [ ] Included PDF files
- [ ] `\input{...}` and `\include{...}`
- [ ] Partial documents
- [ ] References and Bibliography
- [ ] labels, plus `\ref{...}` for referencing them


## Contributing

This crate is still very young so pull requests and issues are welcome! If
you there's something you want then create an [issue] and I'll try to implement
it.


[issue]: https://github.com/Michael-F-Bryan/latex-rs/issues/new
[mdbook]: https://github.com/azerupi/mdBook
[complex example]: https://github.com/Michael-F-Bryan/latex-rs/blob/master/examples/complex.rs

