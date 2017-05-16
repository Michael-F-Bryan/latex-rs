use std::fmt::{self, Display, Formatter, Write};

use paragraph::Paragraph;
use super::Renderable;
use errors::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    class: DocumentClass,
    preamble: Preamble,
    elements: Vec<Element>,
}

impl Document {
    pub fn new(document_class: DocumentClass) -> Self {
        Document {
            class: document_class,
            ..Default::default()
        }
    }
}

impl Renderable for Document {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        let _ = writeln!(writer, r"\documentclass{{{}}}", self.class);

        let _ = writeln!(writer, r"\begin{{document}}");
        let _ = writeln!(writer, r"\end{{document}}");
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Para(Paragraph),
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

impl Renderable for Preamble {
    fn render<W: Write>(&self, writer: &mut W) -> Result<()> {
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
        let mut rendered = String::new();
        doc.render(&mut rendered).unwrap();

        assert_eq!(rendered, should_be);
    }
}
