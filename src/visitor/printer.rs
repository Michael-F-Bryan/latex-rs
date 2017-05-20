use std::io::Write;

use document::{Document, Preamble, Element};
use paragraph::{Paragraph, ParagraphElement};
use super::Visitor;
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

    fn visit_preamble(&mut self, preamble: &Preamble) -> Result<()> {
        for item in &preamble.uses {
            writeln!(self.writer, r"\usepackage{{{}}}", item)?;
        }

        if !preamble.uses.is_empty() && (preamble.title.is_some() || preamble.author.is_some()) {
            writeln!(self.writer)?;
        }

        if let Some(ref title) = preamble.title {
            writeln!(self.writer, r"\title{{{}}}", title)?;
        }
        if let Some(ref author) = preamble.author {
            writeln!(self.writer, r"\author{{{}}}", author)?;
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

    #[test]
    fn preamble_with_author_and_title() {
        let should_be = r#"\title{Sample Document}
\author{Michael-F-Bryan}
"#;
        let mut buffer = Vec::new();

        let mut preamble = Preamble::default();
        preamble.title("Sample Document").author("Michael-F-Bryan");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_preamble(&preamble).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn preamble_with_title_and_package_imports() {
        let should_be = r#"\usepackage{amsmath}
\usepackage{graphics}

\title{Sample Document}
"#;
        let mut buffer = Vec::new();

        let mut preamble = Preamble::default();
        preamble
            .title("Sample Document")
            .use_package("amsmath")
            .use_package("graphics");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_preamble(&preamble).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

}
