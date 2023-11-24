//! https://en.wikibooks.org/wiki/LaTeX/Tables
//! https://en.wikibooks.org/wiki/LaTeX/Tables#The_tabular_environment
//!
//! The `table` module provides a way to create latex tables. `Table` is an interface to the LaTeX `tabular` environment.

use std::{
    default,
    fmt::{format, Display},
    ops::Deref,
    slice::Iter,
};

use document::Element;

/// Column alignment. Part of the "table spec" argument.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColumnAlignment {
    /// left-justified column
    #[default]
    Left,
    /// right-justified column
    Right,
    /// centered column
    Center,
    // TODO: implement
    // paragraph column with text vertically aligned at the top
    // ParagraphTop
    // paragraph column with text vertically aligned in the middle (requires array package)
    // ParagraphMiddle
    // paragraph column with text vertically aligned at the bottom (requires array package)
    // ParagraphBottom
}

impl Display for ColumnAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnAlignment::Left => write!(f, "l"),
            ColumnAlignment::Right => write!(f, "r"),
            ColumnAlignment::Center => write!(f, "c"),
        }
    }
}

/// A Table that can be added to a Document and rendered as a tabular environment.
///
/// # Example
/// ```rust
/// use latex::{Table};
///
/// let mut table = Table::new();
///
/// table.push_row(["a", "b"]);
///
/// table.push_row([1, 1]);
///
/// let mut doc = latex::Document::new(latex::DocumentClass::Article);
///
/// doc.push(latex::Element::Table(table));
///
/// let rendered = latex::print(&doc).unwrap();
///
/// let expected =
/// r#"\documentclass{article}
/// \begin{document}
/// \begin{tabular}{ll}
/// a & b \\
/// 1 & 1 \\
/// \end{tabular}
/// \end{document}
/// "#;
///
/// assert_eq!(rendered, expected)
///
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    /// The content of the table as a vector of `TableRow`.
    pub content: Vec<TableRow>,
    /// The colum settings of the table as `TableColumnSettingsWrapper` which can be either a typed struct or raw LaTeX.
    pub column_settings: TableColumnSettingsWrapper,
    custom_default_column_settings: Option<TableColumnSettings>,
}

/// A Table Row.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableRow {
    /// The content of the row.
    pub content: Vec<String>,
    pub(crate) columns: Option<usize>,
    pub(crate) skip_explicit_new_row: bool,
}

impl TableRow {
    /// Pushes an item to the row.
    pub fn push_item<I>(&mut self, item: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.content.push(item.into());
        self
    }
}

/// The Table Column Settings Wrapper representing either a typed or raw `table spec` argument of the `tabular` environment.
#[derive(Clone, Debug, PartialEq)]
pub enum TableColumnSettingsWrapper {
    /// Column settings as typed struct.
    Typed(Vec<TableColumnSettings>),
    /// Column settings as raw LaTeX.
    Raw(String),
}

impl Default for TableColumnSettingsWrapper {
    fn default() -> Self {
        TableColumnSettingsWrapper::Typed(Vec::new())
    }
}

/// Checks if the `TableColumnSettingsWrapper` is empty.
/// Either `Vec` or `String` is empty.
impl TableColumnSettingsWrapper {
    pub fn is_empty(&self) -> bool {
        match self {
            TableColumnSettingsWrapper::Typed(settings) => settings.is_empty(),
            TableColumnSettingsWrapper::Raw(settings) => settings.is_empty(),
        }
    }
}

/// The struct representing a typed `table spec` argument of the `tabular` environment.
///
/// ```rust
/// use latex::{Table, TableColumnSettings};
///
/// let mut table = Table::new();
///
/// table.push_row(["a", "b"]);
/// table.push_row([1, 1]);
///
/// let column_settings = TableColumnSettings::default().alignment(latex::ColumnAlignment::Center);
/// table.column_settings = vec![column_settings; 2].into();
///
/// let mut doc = latex::Document::new(latex::DocumentClass::Article);
/// doc.push(latex::Element::Table(table));
///
/// let rendered = latex::print(&doc).unwrap();
///
/// let expected =
/// r#"\documentclass{article}
/// \begin{document}
/// \begin{tabular}{cc}
/// a & b \\
/// 1 & 1 \\
/// \end{tabular}
/// \end{document}
/// "#;
///
/// assert_eq!(rendered, expected)
///
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TableColumnSettings {
    /// The alignment of the colum.
    pub alignment: ColumnAlignment,
}

impl TableColumnSettings {
    /// Change the alignment of the column.
    pub fn alignment(&mut self, column_alignment: ColumnAlignment) -> Self {
        self.alignment = column_alignment;
        *self
    }
}

impl Into<TableColumnSettingsWrapper> for Vec<TableColumnSettings> {
    fn into(self) -> TableColumnSettingsWrapper {
        TableColumnSettingsWrapper::Typed(self)
    }
}

impl Into<TableColumnSettingsWrapper> for TableColumnSettings {
    fn into(self) -> TableColumnSettingsWrapper {
        TableColumnSettingsWrapper::Typed(vec![self])
    }
}

