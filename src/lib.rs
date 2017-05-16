//! A crate for generating LaTeX documents programatically.
//!
//! The main purpose of this library is to make the job of programatically
//! generating LaTeX reports and documents (which will probably then be
//! compiled to PDF) as easy as possible.
//!
//!
//! # Examples
//!
//! Here's how to create a basic document containing a title page, a table of
//! contents, and two sections.
//!
//! ```rust
//! use latex::document::{DocumentClass, Element, Document};
//! use latex::paragraph::Paragraph;
//! use latex::section::Section;
//! use latex::Renderable;
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
//! section_1.push("lorem ipsum...");
//! doc.push(section_1);
//!
//! let mut section_2 = Section::new("Section 2");
//! section_2.push("lorem ipsum...");
//! doc.push(section_2);
//!
//! let mut rendered = Vec::new();
//! doc.render(&mut rendered)?;
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```

#![feature(box_syntax)]

#[macro_use]
extern crate error_chain;

pub mod paragraph;
pub mod document;
pub mod section;

pub use errors::*;
pub use document::Document;
pub use paragraph::Paragraph;
pub use section::Section;

use std::io::Write;

mod errors {
    error_chain!{
        foreign_links{
            Io(::std::io::Error);
            Fmt(::std::fmt::Error);
            UtfError(::std::string::FromUtf8Error);
        }
    }
}

/// A generic trait for rendering AST nodes to some `Writer`.
pub trait Renderable {
    fn render<W>(&self, writer: &mut W) -> Result<()> where W: Write;
}
