use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::ops::Deref;

use paragraph::Paragraph;
use section::Section;
use super::Renderable;
use errors::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    pub class: DocumentClass,
    pub preamble: Preamble,
    elements: Vec<Element>,
}

impl Document {
    pub fn new(document_class: DocumentClass) -> Self {
        Document {
            class: document_class,
            ..Default::default()
        }
    }

    pub fn push<E>(&mut self, element: E) -> &mut Self
        where E: Into<Element>
    {
        self.elements.push(element.into());
        self
    }
}

impl Deref for Document {
    type Target = Vec<Element>;

    /// A shortcut to let you iterate over the elements in the `Document`.
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl Renderable for Document {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        writeln!(writer, r"\documentclass{{{}}}", self.class)?;

        self.preamble.render(writer)?;

        writeln!(writer, r"\begin{{document}}")?;

        for element in &self.elements {
            element.render(writer)?;
        }

        writeln!(writer, r"\end{{document}}")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Para(Paragraph),
    Section(Section),
    TableOfContents,
    TitlePage,
    ClearPage,
    UserDefined(String),

    // Add a dummy element so we can expand later on without breaking stuff
    #[doc(hidden)]
    _Other,
}

impl From<Section> for Element {
    fn from(other: Section) -> Self {
        Element::Section(other)
    }
}

impl Renderable for Element {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        match *self {
            Element::Para(ref p) => p.render(writer)?,
            Element::Section(ref s) => s.render(writer)?,
            Element::TableOfContents => writeln!(writer, r"\tableofcontents")?,
            Element::TitlePage => writeln!(writer, r"\maketitle")?,
            Element::ClearPage => writeln!(writer, r"\clearpage")?,
            Element::UserDefined(ref s) => writeln!(writer, "{}", s)?,
            Element::_Other => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DocumentClass {
    Article,
    Book,
    Report,
}

impl Default for DocumentClass {
    fn default() -> Self {
        DocumentClass::Article
    }
}

impl Display for DocumentClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DocumentClass::Article => write!(f, "article"),
            DocumentClass::Book => write!(f, "book"),
            DocumentClass::Report => write!(f, "report"),
        }
    }
}


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Preamble {
    author: Option<String>,
    title: Option<String>,
    uses: Vec<String>,
}

impl Preamble {
    pub fn author(&mut self, name: &str) -> &mut Self {
        self.author = Some(name.to_string());
        self
    }

    pub fn title(&mut self, name: &str) -> &mut Self {
        self.title = Some(name.to_string());
        self
    }

    pub fn use_package(&mut self, name: &str) -> &mut Self {
        self.uses.push(name.to_string());
        self
    }
}

impl Renderable for Preamble {
    fn render<W: Write>(&self, writer: &mut W) -> Result<()> {
        for item in &self.uses {
            writeln!(writer, r"\usepackage{{{}}}", item)?;
        }

        if !self.uses.is_empty() && (self.title.is_some() || self.author.is_some()) {
            writeln!(writer)?;
        }

        if let Some(ref title) = self.title {
            writeln!(writer, r"\title{{{}}}", title)?;
        }
        if let Some(ref author) = self.author {
            writeln!(writer, r"\author{{{}}}", author)?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_empty_document() {
        let should_be = r#"\documentclass{article}
\begin{document}
\end{document}
"#;

        let doc = Document::new(DocumentClass::Article);
        let mut rendered = vec![];
        doc.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }

    #[test]
    fn preamble_with_author_and_title() {
        let should_be = r#"\title{Sample Document}
\author{Michael-F-Bryan}
"#;
        let mut preamble = Preamble::default();
        preamble.title("Sample Document").author("Michael-F-Bryan");

        let mut rendered = vec![];
        preamble.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }

    #[test]
    fn preamble_with_title_and_package_imports() {
        let should_be = r#"\usepackage{amsmath}
\usepackage{graphics}

\title{Sample Document}
"#;
        let mut preamble = Preamble::default();
        preamble
            .title("Sample Document")
            .use_package("amsmath")
            .use_package("graphics");

        let mut rendered = vec![];
        preamble.render(&mut rendered).unwrap();

        assert_eq!(String::from_utf8(rendered).unwrap(), should_be);
    }
}
