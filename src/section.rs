use std::fmt::Write;

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

        for element in &self.elements {
            element.render(writer)?;
            writeln!(writer)?;
        }

        Ok(())
    }
}
