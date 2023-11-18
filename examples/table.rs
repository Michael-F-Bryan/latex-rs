extern crate latex;

use latex::{print, Document, DocumentClass, Element, Section, Table, TableColumnSettings, ColumnAlignment};

fn create_document() -> Document {
    let mut doc = Document::new(DocumentClass::Article);

    doc.preamble.use_package("array");  // <-- column settings
    
    doc.preamble.title("My Table");
    doc.preamble.author("Noah Nachtigall");

    doc.push(Element::TitlePage)
        .push(Element::ClearPage)
        .push(Element::TableOfContents)
        .push(Element::ClearPage);

    let mut section_1 = Section::new("Section 1");

    let mut table = Table::new();

    // let default_column_settings = TableColumnSettings {
    //     alignment: ColumnAlignment::Center,
    //     ..Default::default()
    // };

    // table.set_default_column_settings(default_column_settings);

    let raw_column_settings = r"|>{\centering\arraybackslash}p{1cm}|>{\centering\arraybackslash}p{1cm}|>{\centering\arraybackslash}p{1cm}|";

    table.column_settings = raw_column_settings.into();

    table.push_row(["as", "b"]);
    table.push_row([1, 1, 3]);

    section_1.push(Element::Table(table));

    doc.push(section_1);

    doc
}

fn main() {
    let doc = create_document();
    let rendered = print(&doc).unwrap();
    println!("{}", rendered);
}
