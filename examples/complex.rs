extern crate latex;

use latex::{Document, DocumentClass, print, Element, Section, Align, Equation, List, ListKind};

// const COMPLEX: &'static str = include_str!("complex.tex");

fn create_document() -> Document {
    let mut doc = Document::new(DocumentClass::Article);

    // Set the document's metadata

    doc.preamble
        .title("Hello World")
        .author("Michael-F-Bryan")
        .use_package("amsmath")
        .use_package("parskip");

    doc.push(Element::TitlePage)
        .push(Element::ClearPage)
        .push(Element::TableOfContents)
        .push(Element::ClearPage)
        .push(first_section());

    doc
}

/// The "Introduction" section. It starts off with an intro paragraph, some
/// equations, then lists a bunch of objectives.
fn first_section() -> Section {
    let mut section_1 = Section::new("Introduction");
    section_1.push("This is an example paragraph.");

    let mut equations = Align::new();
    equations
        .push("y &= mx + c")
        .push(Equation::with_label("quadratic", "y &= a x^2 + bx + c"));

    section_1
        .push("Please refer to the equations below:")
        .push(equations);

    let mut objectives = List::new(ListKind::Enumerate);
    objectives
        .push(r"Demonstrate how to use the \textit{latex} library.")
        .push("Create a reasonably complex document")
        .push("???")
        .push("PROFIT!");

    section_1.push("Here are our objectives:").push(objectives);

    section_1
}


pub fn main() {
    let doc = create_document();
    let rendered = print(&doc).unwrap();
    // assert_eq!(rendered, COMPLEX);

    println!("{}", rendered);
}
