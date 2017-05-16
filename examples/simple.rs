extern crate latex;


use latex::document::{DocumentClass, Element, Document};
use latex::section::Section;
use latex::Renderable;

fn create_document() -> latex::Result<String> {
    let mut doc = Document::new(DocumentClass::Article);

    // Set some metadata for the document
    doc.preamble.title("My Fancy Document");
    doc.preamble.author("Michael-F-Bryan");

    doc.push(Element::TitlePage);
    doc.push(Element::ClearPage);
    doc.push(Element::TableOfContents);
    doc.push(Element::ClearPage);

    let mut section_1 = Section::new("Section 1");
    section_1.push("lorem ipsum...");
    doc.push(section_1);

    let mut section_2 = Section::new("Section 2");
    section_2.push("lorem ipsum...");
    doc.push(section_2);

    let mut rendered = vec![];
    doc.render(&mut rendered)?;

    Ok(String::from_utf8(rendered)?)
}

fn main() {
    let rendered = create_document().unwrap();
    println!("{}", rendered);
}
