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
doc.preamble.title("My Fancy Document")
            .author("Michael-F-Bryan");

doc.push(Element::TitlePage)
   .push(Element::ClearPage)
   .push(Element::TableOfContents)
   .push(Element::ClearPage);

let mut section_1 = Section::new("Section 1");
section_1.push("Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Aenean sagittis ante diam, non blandit arcu dignissim dictum. Ut
interdum dapibus odio, ac sagittis quam euismod a. Fusce auctor tempor
ante, et congue lorem ullamcorper vel. Aliquam vehicula nisl sit amet
orci tempus, non porta libero vestibulum. Aliquam posuere odio sed
tristique scelerisque. Donec eget ex faucibus, placerat neque vitae,
consectetur augue. Etiam sed ex id nibh vulputate tempus id nec nisl.")
    .push("Proin lacinia fringilla elit eu dapibus. In quis metus vel diam
laoreet facilisis. Nullam tincidunt metus eu mi rutrum, sit amet congue
mauris tincidunt. Sed suscipit ornare lacus vitae convallis. Suspendisse
tincidunt, est ut dictum consectetur, erat felis euismod lorem, vel
scelerisque mi justo at dolor. ");

doc.push(section_1);

let mut section_2 = Section::new("Section 2");
section_2.push("Ut quis mauris orci. Vivamus pellentesque sollicitudin libero,
nec pharetra eros dapibus vitae. Mauris nec neque eget nibh feugiat
vestibulum. Vivamus in ante sed purus rhoncus euismod. ");
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
