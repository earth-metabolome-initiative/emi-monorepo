//! Submodule defining the possible shapes for nodes in Mermaid diagrams.
use std::{fmt::Display, str::FromStr};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// `FlowchartNodeShape` represents all supported node shapes for Mermaid
/// diagrams.
pub enum FlowchartNodeShape {
    /// Standard process shape
    #[default]
    Rectangle,
    /// Represents an event
    RoundEdges,
    /// Terminal point
    StadiumShape,
    /// Subprocess
    Subprocess,
    /// Database storage
    Cylinder,
    /// Starting point
    Circle,
    /// Odd shape
    Odd,
    /// Decision-making step
    Diamond,
    /// Preparation or condition step
    Hexagon,
    /// Represents input or output (Lean right parallelogram)
    LRParallelogram,
    /// Represents output or input (Lean left parallelogram)
    LLParallelogram,
    /// Priority action (Base bottom trapezoid)
    Trapezoid,
    /// Manual task (Base top trapezoid)
    ReverseTrapezoid,
    /// Represents a stop point
    DoubleCircle,
    /// Represents a card
    NotchedRectangle,
    /// Lined/Shaded process shape
    Linedrectangle,
    /// Small starting point
    SmallCircle,
    /// Stop point
    FramedCircle,
    /// Fork or join in process flow
    LongRectangle,
    /// Represents a collate operation
    Hourglass,
    /// Adds a comment (Left curly brace)
    LeftCurlyBrace,
    /// Adds a comment (Right curly brace)
    RightCurlyBrace,
    /// Adds a comment (Braces on both sides)
    CurlyBraces,
    /// Communication link
    LightningBolt,
    /// Represents a document
    Document,
    /// Represents a delay
    HalfRoundedRectangle,
    /// Direct access storage
    HorizontalCylinder,
    /// Disk storage
    LinedCylinder,
    /// Represents a display
    CurvedTrapezoid,
    /// Divided process shape
    DividedRectangle,
    /// Extraction process
    SmallTriangle,
    /// Internal storage
    WindowPane,
    /// Junction point
    FilledCircle,
    /// Lined document
    LinedDocument,
    /// Loop limit step
    NotchedPentagon,
    /// Manual file operation
    FlippedTriangle,
    /// Manual input step
    SlopedRectangle,
    /// Multiple documents
    StackedDocument,
    /// Multiple processes
    StackedRectangle,
    /// Paper tape
    Flag,
    /// Stored data
    BowTieRectangle,
    /// Summary
    CrossedCircle,
    /// Tagged document
    TaggedDocument,
    /// Tagged process
    TaggedRectangle,
    /// Subprocess (framed rectangle)
    FramedRectangle,
    /// Text block
    TextBlock,
}

impl Display for FlowchartNodeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rectangle => write!(f, "rect"),
            Self::RoundEdges => write!(f, "rounded"),
            Self::StadiumShape => write!(f, "stadium"),
            Self::Subprocess => write!(f, "subproc"),
            Self::Cylinder => write!(f, "cyl"),
            Self::Circle => write!(f, "circle"),
            Self::Odd => write!(f, "odd"),
            Self::Diamond => write!(f, "diamond"),
            Self::Hexagon => write!(f, "hex"),
            Self::LRParallelogram => write!(f, "lean-r"),
            Self::LLParallelogram => write!(f, "lean-l"),
            Self::Trapezoid => write!(f, "trap-b"),
            Self::ReverseTrapezoid => write!(f, "trap-t"),
            Self::DoubleCircle => write!(f, "dbl-circ"),
            Self::NotchedRectangle => write!(f, "notch-rect"),
            Self::Linedrectangle => write!(f, "lin-rect"),
            Self::SmallCircle => write!(f, "sm-circ"),
            Self::FramedCircle => write!(f, "framed-circle"),
            Self::LongRectangle => write!(f, "fork"),
            Self::Hourglass => write!(f, "hourglass"),
            Self::LeftCurlyBrace => write!(f, "comment"),
            Self::RightCurlyBrace => write!(f, "brace-r"),
            Self::CurlyBraces => write!(f, "braces"),
            Self::LightningBolt => write!(f, "bolt"),
            Self::Document => write!(f, "doc"),
            Self::HalfRoundedRectangle => write!(f, "delay"),
            Self::HorizontalCylinder => write!(f, "das"),
            Self::LinedCylinder => write!(f, "lin-cyl"),
            Self::CurvedTrapezoid => write!(f, "curv-trap"),
            Self::DividedRectangle => write!(f, "div-rect"),
            Self::SmallTriangle => write!(f, "tri"),
            Self::WindowPane => write!(f, "win-pane"),
            Self::FilledCircle => write!(f, "f-circ"),
            Self::LinedDocument => write!(f, "lin-doc"),
            Self::NotchedPentagon => write!(f, "notch-pent"),
            Self::FlippedTriangle => write!(f, "flip-tri"),
            Self::SlopedRectangle => write!(f, "sl-rect"),
            Self::StackedDocument => write!(f, "docs"),
            Self::StackedRectangle => write!(f, "processes"),
            Self::Flag => write!(f, "flag"),
            Self::BowTieRectangle => write!(f, "bow-rect"),
            Self::CrossedCircle => write!(f, "cross-circ"),
            Self::TaggedDocument => write!(f, "tag-doc"),
            Self::TaggedRectangle => write!(f, "tag-rect"),
            Self::FramedRectangle => write!(f, "fr-rect"),
            Self::TextBlock => write!(f, "text"),
        }
    }
}

