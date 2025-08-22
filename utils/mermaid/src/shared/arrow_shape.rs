//! Submodule defining the possible arrow shapes for links in Mermaid diagrams.

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the shape of an arrow that can be used in Mermaid diagrams.
pub enum ArrowShape {
    /// Arrow shape with a normal arrowhead.
    #[default]
    Normal,
    /// A sharp arrowhead shape.
    Sharp,
    /// X shape arrowhead.
    X,
    /// Circle shape arrowhead.
    Circle,
    /// Triangle shape arrowhead.
    Triangle,
    /// Star shape arrowhead.
    Star,
    /// Shape representing zero or one dependency.
    ZeroOrOne,
    /// Shape representing exactly one dependency.
    ExactlyOne,
    /// Shape representing zero or more dependencies.
    ZeroOrMore,
    /// Shape representing one or more dependencies.
    OneOrMore,
}

impl ArrowShape {
    #[must_use]
    /// Returns the left-oriented arrow shape.
    pub fn left(&self) -> &str {
        match self {
            ArrowShape::Normal => "<",
            ArrowShape::Sharp => "(",
            ArrowShape::X => "x",
            ArrowShape::Circle => "o",
            ArrowShape::Triangle => "<|",
            ArrowShape::Star => "*",
            ArrowShape::ZeroOrOne => "|o",
            ArrowShape::ExactlyOne => "||",
            ArrowShape::ZeroOrMore => "}o",
            ArrowShape::OneOrMore => "}|",
        }
    }

    #[must_use]
    /// Returns the right-oriented arrow shape.
    pub fn right(&self) -> &str {
        match self {
            ArrowShape::Normal => ">",
            ArrowShape::Sharp => ")",
            ArrowShape::X => "x",
            ArrowShape::Circle => "o",
            ArrowShape::Triangle => "|>",
            ArrowShape::Star => "*",
            ArrowShape::ZeroOrOne => "o|",
            ArrowShape::ExactlyOne => "||",
            ArrowShape::ZeroOrMore => "o{",
            ArrowShape::OneOrMore => "|{",
        }
    }
}
