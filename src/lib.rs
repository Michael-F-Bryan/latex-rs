//! A crate for generating LaTeX documents programatically.
//!
//! The main purpose of this library is to make the job of programatically
//! generating LaTeX reports and documents (which will probably then be
//! compiled to PDF) as easy as possible.
//!
//! This library tries to use Rust's powerful type system to give your document
//! additional semantic meaning and compile-time typesafety. For example,
//! [`Element::ClearPage`] could easily be implemented using
//! `Element::UserDefined(r"\clearpage")`, however it is common enough to
//! justify its own variant to make the generating code easier to read.
//!
//!
//! # Examples
//!
//! Here's how to create a basic document containing a title page, a table of
//! contents, and two sections.
//!
//! ```rust
//! use latex::{DocumentClass, Element, Document, Section, Renderable};
//!
//! # fn run() -> latex::Result<()> {
//! let mut doc = Document::new(DocumentClass::Article);
//!
//! // Set some metadata for the document
//! doc.preamble.title("My Fancy Document");
//! doc.preamble.author("Michael-F-Bryan");
//!
//! doc.push(Element::TitlePage)
//!     .push(Element::ClearPage)
//!     .push(Element::TableOfContents)
//!     .push(Element::ClearPage);
//!
//! let mut section_1 = Section::new("Section 1");
//! section_1.push("Here is some text which will be put in paragraph 1.")
//!          .push("And here is some more text for paragraph 2.");
//! doc.push(section_1);
//!
//! let mut section_2 = Section::new("Section 2");
//! section_2.push("More text...");
//! doc.push(section_2);
//!
//! let mut buffer = Vec::new();
//! doc.render(&mut buffer)?;
//!
//! let rendered = String::from_utf8(buffer)?;
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! This will generate the LaTeX source for you, so all you need to do now is
//! write it to a file and then run your favourite tex build tool on it (I
//! personally use [latexmk]).
//!
//! ```rust,no_run
//! use std::fs::File;
//! use std::io::Write;
//! use std::process::Command;
//!
//! # fn run() -> latex::Result<()> {
//! # let rendered = String::new();
//! // Write our rendered text to a file
//! let mut f = File::open("report.tex")?;
//! write!(f, "{}", rendered)?;
//!
//! // Then call latexmk on it
//! let exit_status = Command::new("latexmk").arg("report.tex").status()?;
//!
//! assert!(exit_status.success());
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! [latexmk]: http://mg.readthedocs.io/latexmk.html
//! [`ClearPage`]: enum.Element.html

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;

mod paragraph;
mod document;
mod section;
mod equations;

pub use errors::*;
pub use document::{Document, DocumentClass, Element, Preamble};
pub use paragraph::{Paragraph, ParagraphElement};
pub use section::Section;
pub use equations::{Equations, Equation};

use std::io::Write;

mod errors {
    error_chain!{
        foreign_links{
            Io(::std::io::Error) #[doc = "Wrapper around `std::io::Error`"];
            Fmt(::std::fmt::Error) #[doc = "A formatting error"];
            UtfError(::std::string::FromUtf8Error) #[doc = "A UTF8 conversion error"];
        }
    }
}

/// A generic trait for rendering AST nodes to some `Writer`.
pub trait Renderable {
    /// Render the item.
    fn render<W>(&self, writer: &mut W) -> Result<()> where W: Write;
}
