# LaTeX-rs

An ergonomic library for programatically generating LaTeX documents and reports.

This originally came from a desire to create an alternate renderer for [mdbook]
which saves to PDF. I quickly found that I needed a library for programatically
generating LaTeX documents from an AST and because there wasn't anything out
there which suited my use case, I made my own.


## Getting Started

Here's a skeleton document which has a title page, table of contents, and two
sections. You can then `render()` it to any `Writer` (e.g. a File) and pass it
along to whatever you're using to compile LaTeX to PDF.

```rust
extern crate latex;

use latex::{DocumentClass, Element, Document, Section, Renderable};

let mut doc = Document::new(DocumentClass::Article);

// Set some metadata for the document
doc.preamble.title("My Fancy Document");
doc.preamble.author("Michael-F-Bryan");

doc.push(Element::TitlePage)
    .push(Element::ClearPage)
    .push(Element::TableOfContents)
    .push(Element::ClearPage);

let mut section_1 = Section::new("Section 1");
section_1
    .push("Here is some text which will be put in paragraph 1.")
    .push("And here is some more text for paragraph 2.");
doc.push(section_1);

let mut section_2 = Section::new("Section 2");
section_2.push("More text...");
doc.push(section_2);

let mut buffer = vec![];
doc.render(&mut buffer)?;

let rendered = String::from_utf8(buffer)?;

println!("{}", rendered);
```


## Contributing

This crate is still very young so pull requests and issues are welcome! If
you there's something you want then create an [issue] and I'll try to implement
it.

[issue]: insert_issue_tracker_here
