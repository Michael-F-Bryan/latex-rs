use std::io::Write;
use std::slice::Iter;
use std::ops::Deref;

use super::Renderable;
use errors::*;


#[derive(Clone, Debug, PartialEq)]
pub struct Item(String);

impl Deref for Item {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Which kind of list should be used?
#[derive(Clone, Debug, PartialEq)]
pub enum ListKind {
    /// A numbered list.
    Enumerate,
    /// An un-numbered list.
    Itemize,
}

impl ListKind {
    fn environment_name(&self) -> &str {
        match *self {
            ListKind::Enumerate => "enumerate",
            ListKind::Itemize => "itemize",
        }
    }
}

/// A list (either dot points or numbered).
///
/// # Examples
///
/// A list can be used like so:
///
/// ```rust
/// use latex::{List, ListKind};
///
/// let mut list = List::new(ListKind::Itemize);
/// list.push("Hello").push("From").push("Some").push("Dot-points");
/// ```
///
/// Calling the `render()` method on the list will then give something like
/// this:
///
/// ```tex
/// \begin{itemize}
/// \item Hello
/// \item From
/// \item Some
/// \item Dot-points
/// \end{itemize}
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct List {
    /// The kind of list this is.
    pub kind: ListKind,
    items: Vec<Item>,
}

impl List {
    /// Create an empty list of the specified type.
    pub fn new(kind: ListKind) -> List {
        List {
            kind: kind,
            items: Vec::new(),
        }
    }

    /// Add an element to the list.
    pub fn push<S: AsRef<str>>(&mut self, item: S) -> &mut Self {
        self.items.push(Item(item.as_ref().to_string()));
        self
    }

    /// Iterate over the items in the list.
    pub fn iter(&self) -> Iter<Item> {
        self.items.iter()
    }
}

impl Renderable for List {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        let env = self.kind.environment_name();

        writeln!(writer, r"\begin{{{}}}", env)?;

        for item in &self.items {
            writeln!(writer, r"\item {}", item.0)?;
        }

        writeln!(writer, r"\end{{{}}}", env)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_enumerated_list() {
        let should_be = "\\begin{enumerate}\n\\end{enumerate}\n";
        let list = List::new(ListKind::Enumerate);

        let mut buffer = Vec::new();
        list.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_empty_itemize_list() {
        let should_be = "\\begin{itemize}\n\\end{itemize}\n";
        let list = List::new(ListKind::Itemize);

        let mut buffer = Vec::new();
        list.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn push_item_to_list() {
        let mut list = List::new(ListKind::Itemize);

        assert_eq!(list.items.len(), 0);
        list.push("Hello World");
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn render_list_with_items() {
        let should_be = r"\begin{itemize}
\item This
\item is
\item a
\item list!
\end{itemize}
";
        let mut list = List::new(ListKind::Itemize);
        list.push("This").push("is").push("a").push("list!");

        let mut buffer = Vec::new();
        list.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }
}
