//! Basic paragraph manipulation.
use std::fmt::Write;

use errors::*;
use super::Renderable;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Paragraph {
    elements: Vec<ParagraphElement>,
}

impl Paragraph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, elem: ParagraphElement) -> &mut Self {
        self.elements.push(elem);
        self
    }

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


#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphElement {
    Plain(String),
    Bold(Box<ParagraphElement>),
    Italic(Box<ParagraphElement>),
    InlineCode(String),
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

        let mut rendered = String::new();
        para.render(&mut rendered).unwrap();

        assert_eq!(rendered, should_be);
    }

    #[test]
    fn paragraph_with_bold_text() {
        let should_be = r"Hello \textbf{World}";
        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Bold(box Plain("World".to_string())));

        let mut rendered = String::new();
        para.render(&mut rendered).unwrap();

        assert_eq!(rendered, should_be);
    }

    #[test]
    fn paragraph_with_italic_text() {
        let should_be = r"Hello \textit{World}";
        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Italic(box Plain("World".to_string())));

        let mut rendered = String::new();
        para.render(&mut rendered).unwrap();

        assert_eq!(rendered, should_be);
    }

    #[test]
    fn inline_code() {
        let should_be = r"Hello $\lambda$ World!";

        let mut para = Paragraph::new();
        para.push_text("Hello ")
            .push(InlineCode(r"\lambda".to_string()))
            .push_text(" World!");

        let mut rendered = String::new();
        para.render(&mut rendered).unwrap();

        assert_eq!(rendered, should_be);
    }
}
