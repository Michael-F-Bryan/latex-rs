use std::io::Write;

use errors::*;
use super::Renderable;

/// A single paragraph.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Paragraph {
    /// A list of `ParagraphElements` which make up the paragraph's contents.
    pub elements: Vec<ParagraphElement>,
}

impl Paragraph {
    /// Create a new paragraph.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a `ParagraphElement` to the `Paragraph`.
    pub fn push(&mut self, elem: ParagraphElement) -> &mut Self {
        self.elements.push(elem);
        self
    }

    /// Add some raw text to the paragraph.
    pub fn push_text(&mut self, text: &str) -> &mut Self {
        self.push(ParagraphElement::Plain(text.to_string()))
    }
}

impl Renderable for Paragraph {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        for element in &self.elements {
            element.render(writer)?;
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for Paragraph {
    fn from(other: &'a str) -> Paragraph {
        let mut para = Paragraph::new();
        para.push_text(other);
        para
    }
}


/// The various paragraph elements.
///
/// For convenience, you can convert from a string to a `ParagraphElement`
/// using `into()`.
#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphElement {
    /// A plain string.
    Plain(String),
    /// Bolded text.
    Bold(Box<ParagraphElement>),
    /// Italicized text.
    Italic(Box<ParagraphElement>),
    /// An inline mathematical expression.
    InlineCode(String),
}

impl<'a> From<&'a str> for ParagraphElement {
    fn from(other: &'a str) -> Self {
        ParagraphElement::Plain(other.to_string())
    }
}

impl Renderable for ParagraphElement {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        match *self {
            ParagraphElement::Plain(ref s) => write!(writer, "{}", s)?,
            ParagraphElement::InlineCode(ref s) => write!(writer, "${}$", s)?,
            ParagraphElement::Bold(ref e) => {
                write!(writer, r"\textbf{{")?;
                e.render(writer)?;
                write!(writer, "}}")?;
            }
            ParagraphElement::Italic(ref e) => {
                write!(writer, r"\textit{{")?;
                e.render(writer)?;
                write!(writer, "}}")?;
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
        let mut para = Paragraph::new();
        para.push_text("Hello World");

        let mut rendered = vec![];
        para.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }

    #[test]
    fn paragraph_with_bold_text() {
        let should_be = r"Hello \textbf{World}";
        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Bold(box Plain("World".to_string())));

        let mut rendered = vec![];
        para.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }

    #[test]
    fn paragraph_with_italic_text() {
        let should_be = r"Hello \textit{World}";
        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Italic(box Plain("World".to_string())));

        let mut rendered = vec![];
        para.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }

    #[test]
    fn inline_code() {
        let should_be = r"Hello $\lambda$ World!";

        let mut para = Paragraph::new();
        para.push_text("Hello ")
            .push(InlineCode(r"\lambda".to_string()))
            .push_text(" World!");

        let mut rendered = vec![];
        para.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }
}
