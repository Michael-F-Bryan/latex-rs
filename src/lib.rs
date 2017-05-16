//! A library for generating LaTeX documents programatically.

#![feature(box_syntax)]

#[macro_use]
extern crate error_chain;

pub mod paragraph;
pub mod document;

pub use errors::*;
use std::fmt::Write;

mod errors {
    error_chain!{
        foreign_links{
            Io(::std::io::Error);
            Fmt(::std::fmt::Error);
        }
    }
}

trait Renderable {
    fn render<W>(&self, writer: &mut W) -> Result<()> where W: Write;
}