impl Into<TableColumnSettingsWrapper> for String {
    fn into(self) -> TableColumnSettingsWrapper {
        TableColumnSettingsWrapper::Raw(self)
    }
}

impl Into<TableColumnSettingsWrapper> for &str {
    fn into(self) -> TableColumnSettingsWrapper {
        TableColumnSettingsWrapper::Raw(self.to_string())
    }
}

pub trait IntoTableRow {
    fn into_table_row(self) -> TableRow;
}

impl<I, T> IntoTableRow for I
where
    I: IntoIterator<Item = T>,
    T: Display + Clone,
{
    fn into_table_row(self) -> TableRow {
        let mut row = TableRow::default();

        row.columns = Some(0);

        for e in self.into_iter() {
            row.push_item(format!("{}", e));

            let current_columns = match row.columns {
                Some(columns) => columns,
                None => 0,
            };

            row.columns = Some(current_columns + 1);
        }

        row
    }
}

/// A Horizontal Line in a Table.
/// # Example
/// ```
/// use latex::{Table, TableHLine};
///
/// let mut table = Table::new();
///
/// table.push_row(TableHLine::default());
///
/// assert_eq!(&table.content[0].content[0], r"\hline");
/// ```
pub struct TableHLine {}

impl Default for TableHLine {
    fn default() -> Self {
        TableHLine {}
    }
}

impl IntoTableRow for TableHLine {
    fn into_table_row(self) -> TableRow {
        let mut row = TableRow::default();

        row.content = vec![r"\hline".to_string()];
        row.skip_explicit_new_row = true;

        row
    }
}

impl Table {
    /// Creates a new table with default settings.
    /// # Example
    /// ```rust
    /// use latex::{Table};
    /// let mut table = Table::new();
    /// ```
    pub fn new() -> Table {
        Table {
            ..Default::default()
        }
    }

    /// Push a row to the end of the table.
    /// # Example
    /// ```
    /// use latex::{Table};
    /// let mut table = Table::new();
    /// table.push_row(["a", "b"]);
    /// ```
    pub fn push_row<R>(&mut self, row: R) -> &mut Self
    where
        R: IntoTableRow,
    {
        self.content.push(row.into_table_row());
        self
    }

    /// Insert a row at `index`.
    /// # Example
    /// ```
    /// use latex::{Table};
    /// let mut table = Table::new();
    /// table.insert_row(0, ["a", "b"]);
    /// ```
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn insert_row<R>(&mut self, index: usize, row: R) -> &mut Self
    where
        R: IntoTableRow,
    {
        self.content.insert(index, row.into_table_row());
        self
    }

    /// Replace a row at `index`.
    /// # Example
    /// ```
    /// use latex::{Table};
    /// let mut table = Table::new();
    /// table.push_row(["a", "b"]);
    /// table.replace_row(0, ["c", "d"]);
    /// ```
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn replace_row<R>(&mut self, index: usize, row: R) -> &mut Self
    where
        R: IntoTableRow,
    {
        self.content[index] = row.into_table_row();
        self
    }

    /// Iterate over the rows in this table.
    /// # Example
    /// ```
    /// use latex::{Table};
    /// let mut table = Table::new();
    ///
    /// table.push_row(["a", "b"]);
    /// table.push_row(["c", "d"]);
    ///
    /// for row in table.iter_row() {
    ///    println!("{:?}", row);
    /// }
    /// ```
    pub fn iter_row(&self) -> Iter<TableRow> {
        self.content.iter()
    }

    /// Is this table empty?
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get the maximum number of columns of all rows.
    /// # Example
    /// ```
    /// use latex::{Table};
    /// let mut table = Table::new();
    ///
    /// table.push_row(["a", "b"]);
    /// table.push_row(["c", "d", "e"]);
    ///
    /// assert_eq!(table.number_columns(), 3);
    pub fn number_columns(&self) -> usize {
        self.iter_row().fold(0, |acc, row| {
            let columns = row.columns.unwrap_or(0);

            if columns > acc {
                columns
            } else {
                acc
            }
        })
    }

    /// Replace column settings for a specific column. (Completely overrides the a column settings.)
    ///
    /// When used on a table with typed column settings, this method will replace the column settings for the specified column.
    ///
    /// When used on a table with raw column settings, this method will replace the column settings for the specified column and set all other columns to the default typed column settings.
    /// # Panics
    /// Panics if the column does not exist.
    pub fn replace_column_settings(
        &mut self,
        column: usize,
        column_settings: TableColumnSettings,
    ) -> &mut Self {
        if column >= self.number_columns() {
            // TODO: panic?
            panic!("Column {} does not exist", column);
        }

        let mut current_settings = match &self.column_settings {
            TableColumnSettingsWrapper::Typed(settings) => settings.clone(),
            TableColumnSettingsWrapper::Raw(_) => vec![TableColumnSettings::default(); column],
        };

        current_settings[column] = column_settings;

        self.column_settings = current_settings.into();

        self
    }
}
