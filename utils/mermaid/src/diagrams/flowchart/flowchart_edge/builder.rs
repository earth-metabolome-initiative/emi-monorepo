//! Submodule defining the struct for building a flowchart edge.

use crate::{diagrams::flowchart::curve_styles::CurveStyle, traits::EdgeBuilder};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowchartEdgeBuilder {
    /// The curve style for the edge.
    curve_style: CurveStyle,
    /// Length of the edge.
    length: u8,
}

impl EdgeBuilder for FlowchartEdgeBuilder {}
