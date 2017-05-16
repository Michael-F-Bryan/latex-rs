//! Module for working with `Sections`

use std::io::Write;

use paragraph::Paragraph;
use super::Renderable;
use errors::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    name: String,
    elements: Vec<Paragraph>,
}

impl Section {
    pub fn new(name: &str) -> Section {
        Section {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn push<I>(&mut self, element: I) -> &mut Self
        where I: Into<Paragraph>
    {
        self.elements.push(element.into());
        self
    }
}

impl Renderable for Section {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        writeln!(writer, r"\section{{{}}}", self.name)?;

        if !self.elements.is_empty() {
            // Make sure there's space between the \section{...} and the next line
            writeln!(writer)?;
        }

        for element in &self.elements {
            element.render(writer)?;
            // LaTeX needs an empty line between paragraphs/elements otherwise
            // it'll automatically concatenate them together
            write!(writer, "\n\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_blank_section() {
        let should_be = "\\section{First Section}\n";
        let section = Section::new("First Section");

        let mut rendered = vec![];
        section.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }

    #[test]
    fn section_with_paragraphs() {
        let should_be = r#"\section{First Section}

Lorem Ipsum...

Hello World!

"#;
        let mut section = Section::new("First Section");
        section.push("Lorem Ipsum...").push("Hello World!");

        let mut rendered = vec![];
        section.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }
}