impl FromStr for FlowchartNodeShape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            // Rectangle
            "rect" | "rectangle" | "proc" | "process" => Ok(Self::Rectangle),
            // Rounded Rectangle
            "rounded" | "event" => Ok(Self::RoundEdges),
            // Stadium
            "stadium" | "pill" | "terminal" => Ok(Self::StadiumShape),
            // Subprocess
            "subproc" | "subprocess" | "subroutine" | "framed-rectangle" => Ok(Self::Subprocess),
            // Cylinder
            "cyl" | "cylinder" | "database" | "db" => Ok(Self::Cylinder),
            // Circle
            "circle" | "circ" => Ok(Self::Circle),
            // Odd
            "odd" => Ok(Self::Odd),
            // Diamond
            "diamond" | "diam" | "decision" | "question" => Ok(Self::Diamond),
            // Hexagon
            "hex" | "hexagon" | "prepare" => Ok(Self::Hexagon),
            // Lean right parallelogram
            "lean-r" | "lean-right" | "in-out" => Ok(Self::LRParallelogram),
            // Lean left parallelogram
            "lean-l" | "lean-left" | "out-in" => Ok(Self::LLParallelogram),
            // Base bottom trapezoid
            "trap-b" | "trapezoid" | "priority" | "trapezoid-bottom" => Ok(Self::Trapezoid),
            // Base top trapezoid
            "trap-t" | "inv-trapezoid" | "manual" | "trapezoid-top" => Ok(Self::ReverseTrapezoid),
            // Double Circle
            "dbl-circ" | "double-circle" | "stop" => Ok(Self::DoubleCircle),
            // Notched Rectangle
            "notch-rect" | "card" | "notched-rectangle" => Ok(Self::NotchedRectangle),
            // Lined Rectangle
            "lin-rect" | "lin-proc" | "lined-process" | "lined-rectangle" | "shaded-process" => {
                Ok(Self::Linedrectangle)
            }
            // Small Circle
            "sm-circ" | "small-circle" | "start" => Ok(Self::SmallCircle),
            // Framed Circle
            "framed-circle" | "fr-circ" => Ok(Self::FramedCircle),
            // Long Rectangle
            "fork" | "join" => Ok(Self::LongRectangle),
            // Hourglass
            "hourglass" | "collate" => Ok(Self::Hourglass),
            // Left Curly Brace
            "comment" | "brace-l" => Ok(Self::LeftCurlyBrace),
            // Right Curly Brace
            "brace-r" => Ok(Self::RightCurlyBrace),
            // Curly Braces
            "braces" => Ok(Self::CurlyBraces),
            // Lightning Bolt
            "bolt" | "com-link" | "lightning-bolt" => Ok(Self::LightningBolt),
            // Document
            "doc" | "document" => Ok(Self::Document),
            // Half-Rounded Rectangle
            "delay" | "half-rounded-rectangle" => Ok(Self::HalfRoundedRectangle),
            // Horizontal Cylinder
            "das" | "h-cyl" | "horizontal-cylinder" => Ok(Self::HorizontalCylinder),
            // Lined Cylinder
            "lin-cyl" | "disk" | "lined-cylinder" => Ok(Self::LinedCylinder),
            // Curved Trapezoid
            "curv-trap" | "curved-trapezoid" | "display" => Ok(Self::CurvedTrapezoid),
            // Divided Rectangle
            "div-rect" | "div-proc" | "divided-process" | "divided-rectangle" => {
                Ok(Self::DividedRectangle)
            }
            // Small Triangle
            "tri" | "extract" | "triangle" => Ok(Self::SmallTriangle),
            // Window Pane
            "win-pane" | "internal-storage" | "window-pane" => Ok(Self::WindowPane),
            // Filled Circle
            "f-circ" | "filled-circle" | "junction" => Ok(Self::FilledCircle),
            // Lined Document
            "lin-doc" | "lined-document" => Ok(Self::LinedDocument),
            // Notched Pentagon
            "notch-pent" | "loop-limit" | "notched-pentagon" => Ok(Self::NotchedPentagon),
            // Flipped Triangle
            "flip-tri" | "flipped-triangle" | "manual-file" => Ok(Self::FlippedTriangle),
            // Sloped Rectangle
            "sl-rect" | "manual-input" | "sloped-rectangle" => Ok(Self::SlopedRectangle),
            // Stacked Document
            "docs" | "documents" | "st-doc" | "stacked-document" => Ok(Self::StackedDocument),
            // Stacked Rectangle
            "processes" | "procs" | "st-rect" | "stacked-rectangle" => Ok(Self::StackedRectangle),
            // Flag
            "flag" | "paper-tape" => Ok(Self::Flag),
            // Bow Tie Rectangle
            "bow-rect" | "bow-tie-rectangle" | "stored-data" => Ok(Self::BowTieRectangle),
            // Crossed Circle
            "cross-circ" | "crossed-circle" | "summary" => Ok(Self::CrossedCircle),
            // Tagged Document
            "tag-doc" | "tagged-document" => Ok(Self::TaggedDocument),
            // Tagged Rectangle
            "tag-rect" | "tag-proc" | "tagged-process" | "tagged-rectangle" => {
                Ok(Self::TaggedRectangle)
            }
            // Framed Rectangle (added for completeness)
            "fr-rect" => Ok(Self::FramedRectangle),
            // Text Block
            "text" | "text-block" => Ok(Self::TextBlock),
            _ => Err(()),
        }
    }
}
