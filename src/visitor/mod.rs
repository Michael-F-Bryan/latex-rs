//! A trait which lets you walk your document's AST.

#![allow(missing_docs)]

use document::{Document, Preamble, Element};
use paragraph::{Paragraph, ParagraphElement};
use errors::*;

mod printer;

pub use self::printer::{print, Printer};

#[allow(unused_variables)]
pub trait Visitor {
    fn visit_document(&mut self, doc: &Document) -> Result<()> {
        self.visit_preamble(&doc.preamble)?;

        for element in &doc.elements {
            self.visit_element(element)?;
        }

        Ok(())
    }

    fn visit_element(&mut self, elem: &Element) -> Result<()> {
        match *elem {
            Element::Para(ref par) => self.visit_paragraph(par)?,
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn visit_preamble(&mut self, preamble: &Preamble) -> Result<()> {
        Ok(())
    }
    fn visit_paragraph_element(&mut self, element: &ParagraphElement) -> Result<()> {
        Ok(())
    }
    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<()> {
        for elem in &paragraph.elements {
            self.visit_paragraph_element(elem)?;
        }
        Ok(())
    }
}

// Para(Paragraph),
// Section(Section),
// TableOfContents,
// TitlePage,
// ClearPage,
// Align(Align),
// Environment(String, Vec<String>),
// UserDefined(String),
// List(List),
