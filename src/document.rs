use std::fmt::{self, Display, Formatter};
use std::ops::Deref;
use std::slice::Iter;

use equations::Align;
use lists::List;
use paragraph::Paragraph;
use section::Section;

/// The root Document node.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    /// The document class.
    pub class: DocumentClass,
    /// The `Document`'s preamble.
    pub preamble: Preamble,
    /// The various elements inside this `Document`.
    elements: Vec<Element>,
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
    where
        E: Into<Element>,
    {
        self.elements.push(element.into());
        self
    }

    /// Iterate over the Elements in this document.
    pub fn iter(&self) -> Iter<Element> {
        self.elements.iter()
    }
}

impl Deref for Document {
    type Target = Vec<Element>;

    /// A shortcut to let you iterate over the elements in the `Document`.
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

/// The major elements in a `Document`, representing each type of possible
/// node.
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
        Element::Para(Paragraph::from(other))
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
where
    S: AsRef<str>,
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    /// Converts a tuple of name and a list of lines into an
    /// `Element::Environment`.
    fn from(other: (S, I)) -> Self {
        let (name, lines) = other;
        Element::Environment(
            name.as_ref().to_string(),
            lines.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }
}

/// The kind of Document being generated.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum DocumentClass {
    Article,
    Book,
    Report,
    /// A partial document comes without header and footer.
    /// It is intended to be included (`include{}`) in some other tex file.
    Part,
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
            DocumentClass::Part => write!(f, ""),
        }
    }
}

/// A node representing the document's preamble.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Preamble {
    /// The document's author.
    pub author: Option<String>,
    /// An optional title for the document.
    pub title: Option<String>,
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

    /// Iterate over each package used in the Preamble.
    pub fn iter(&self) -> Iter<String> {
        self.uses.iter()
    }

    /// Are any packages being imported in the Preamble?
    pub fn is_empty(&self) -> bool {
        self.uses.is_empty()
    }
}
