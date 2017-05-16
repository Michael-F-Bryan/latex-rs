use paragraph::Paragraph;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    preamble: Preamble,
    elements: Vec<Element>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Preamble {
    author: Option<String>,
    title: Option<String>,
    uses: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Text(Paragraph),
}
