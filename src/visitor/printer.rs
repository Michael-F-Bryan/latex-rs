use std::io::Write;

use document::{Document, Preamble, Element, DocumentClass};
use paragraph::{Paragraph, ParagraphElement};
use lists::{Item, ListKind, List};
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
    fn visit_document(&mut self, doc: &Document) -> Result<()> {
        writeln!(self.writer, r"\documentclass{{{}}}", doc.class)?;

        self.visit_preamble(&doc.preamble)?;

        writeln!(self.writer, r"\begin{{document}}")?;

        for element in &doc.elements {
            self.visit_element(element)?;
        }

        writeln!(self.writer, r"\end{{document}}")?;
        Ok(())
    }

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

    fn visit_list(&mut self, list: &List) -> Result<()> {
        let env = list.kind.environment_name();

        writeln!(self.writer, r"\begin{{{}}}", env)?;

        for item in list.iter() {
            self.visit_list_item(item)?;
        }

        writeln!(self.writer, r"\end{{{}}}", env)?;

        Ok(())
    }

    fn visit_list_item(&mut self, item: &Item) -> Result<()> {
        writeln!(self.writer, r"\item {}", item.0)?;
        Ok(())
    }

    fn visit_element(&mut self, element: &Element) -> Result<()> {
        match *element {
            Element::Para(ref p) => self.visit_paragraph(p)?,
            Element::Section(ref s) => self.visit_section(s)?,
            Element::TableOfContents => writeln!(self.writer, r"\tableofcontents")?,
            Element::TitlePage => writeln!(self.writer, r"\maketitle")?,
            Element::ClearPage => writeln!(self.writer, r"\clearpage")?,
            Element::UserDefined(ref s) => writeln!(self.writer, "{}", s)?,
            Element::Align(ref equations) => self.visit_align(equations)?,

            Element::Environment(ref name, ref lines) => {
                writeln!(self.writer, r"\begin{{{}}}", name)?;
                for line in lines {
                    writeln!(self.writer, "{}", line)?;
                }
                writeln!(self.writer, r"\end{{{}}}", name)?;
            }
            Element::List(ref list) => self.visit_list(list)?,

            Element::_Other => unreachable!(),
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


    #[test]
    fn render_empty_document() {
        let should_be = r#"\documentclass{article}
\begin{document}
\end{document}
"#;
        let mut buffer = Vec::new();

        let doc = Document::new(DocumentClass::Article);

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_document(&doc).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }


    #[test]
    fn render_enumerated_list() {
        let should_be = "\\begin{enumerate}\n\\end{enumerate}\n";
        let mut buffer = Vec::new();

        let list = List::new(ListKind::Enumerate);

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_list(&list).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_empty_itemize_list() {
        let should_be = "\\begin{itemize}\n\\end{itemize}\n";
        let mut buffer = Vec::new();

        let list = List::new(ListKind::Itemize);

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_list(&list).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_list_with_items() {
        let should_be = r"\begin{itemize}
\item This
\item is
\item a
\item list!
\end{itemize}
";
        let mut buffer = Vec::new();

        let mut list = List::new(ListKind::Itemize);
        list.push("This").push("is").push("a").push("list!");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_list(&list).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }
}
