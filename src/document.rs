use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::ops::Deref;

use paragraph::Paragraph;
use section::Section;
use equations::Align;
use errors::*;
use lists::List;
use super::Renderable;

/// The root Document node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    /// The document class.
    pub class: DocumentClass,
    /// The `Document`'s preamble.
    pub preamble: Preamble,
    /// The various elements inside this `Document`.
    pub elements: Vec<Element>,
}

impl Document {
    /// Create a new `Document` with the specified `DocumentClass`.
    pub fn new(document_class: DocumentClass) -> Self {
        Document {
            class: document_class,
            ..Default::default()
        }
    }

    /// Add an element to the `Document`.
    ///
    /// To make this work as seamlessly as possible, it will accept anything
    /// which can be converted into an `Element` using `into()` and supports
    /// the builder pattern with method chaining.
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

/// The major elements in a `Document`.
///
/// For convenience, any variant which wraps a struct will implement `From` for
/// that struct. Meaning you can create an `Element::Para` node just by using
/// `some_paragraph.into()`.
#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    /// A bare paragraph.
    ///
    /// # Note
    ///
    /// You probably don't want to add a paragraph directly to your document,
    /// instead add it to a `Section` so that if you are walking the AST later
    /// on things make sense.
    Para(Paragraph),
    /// A section.
    Section(Section),
    /// The table of contents.
    TableOfContents,
    /// The title page.
    TitlePage,
    /// Clear the page.
    ClearPage,
    /// An `align` environment for containing a bunch of equations.
    Align(Align),

    /// A generic environment and its lines.
    Environment(String, Vec<String>),

    /// Any other element.
    ///
    /// This can be used as an escape hatch if the particular element you want
    /// isn't directly supported or if you need to do something which isn't
    /// easily expressed any other way. You simply provide the raw string you
    /// want and it will be rendered unchanged in the final document.
    UserDefined(String),
    /// A list.
    List(List),

    // Add a dummy element so we can expand later on without breaking stuff
    #[doc(hidden)]
    _Other,
}

impl From<Paragraph> for Element {
    fn from(other: Paragraph) -> Self {
        Element::Para(other)
    }
}

impl<'a> From<&'a str> for Element {
    /// Create an arbitrary unescaped element from a string.
    fn from(other: &'a str) -> Self {
        Element::UserDefined(other.to_string())
    }
}

impl From<List> for Element {
    fn from(other: List) -> Self {
        Element::List(other)
    }
}

impl From<Align> for Element {
    fn from(other: Align) -> Self {
        Element::Align(other)
    }
}

impl From<Section> for Element {
    fn from(other: Section) -> Self {
        Element::Section(other)
    }
}

impl<S, I> From<(S, I)> for Element
    where S: AsRef<str>,
          I: IntoIterator,
          I::Item: AsRef<str>
{
    /// Converts a tuple of name and a list of lines into an
    /// `Element::Environment`.
    fn from(other: (S, I)) -> Self {
        let (name, lines) = other;
        Element::Environment(name.as_ref().to_string(),
                             lines.into_iter().map(|s| s.as_ref().to_string()).collect())
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
            Element::Align(ref equations) => equations.render(writer)?,

            Element::Environment(ref name, ref lines) => {
                writeln!(writer, r"\begin{{{}}}", name)?;
                for line in lines {
                    writeln!(writer, "{}", line)?;
                }
                writeln!(writer, r"\end{{{}}}", name)?;
            }
            Element::List(ref list) => list.render(writer)?,

            Element::_Other => unreachable!(),
        }

        Ok(())
    }
}

/// The kind of Document being generated.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
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


/// A node representing the document's preamble.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Preamble {
    author: Option<String>,
    title: Option<String>,
    uses: Vec<String>,
}

impl Preamble {
    /// Set the document's author.
    pub fn author(&mut self, name: &str) -> &mut Self {
        self.author = Some(name.to_string());
        self
    }

    /// Set the document title.
    pub fn title(&mut self, name: &str) -> &mut Self {
        self.title = Some(name.to_string());
        self
    }

    /// Add a package import to the preamble.
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
