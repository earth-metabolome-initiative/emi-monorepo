//! Submodule providing an enumeration of possible curve styles for flowchart
//! edges in Mermaid diagrams.

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents the curve styles available for flowchart edges in Mermaid syntax.
pub enum CurveStyle {
    /// Basis curve style.
    #[default]
    Basis,
    /// `BumpX` curve style.
    BumpX,
    /// `BumpY` curve style.
    BumpY,
    /// `Cardinal` curve style.
    Cardinal,
    /// `CatmullRom` curve style.
    CatmullRom,
    /// `Linear` curve style.
    Linear,
    /// `MonotoneX` curve style.
    MonotoneX,
    /// `MonotoneY` curve style.
    MonotoneY,
    /// `Natural` curve style.
    Natural,
    /// `Step` curve style.
    Step,
    /// `StepAfter` curve style.
    StepAfter,
    /// `StepBefore` curve style.
    StepBefore,
}

impl std::fmt::Display for CurveStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurveStyle::Basis => write!(f, "basis"),
            CurveStyle::BumpX => write!(f, "bumpX"),
            CurveStyle::BumpY => write!(f, "bumpY"),
            CurveStyle::Cardinal => write!(f, "cardinal"),
            CurveStyle::CatmullRom => write!(f, "catmullRom"),
            CurveStyle::Linear => write!(f, "linear"),
            CurveStyle::MonotoneX => write!(f, "monotoneX"),
            CurveStyle::MonotoneY => write!(f, "monotoneY"),
            CurveStyle::Natural => write!(f, "natural"),
            CurveStyle::Step => write!(f, "step"),
            CurveStyle::StepAfter => write!(f, "stepAfter"),
            CurveStyle::StepBefore => write!(f, "stepBefore"),
        }
    }
}
