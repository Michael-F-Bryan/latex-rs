//! A trait which lets you walk your document's AST.

mod printer;

pub use self::printer::{print, Printer};

use document::{Document, DocumentClass, Element, Preamble};
use equations::{Align, Equation};
use failure::Error;
use lists::{Item, List};
use paragraph::{Paragraph, ParagraphElement};
use section::Section;
use std::ops::Deref;

/// A trait which uses the [Visitor Pattern] to recursively visit each node in
/// a `Document`.
///
/// [Visitor Pattern]: https://en.wikipedia.org/wiki/Visitor_pattern
#[allow(unused_variables)]
pub trait Visitor {
    /// Visit the root `Document` node, then recursively visit the preamble and
    /// each element in the `Document`.
    fn visit_document(&mut self, doc: &Document) -> Result<(), Error> {
        if doc.class != DocumentClass::Part {
            self.visit_preamble(&doc.preamble)?;
        }

        for element in doc.iter() {
            self.visit_element(element)?;
        }

        Ok(())
    }

    /// Visit a single `Element` node, dispatching to the more specific
    /// `visit_*()` methods.
    ///
    /// > **Note:** You probably don't want to implement this one yourself. If
    /// you forget to recursively visit each and every variant of `Element`
    /// you may end up accidentally ignoring half your document!
    fn visit_element(&mut self, elem: &Element) -> Result<(), Error> {
        match *elem {
            Element::Para(ref p) => self.visit_paragraph(p)?,
            Element::Section(ref s) => self.visit_section(s)?,
            Element::UserDefined(ref s) => self.visit_user_defined_line(s)?,
            Element::Align(ref equations) => self.visit_align(equations)?,

            Element::Environment(ref name, ref lines) => {
                self.visit_custom_environment(name, lines.iter().map(Deref::deref))?
            }
            Element::List(ref list) => self.visit_list(list)?,
            Element::Input(ref s) => self.visit_input(s)?,

            _ => {}
        }

        Ok(())
    }

    /// Visit a document's `Preamble`.
    fn visit_preamble(&mut self, preamble: &Preamble) -> Result<(), Error> {
        Ok(())
    }

    /// Visit an element in a `Paragraph` (e.g. `Italic`, `InlineCode`).
    fn visit_paragraph_element(&mut self, element: &ParagraphElement) -> Result<(), Error> {
        Ok(())
    }

    /// Visit a user defined line.
    fn visit_user_defined_line(&mut self, line: &str) -> Result<(), Error> {
        Ok(())
    }

    /// Visit a input element.
    fn visit_input(&mut self, input: &str) -> Result<(), Error> {
        Ok(())
    }

    /// Visit a paragraph, and every `ParagraphElement` in it.
    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<(), Error> {
        for elem in &paragraph.elements {
            self.visit_paragraph_element(elem)?;
        }

        Ok(())
    }

    /// Visit a `Section` and then recursively visit each of its `Element`s.
    fn visit_section(&mut self, section: &Section) -> Result<(), Error> {
        for elem in section.iter() {
            self.visit_element(elem)?;
        }

        Ok(())
    }

    /// Visit an `Align` block and then recursively visit each equation in the
    /// block.
    fn visit_align(&mut self, align: &Align) -> Result<(), Error> {
        for equation in align.iter() {
            self.visit_equation(equation)?;
        }

        Ok(())
    }

    /// Visit a single `Equation`.
    fn visit_equation(&mut self, equation: &Equation) -> Result<(), Error> {
        Ok(())
    }

    /// Visit a `List` and all of its items.
    fn visit_list(&mut self, list: &List) -> Result<(), Error> {
        for item in list.iter() {
            self.visit_list_item(item)?;
        }

        Ok(())
    }

    /// Visit a single list item.
    fn visit_list_item(&mut self, item: &Item) -> Result<(), Error> {
        Ok(())
    }

    /// Visit an arbitrary environment and receive an iterator over its lines.
    fn visit_custom_environment<'a, I>(&mut self, name: &str, lines: I) -> Result<(), Error>
    where
        I: Iterator<Item = &'a str>,
    {
        Ok(())
    }
}
