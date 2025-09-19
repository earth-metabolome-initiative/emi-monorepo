//! The themes enumeration to use for rendering a Mermaid diagram.

use std::fmt::Display;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The themes enumeration to use for rendering a Mermaid diagram.
pub enum Theme {
    /// The classic Mermaid chart theme.
    MermaidChart,
    /// The `Neo` theme, a modern style for diagrams.
    Neo,
    /// The `NeoDark` theme, a dark variant of Neo.
    NeoDark,
    /// The default theme for Mermaid diagrams.
    #[default]
    Default,
    /// The `Forest` theme, with green accents and natural tones.
    Forest,
    /// The `Base` theme, a minimal and clean style.
    Base,
    /// The `Dark` theme, for diagrams with a dark background.
    Dark,
    /// The `Neutral` theme, with muted and balanced colors.
    Neutral,
    /// The `Redux` theme, a vibrant and bold style.
    Redux,
    /// The `ReduxDark` theme, a dark variant of Redux.
    ReduxDark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::MermaidChart => "mc",
                Theme::Neo => "neo",
                Theme::NeoDark => "neo-dark",
                Theme::Default => "default",
                Theme::Forest => "forest",
                Theme::Base => "base",
                Theme::Dark => "dark",
                Theme::Neutral => "neutral",
                Theme::Redux => "redux",
                Theme::ReduxDark => "redux-dark",
            }
        )
    }
}
