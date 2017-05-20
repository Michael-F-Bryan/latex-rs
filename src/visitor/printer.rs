use std::io::Write;

use document::{Document, Preamble, Element};
use paragraph::{Paragraph, ParagraphElement};
use lists::{Item, List};
use section::Section;
use equations::{Align, Equation};
use super::Visitor;
use errors::*;


/// Print a document to a string.
pub fn print(doc: &Document) -> Result<String> {
    let mut buffer = Vec::new();
    {
        let mut printer = Printer::new(&mut buffer);
        printer.visit_document(doc)?;
    }

    let rendered = String::from_utf8(buffer)?;
    Ok(rendered)
}

/// The type which uses the `Visitor` pattern to visit each node in a document
/// and write its `tex` representation to a `Writer`.
pub struct Printer<W> {
    writer: W,
}

impl<W> Printer<W>
    where W: Write
{
    /// Create a new `Printer` which will write to the provided `Writer`.
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

    fn visit_paragraph(&mut self, para: &Paragraph) -> Result<()> {
        for elem in para.iter() {
            self.visit_paragraph_element(elem)?;
        }
        writeln!(self.writer)?;

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

    fn visit_section(&mut self, section: &Section) -> Result<()> {
        writeln!(self.writer, r"\section{{{}}}", section.name)?;

        if !section.elements.is_empty() {
            // Make sure there's space between the \section{...} and the next line
            writeln!(self.writer)?;
        }

        for element in section.iter() {
            self.visit_element(element)?;
            // LaTeX needs an empty line between paragraphs/elements otherwise
            // it'll automatically concatenate them together
            write!(self.writer, "\n")?;
        }

        Ok(())
    }

    fn visit_equation(&mut self, equation: &Equation) -> Result<()> {
        write!(self.writer, r"{}", equation.text)?;

        if let Some(ref label) = equation.label {
            write!(self.writer, r" \label{{{}}}", label)?;
        }
        if equation.not_numbered {
            write!(self.writer, r" \nonumber")?;
        }

        writeln!(self.writer, r" \\")?;
        Ok(())
    }

    fn visit_align(&mut self, align: &Align) -> Result<()> {
        writeln!(self.writer, r"\begin{{align}}")?;

        for item in align.iter() {
            self.visit_equation(item)?;
        }

        writeln!(self.writer, r"\end{{align}}")?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use {Section, ListKind, Paragraph, DocumentClass, Equation, Align};
    use self::ParagraphElement::*;

    #[test]
    fn create_simple_paragraph() {
        let should_be = "Hello World\n";
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
        let should_be = "Hello \\textbf{World}\n";
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
        let should_be = "Hello \\textit{World}\n";
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
        let should_be = "Hello $\\lambda$ World!\n";
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

    #[test]
    fn render_blank_section() {
        let should_be = "\\section{First Section}\n";
        let mut buffer = Vec::new();

        let section = Section::new("First Section");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_section(&section).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn section_with_paragraphs() {
        let should_be = r#"\section{First Section}

Lorem Ipsum...

Hello World!

"#;
        let mut buffer = Vec::new();

        let mut section = Section::new("First Section");
        section.push("Lorem Ipsum...").push("Hello World!");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_section(&section).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_empty_align() {
        let should_be = "\\begin{align}\n\\end{align}\n";
        let mut buffer = Vec::new();

        let equations = Align::new();

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_align(&equations).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_simple_equation() {
        let should_be = "x &= y + \\sigma \\\\\n";
        let mut buffer = Vec::new();
        let eq = Equation::new(r"x &= y + \sigma");


        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_equation(&eq).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_several_equations() {
        let should_be = r"\begin{align}
E &= m c^2 \label{eq:mass-energy-equivalence} \\
y &= m x + c \\
\end{align}
";
        let mut buffer = Vec::new();

        let mut equations = Align::new();

        equations
            .push(Equation::with_label("eq:mass-energy-equivalence", "E &= m c^2"))
            .push("y &= m x + c");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_align(&equations).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn equation_with_label() {
        let should_be = "E &= m c^2 \\label{eq:mass-energy-equivalence} \\\\\n";
        let mut buffer = Vec::new();

        let mut eq = Equation::new("E &= m c^2");
        eq.label("eq:mass-energy-equivalence");

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_equation(&eq).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn equation_with_no_numbering() {
        let should_be = "E &= m c^2 \\nonumber \\\\\n";
        let mut buffer = Vec::new();

        let mut eq = Equation::new("E &= m c^2");
        eq.not_numbered();

        {
            let mut printer = Printer::new(&mut buffer);
            printer.visit_equation(&eq).unwrap();
        }

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }
}
