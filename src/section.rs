use std::slice::Iter;

use document::Element;

/// A document Section.
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
        where I: Into<Element>
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
