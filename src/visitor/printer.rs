use std::io::Write;

use document::{Document, Preamble, Element};
use paragraph::{Paragraph, ParagraphElement};
use super::Visitor;
use super::super::Renderable;
use errors::*;


pub fn print(doc: &Document) -> Result<String> {
    let mut buffer = Vec::new();
    {
        let mut printer = Printer::new(&mut buffer);
        printer.visit_document(doc)?;
    }

    let rendered = String::from_utf8(buffer)?;
    Ok(rendered)
}

pub struct Printer<W> {
    writer: W,
}

impl<W> Printer<W>
    where W: Write
{
    pub fn new(writer: W) -> Printer<W> {
        Printer { writer: writer }
    }
}

impl<W> Visitor for Printer<W>
    where W: Write
{
    fn visit_paragraph_element(&mut self, element: &ParagraphElement) -> Result<()> {
        match *element {
            ParagraphElement::Plain(ref s) => write!(self.writer, "{}", s)?,
            ParagraphElement::InlineCode(ref s) => write!(self.writer, "${}$", s)?,
            ParagraphElement::Bold(ref e) => {
                write!(self.writer, r"\textbf{{")?;
                self.visit_paragraph_element(e)?;
                write!(self.writer, "}}")?;
            }
            ParagraphElement::Italic(ref e) => {
                write!(self.writer, r"\textit{{")?;
                self.visit_paragraph_element(e)?;
                write!(self.writer, "}}")?;
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use self::ParagraphElement::*;

    #[test]
    fn create_simple_paragraph() {
        let should_be = "Hello World";
        let mut buffer = Vec::new();

        let mut para = Paragraph::new();
        para.push_text("Hello World");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_paragraph(&para).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn paragraph_with_bold_text() {
        let should_be = r"Hello \textbf{World}";
        let mut buffer = Vec::new();

        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Bold(Box::new(Plain("World".to_string()))));

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_paragraph(&para).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn paragraph_with_italic_text() {
        let should_be = r"Hello \textit{World}";
        let mut buffer = Vec::new();

        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Italic(Box::new(Plain("World".to_string()))));

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_paragraph(&para).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn inline_code() {
        let should_be = r"Hello $\lambda$ World!";
        let mut buffer = Vec::new();


        let mut para = Paragraph::new();
        para.push_text("Hello ")
            .push(InlineCode(r"\lambda".to_string()))
            .push_text(" World!");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_paragraph(&para).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }
}
