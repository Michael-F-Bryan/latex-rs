use std::slice::Iter;

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
    InlineCode(String),
}

impl<'a> From<&'a str> for ParagraphElement {
    fn from(other: &'a str) -> Self {
        ParagraphElement::Plain(other.to_string())
    }
}
