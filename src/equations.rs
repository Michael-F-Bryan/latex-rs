use std::slice::Iter;

/// A single equation.
///
/// # Examples
///
/// The `Equation` struct is designed to be added to an `Align` object.
/// Creating one is as simple as using the constructor.
///
/// ```rust
/// # use latex::Equation;
/// let eq = Equation::new("y &= mx + c");
/// ```
///
/// For convenience, you can also convert from a `&str` to an `Equation` using
/// `into()`.
///
/// ```rust
/// # use latex::Equation;
/// let eq: Equation = "y &= mx + c".into();
/// ```
///
/// You can also assign a `label` to an equation so it can be referenced later.
///
/// ```rust
/// # use latex::Equation;
/// # let mut eq: Equation = "y &= mx + c".into();
/// eq.label("basic-linear-equation");
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Equation {
    pub(crate) text: String,
    pub(crate) label: Option<String>,
    pub(crate) not_numbered: bool,
}

impl Equation {
    /// Create a new `Equation`.
    pub fn new<S: AsRef<str>>(src: S) -> Equation {
        Equation {
            text: src.as_ref().to_string(),
            label: None,
            not_numbered: false,
        }
    }

    /// Create an equation which has a label.
    pub fn with_label(label: &str, text: &str) -> Equation {
        let mut eq = Equation::new(text);
        eq.label(label);
        eq
    }

    /// Set the `Equation`'s label.
    pub fn label<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
        self.label = Some(text.as_ref().to_string());
        self
    }

    /// Don't number this equation.
    pub fn not_numbered(&mut self) -> &mut Self {
        self.not_numbered = true;
        self
    }
}

/// A list of equations to be used in an `align` environment.
///
/// # Note
///
/// Using this environment requires you to include the `amsmath` package in
/// your preamble.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Align {
    items: Vec<Equation>,
}

impl Align {
    /// Create an empty equation list.
    pub fn new() -> Align {
        Default::default()
    }

    /// Iterate over each of this equations in the list.
    pub fn iter(&self) -> Iter<Equation> {
        self.items.iter()
    }

    /// Add an equation to the end of the list.
    pub fn push<E: Into<Equation>>(&mut self, eq: E) -> &mut Self {
        self.items.push(eq.into());
        self
    }
}

impl<'a> From<&'a str> for Equation {
    fn from(other: &'a str) -> Equation {
        Equation::new(other)
    }
}


impl<'a> From<&'a str> for Align {
    /// Convert a string into a single equation wrapped in an `align`.
    fn from(other: &'a str) -> Align {
        let mut eq = Align::new();
        eq.push(other);
        eq
    }
}
