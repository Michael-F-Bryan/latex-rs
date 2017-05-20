use std::slice::Iter;

/// A single paragraph.
///
/// # Examples
///
/// Like most of the other types in this crate, the standard workflow is to
/// create an empty `Paragraph` then incrementally add bits to it using method
/// chaining and the `push()` method.
///
/// ```rust
/// use latex::{Paragraph, ParagraphElement};
///
/// let mut p = Paragraph::new();
/// p.push("Hello ")
///  .push(ParagraphElement::italic("World"))
///  .push("!")
///  .push(" Here is an equation ")
///  .push(ParagraphElement::InlineMath("y = mx + c".to_string()))
///  .push(".");
/// ```
///
/// The above paragraph would get rendered to something like this:
///
/// ```tex
/// Hello \textit{World}! Here is an equation $y = mx + c$.
/// ```
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
    pub fn push<P>(&mut self, elem: P) -> &mut Self
        where P: Into<ParagraphElement>
    {
        self.elements.push(elem.into());
        self
    }

    /// Add some raw text to the paragraph.
    pub fn push_text(&mut self, text: &str) -> &mut Self {
        self.push(ParagraphElement::Plain(text.to_string()))
    }

    /// Iterate over the `ParagraphElement`s in this `Paragraph`.
    pub fn iter(&self) -> Iter<ParagraphElement> {
        self.elements.iter()
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
    InlineMath(String),
}

impl ParagraphElement {
    /// Convenience method for wrapping a `ParagraphElement` in an italics tag.
    pub fn italic<E>(elem: E) -> ParagraphElement
        where E: Into<ParagraphElement>
    {
        ParagraphElement::Italic(Box::new(elem.into()))
    }

    /// Convenience method for wrapping a `ParagraphElement` in a bold tag.
    pub fn bold<E>(elem: E) -> ParagraphElement
        where E: Into<ParagraphElement>
    {
        ParagraphElement::Bold(Box::new(elem.into()))
    }
}

impl<'a> From<&'a str> for ParagraphElement {
    fn from(other: &'a str) -> Self {
        ParagraphElement::Plain(other.to_string())
    }
}
