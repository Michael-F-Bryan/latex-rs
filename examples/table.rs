extern crate latex;

use latex::{
    print, ColumnAlignment, Document, DocumentClass, Element, Section, Table, TableColumnSettings,
    TableHLine,
};

fn create_document() -> Document {
    let mut doc = Document::new(DocumentClass::Article);

    doc.preamble.use_package("array"); // <-- column settings

    doc.preamble.title("Table Example");
    doc.preamble.author("Noah Nachtigall");

    let mut section_1 = Section::new("Section 1");

    let mut table = Table::new();

    let raw_column_settings = r"|>{\centering\arraybackslash}p{2cm}|>{\centering\arraybackslash}p{2cm}|>{\centering\arraybackslash}p{2cm}|"; // any valid raw latex code should be valid here

    // Table Column Settings with a raw latex string
    table.column_settings = raw_column_settings.into();

    table
        .push_row(["Example A", "Example B"])
        .push_row(TableHLine::default())
        .push_row([1, 1, 3]); // columns are automatically padded to match the maximum number of columns

    section_1.push(Element::Table(table.clone()));

    // Table Column Settings with a typed TableColumnSettings struct
    let table_column_settings_first = TableColumnSettings {
        alignment: ColumnAlignment::Left,
        ..Default::default()
    };

    let table_column_settings_rest = TableColumnSettings {
        alignment: ColumnAlignment::Right,
        ..Default::default()
    };

    table.column_settings = vec![table_column_settings_first, table_column_settings_rest].into(); // unspecified columns are assumed to contain the same settings as the last specified column (left to right)

    section_1.push(Element::Table(table.clone()));

    table.column_settings = TableColumnSettings {
        alignment: ColumnAlignment::Center,
        ..Default::default()
    }
    .into(); // with one column setting, all columns are assumed to have the same settings

    section_1.push(Element::Table(table.clone()));

    doc.push(section_1);

    doc
}

fn main() {
    let doc = create_document();
    let rendered = print(&doc).unwrap();
    println!("{}", rendered);
}
