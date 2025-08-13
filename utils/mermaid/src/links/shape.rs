use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) enum LinkShape {
    #[default]
    ArrowHead,
    ArrowHeadBidirectional,
    Open,
    Dotted,
    DottedBidirectional,
    Thick,
    ThickBidirectional,
    Invisible,
    CircleEdge,
    CircleEdgeBidirectional,
    CrossEdge,
    CrossEdgeBidirectional,
}

impl Display for LinkShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrowHead => write!(f, "-->"),
            Self::ArrowHeadBidirectional => write!(f, "<-->"),
            Self::Open => write!(f, "---"),
            Self::Dotted => write!(f, "-.->"),
            Self::DottedBidirectional => write!(f, "<-.->"),
            Self::Thick => write!(f, "==>"),
            Self::ThickBidirectional => write!(f, "<==>"),
            Self::Invisible => write!(f, "~~~"),
            Self::CircleEdge => write!(f, "--o"),
            Self::CircleEdgeBidirectional => write!(f, "o--o"),
            Self::CrossEdge => write!(f, "--x"),
            Self::CrossEdgeBidirectional => write!(f, "x--x"),
        }
    }
}
