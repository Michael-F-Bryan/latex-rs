use std::{default, fmt::{Display, format}, ops::Deref, slice::Iter};

use document::Element;

/// Column alignment.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ColumnAlignment {
    #[default]
    /// Align the equations to the left.
    Left,
    /// Align the equations to the right.
    Right,
    /// Align the equations to the center.
    Center,
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

/// A Table.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    /// The content of the table.
    pub content: Vec<TableRow>,
    /// The colum settings of the table.
    pub column_settings: TableColumnSettingsWrapper,
    custom_default_column_settings: Option<TableColumnSettings>,
}

/// A Table Row.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableRow {
    /// The content of the row.
    pub content: Vec<String>,
    pub(crate) columns: usize,
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

/// The Table Colum Settings
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableColumnSettings {
    /// The alignment of the colum.
    pub alignment: ColumnAlignment,
}

impl Into<TableColumnSettingsWrapper> for Vec<TableColumnSettings> {
    fn into(self) -> TableColumnSettingsWrapper {
        TableColumnSettingsWrapper::Typed(self)
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

impl Table {
    /// Create a new table.
    pub fn new() -> Table {
        Table {
            ..Default::default()
        }
    }

    /// Add a row to the table.
    pub fn push_row<I, T>(&mut self, row_iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Display + Clone,

    {
        let mut row = TableRow::default();

        for e in row_iter {
            row.push_item(format!("{}", e));
            row.columns += 1;
        }

        self.content.push(row);
        self
    }

    /// Iterate over the rows in this table.
    pub fn iter_row(&self) -> Iter<TableRow> {
        self.content.iter()
    }

    /// Is this table empty?
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get the maximum number of colums of all rows.
    pub fn number_columns(&self) -> usize {
        self.iter_row().fold(0, |acc, row| {
            if row.columns > acc {
                row.columns
            } else {
                acc
            }
        })
    }

    /// Set the colum settings for all columns.
    pub fn set_default_column_settings(&mut self, column_settings: TableColumnSettings) -> &mut Self {
        self.custom_default_column_settings = Some(column_settings.into());
        self
    }

    /// Get the colum settings for all columns.
    pub fn default_column_settings(&self) -> Option<&TableColumnSettings> {
        self.custom_default_column_settings.as_ref()
    }

    /// Set colum settings for a specific column.
    /// # Panics
    /// Panics if the column does not exist.
    pub fn set_column_settings(&mut self, column: usize, column_settings: TableColumnSettings) 
    {
        if column >= self.number_columns() {
            // TODO: panic?
            panic!("Column {} does not exist", column);
        }

        let mut current_settings = match &self.column_settings {
            TableColumnSettingsWrapper::Typed(settings) => settings.clone(),
            TableColumnSettingsWrapper::Raw(_) => Vec::new(),
        };

        current_settings[column] = column_settings;

        self.column_settings = current_settings.into();
    }
}
