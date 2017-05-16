use paragraph::Paragraph;

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
