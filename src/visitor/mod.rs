//! A trait which lets you walk your document's AST.

#![allow(missing_docs)]

use std::ops::Deref;

use document::{Document, Preamble, Element};
use paragraph::{Paragraph, ParagraphElement};
use section::Section;
use equations::{Align, Equation};
use lists::{List, Item};
use errors::*;

mod printer;

pub use self::printer::{print, Printer};

#[allow(unused_variables)]
pub trait Visitor {
    fn visit_document(&mut self, doc: &Document) -> Result<()> {
        self.visit_preamble(&doc.preamble)?;

        for element in doc.iter() {
            self.visit_element(element)?;
        }

        Ok(())
    }

    fn visit_element(&mut self, elem: &Element) -> Result<()> {
        match *elem {
            Element::Para(ref p) => self.visit_paragraph(p)?,
            Element::Section(ref s) => self.visit_section(s)?,
            Element::UserDefined(ref s) => self.visit_user_defined_line(s)?,
            Element::Align(ref equations) => self.visit_align(equations)?,

            Element::Environment(ref name, ref lines) => {
                self.visit_custom_environment(name, lines.iter().map(Deref::deref))?
            }
            Element::List(ref list) => self.visit_list(list)?,

            _ => {}
        }

        Ok(())
    }

    fn visit_preamble(&mut self, preamble: &Preamble) -> Result<()> {
        Ok(())
    }

    fn visit_paragraph_element(&mut self, element: &ParagraphElement) -> Result<()> {
        Ok(())
    }

    fn visit_user_defined_line(&mut self, line: &str) -> Result<()> {
        Ok(())
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<()> {
        for elem in &paragraph.elements {
            self.visit_paragraph_element(elem)?;
        }

        Ok(())
    }

    fn visit_section(&mut self, section: &Section) -> Result<()> {
        for elem in section.iter() {
            self.visit_element(elem)?;
        }

        Ok(())
    }

    fn visit_align(&mut self, align: &Align) -> Result<()> {
        for equation in align.iter() {
            self.visit_equation(equation)?;
        }

        Ok(())
    }

    fn visit_equation(&mut self, equation: &Equation) -> Result<()> {
        Ok(())
    }

    fn visit_list(&mut self, list: &List) -> Result<()> {
        for item in list.iter() {
            self.visit_list_item(item)?;
        }

        Ok(())
    }

    fn visit_list_item(&self, item: &Item) -> Result<()> {
        Ok(())
    }

    fn visit_custom_environment<'a, I>(&self, name: &str, lines: I) -> Result<()>
        where I: Iterator<Item = &'a str>
    {
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
