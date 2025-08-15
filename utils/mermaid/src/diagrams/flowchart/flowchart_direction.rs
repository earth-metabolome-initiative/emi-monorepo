//! Submodule defining whether a flowchart is meant to extend in a horizontal or
//! vertical direction.

use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the direction of a flowchart in Mermaid diagrams.
pub enum FlowchartDirection {
    /// The flowchart extends horizontally.
    #[default]
    LeftToRight,
    /// The flowchart extends vertically.
    TopToBottom,
    /// The flowchart extends in a right-to-left direction.
    RightToLeft,
    /// The flowchart extends in a bottom-to-top direction.
    BottomToTop,
}

impl Display for FlowchartDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match self {
            FlowchartDirection::LeftToRight => "LR",
            FlowchartDirection::TopToBottom => "TB",
            FlowchartDirection::RightToLeft => "RL",
            FlowchartDirection::BottomToTop => "BT",
        };
        write!(f, "direction {direction}",)
    }
}
