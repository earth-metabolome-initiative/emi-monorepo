use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Colors {
    Red,
    Maroon,
    Brown,
    Tan,
    Orange,
    Peach,
    Gold,
    Yellow,
    Lime,
    Olive,
    Green,
    Teal,
    Cyan,
    Blue,
    Navy,
    Purple,
    Magenta,
    Pink,
    Grey,
    Silver,
    White,
    #[default]
    Black,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "#ff0000"),
            Self::Maroon => write!(f, "#800000"),
            Self::Brown => write!(f, "#a52a2a"),
            Self::Tan => write!(f, "#d2b48c"),
            Self::Orange => write!(f, "#ffa500"),
            Self::Peach => write!(f, "#ffdab9"),
            Self::Gold => write!(f, "#ffd700"),
            Self::Yellow => write!(f, "#ffff00"),
            Self::Lime => write!(f, "#00ff00"),
            Self::Olive => write!(f, "#808000"),
            Self::Green => write!(f, "#008000"),
            Self::Teal => write!(f, "#008080"),
            Self::Cyan => write!(f, "#00ffff"),
            Self::Blue => write!(f, "#0000ff"),
            Self::Navy => write!(f, "#000080"),
            Self::Purple => write!(f, "#800080"),
            Self::Magenta => write!(f, "#ff00ff"),
            Self::Pink => write!(f, "#ffc0cb"),
            Self::Grey => write!(f, "#808080"),
            Self::Silver => write!(f, "#c0c0c0"),
            Self::White => write!(f, "#ffffff"),
            Self::Black => write!(f, "#000000"),
        }
    }
}
