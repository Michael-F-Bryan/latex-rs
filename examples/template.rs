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
    let part = create_part_document();
    println!("{}\n", print(&part).unwrap());

    let template = create_document();
    let rendered = print(&template).unwrap();
    println!("{}", rendered);
}
