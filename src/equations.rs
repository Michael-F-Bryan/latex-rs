use std::io::Write;
use std::slice::Iter;
use std::ops::Deref;

use super::Renderable;
use errors::*;


/// A single equation.
///
/// # Examples
///
/// The `Equation` struct is designed to be added to an `Align` object.
/// Creating one is as simple as using the constructor.
///
/// ```rust
/// use latex::Equation;
///
/// let eq = Equation::new("y &= mx + c");
/// ```
///
/// For convenience, you can also convert from a `&str` to an `Equation` using
/// `into()`.
///
/// ```rust
/// use latex::Equation;
///
/// let eq: Equation = "y &= mx + c".into();
/// ```
///
/// You can assign a `label` to an equation so it can be referenced later.
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

impl Renderable for Equation {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        write!(writer, r"{}", self.text)?;
        if let Some(ref label) = self.label {
            write!(writer, r" \label{{{}}}", label)?;
        }
        if self.not_numbered {
            write!(writer, r" \nonumber")?;
        }

        writeln!(writer, r" \\")?;
        Ok(())
    }
}


/// A list of equations to be used in an `align` environment.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Equations {
    items: Vec<Equation>,
}

impl Equations {
    /// Create an empty equation list.
    pub fn new() -> Equations {
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

impl Renderable for Equations {
    fn render<W>(&self, writer: &mut W) -> Result<()>
        where W: Write
    {
        writeln!(writer, r"\begin{{align}}")?;

        for item in &self.items {
            item.render(writer)?;
        }

        writeln!(writer, r"\end{{align}}")?;
        Ok(())
    }
}


impl<'a> From<&'a str> for Equation {
    fn from(other: &'a str) -> Equation {
        Equation::new(other)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_empty_align() {
        let should_be = "\\begin{align}\n\\end{align}\n";
        let equations = Equations::new();

        let mut buffer = Vec::new();
        equations.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_simple_equation() {
        let should_be = "x &= y + \\sigma \\\\\n";
        let eq = Equation::new(r"x &= y + \sigma");

        let mut buffer = Vec::new();
        eq.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn render_several_equations() {
        let should_be = r"\begin{align}
E &= m c^2 \label{eq:mass-energy-equivalence} \\
y &= m x + c \\
\end{align}
";
        let mut equations = Equations::new();

        let mut eq = Equation::new("E &= m c^2");
        eq.label("eq:mass-energy-equivalence");
        equations.push(eq).push("y &= m x + c");

        let mut buffer = Vec::new();
        equations.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn equation_with_label() {
        let should_be = "E &= m c^2 \\label{eq:mass-energy-equivalence} \\\\\n";
        let mut eq = Equation::new("E &= m c^2");
        eq.label("eq:mass-energy-equivalence");

        let mut buffer = Vec::new();
        eq.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }

    #[test]
    fn equation_with_no_numbering() {
        let should_be = "E &= m c^2 \\nonumber \\\\\n";
        let mut eq = Equation::new("E &= m c^2");
        eq.not_numbered();

        let mut buffer = Vec::new();
        eq.render(&mut buffer).unwrap();

        assert_eq!(String::from_utf8(buffer).unwrap(), should_be);
    }
}
