
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Paragraph {
    elements: Vec<ParagraphElement>,
}

impl Paragraph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn render(&self) -> String {
        self.elements
            .iter()
            .map(|e| e.render())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn push(&mut self, elem: ParagraphElement) -> &mut Self {
        self.elements.push(elem);
        self
    }

    pub fn push_text(&mut self, text: &str) -> &mut Self {
        self.push(ParagraphElement::Plain(text.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParagraphElement {
    Plain(String),
    Bold(Box<ParagraphElement>),
    Italic(Box<ParagraphElement>),
}

impl ParagraphElement {
    fn render(&self) -> String {
        match *self {
            ParagraphElement::Plain(ref s) => s.clone(),
            ParagraphElement::Bold(ref e) => format!(r"\textbf{{{}}}", e.render()),
            ParagraphElement::Italic(ref e) => format!(r"\textit{{{}}}", e.render()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use self::ParagraphElement::*;

    #[test]
    fn create_simple_paragraph() {
        let should_be = "Hello World";
        let mut para = Paragraph::new();
        para.push_text("Hello World");

        let rendered = para.render();

        assert_eq!(rendered, should_be);
    }

    #[test]
    fn paragraph_with_bold_text() {
        let should_be = r"Hello \textbf{World}";
        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Bold(box Plain("World".to_string())));

        let rendered = para.render();

        assert_eq!(rendered, should_be);
    }

    #[test]
    fn paragraph_with_italic_text() {
        let should_be = r"Hello \textit{World}";
        let mut para = Paragraph::new();
        para.push_text("Hello ");
        para.push(Italic(box Plain("World".to_string())));

        let rendered = para.render();

        assert_eq!(rendered, should_be);
    }
}
