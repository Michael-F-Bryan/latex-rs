extern crate latex;

use latex::{print, Document, DocumentClass, Element, Section};

fn create_document() -> Document {
    let mut doc = Document::new(DocumentClass::Article);
    doc.preamble.title("Template document");
    doc.preamble.author("Henrik");

    doc.push(Element::TitlePage).push(Element::ClearPage);
    doc
}

fn create_part_document() -> Document {
    let mut doc = Document::new(DocumentClass::Part);

    let mut section = Section::new("Section 1");
    section.push("Some text which gets included into the main document.");
    doc.push(section);
    doc
}

fn main() {
    let mut template = create_document();
    let mut template2 = template.clone();

    let part = create_part_document();
    println!("{}", print(&part).unwrap());

    template.push(Element::Input("part.tex".into()));
    println!("{}", print(&template).unwrap());

    template2.push_doc(&part);
    println!("{}", print(&template2).unwrap());
}
