extern crate latex;

use latex::{DocumentClass, Element, Document, Section, Renderable};


fn create_document() -> latex::Result<String> {
    let mut doc = Document::new(DocumentClass::Article);

    // Set some metadata for the document
    doc.preamble.title("My Fancy Document");
    doc.preamble.author("Michael-F-Bryan");

    doc.push(Element::TitlePage)
        .push(Element::ClearPage)
        .push(Element::TableOfContents)
        .push(Element::ClearPage);

    let mut section_1 = Section::new("Section 1");
    section_1
        .push("Here is some text which will be put in paragraph 1.")
        .push("And here is some more text for paragraph 2.");
    doc.push(section_1);

    let mut section_2 = Section::new("Section 2");
    section_2.push("More text...");
    doc.push(section_2);

    let mut buffer = vec![];
    doc.render(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
}

fn main() {
    let rendered = create_document().unwrap();
    println!("{}", rendered);
}
