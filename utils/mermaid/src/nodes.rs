mod builder;
mod click_event;
mod error;
mod shape;
use std::{fmt::Display, rc::Rc};

pub use error::NodeError;
pub use shape::NodeShape;

use crate::{
    nodes::click_event::ClickEvent,
    shared::{label::Label, style_class::StyleClass},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a node in a Mermaid diagram.
pub struct Node {
    /// Unique identifier for the node.
    id: u32,
    /// The visual shape of the node (e.g., rectangle, circle, etc.).
    shape: NodeShape,
    /// The text label displayed inside the node.
    label: Label,
    /// The style classes applied to the node.
    style_classes: Vec<Rc<StyleClass>>,
    /// The click event associated with the node, if any.
    click_event: Option<ClickEvent>,
}

impl Node {
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}@{{label: {}, shape: {}}}", self.id, self.label, self.shape)?;

        for style_class in &self.style_classes {
            write!(f, "class {} {}", self.id, style_class)?;
        }

        if let Some(click_event) = &self.click_event {
            write!(f, "{}", click_event)?;
        }

        Ok(())
    }
}
