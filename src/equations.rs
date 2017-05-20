use std::slice::Iter;
use std::ops::Deref;

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
    text: String,
    label: Option<String>,
    not_numbered: bool,
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

    //FIXME: These getters and setters are a bit of a hack because pub(restricted) isn't stable

    /// Give the equation a label.
    pub fn label(&mut self, name: &str) -> &mut Self {
        self.label = Some(name.to_string());
        self
    }

    /// Set the equation's text.
    pub fn text(&mut self, src: &str) -> &mut Self {
        self.text = src.to_string();
        self
    }

    /// Set whether the `\nonumber` command should be used to ignore numbering
    /// for this equation.
    pub fn not_numbered(&mut self) -> &mut Self {
        self.not_numbered = true;
        self
    }

    /// Get the equation's text.
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Get the equation label, if there is one.
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_ref().map(Deref::deref)
    }

    /// Is this equation numbered?
    pub fn is_numbered(&self) -> bool {
        !self.not_numbered
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
