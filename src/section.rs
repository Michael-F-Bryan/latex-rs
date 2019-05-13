use std::slice::Iter;

use document::Element;

/// A document Section.
///
/// Like the `Document` type, a `Section` is more or less just a collection of
/// `Element`s. When rendered it will start with `\section{Section Name}` and
/// then each element will be rendered in turn.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    /// The name of the section.
    pub name: String,
    elements: Vec<Element>,
}

impl Section {
    /// Create a new section with the specified name.
    pub fn new(name: &str) -> Section {
        Section {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Add an element to the Section.
    pub fn push<I>(&mut self, element: I) -> &mut Self
    where
        I: Into<Element>,
    {
        self.elements.push(element.into());
        self
    }

    /// Iterate over the elements in this list.
    pub fn iter(&self) -> Iter<Element> {
        self.elements.iter()
    }

    /// Is this section empty?
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
