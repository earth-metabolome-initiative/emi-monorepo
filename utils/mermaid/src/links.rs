mod shape;
use std::fmt::{Display, format};

pub(crate) use shape::LinkShape;

use super::colors::Colors;

#[derive(Clone, Debug)]
pub(crate) struct Link {
    link_id: u32,
    shape: LinkShape,
    label: Option<String>,
    config: LinkConfig,
}

/// Represents the color and the line width of a link in the Mermaid diagram.
#[derive(Clone, Debug)]
pub(crate) struct LinkConfig {
    link_id: u32,
    stroke_color: Colors,
    stroke_width: u8,
}

impl Link {
    pub fn new(
        link_id: u32,
        shape: LinkShape,
        label: Option<String>,
        link_color: Colors,
        stroke_width: u8,
    ) -> Self {
        Link {
            link_id,
            shape,
            label,
            config: LinkConfig { link_id, stroke_color: link_color, stroke_width },
        }
    }

    pub fn get_config(&self) -> &LinkConfig {
        &self.config
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label_str = match &self.label {
            Some(label) => write!(f, "{}|{}|", self.shape, label),
            None => write!(f, "{}", self.shape),
        };
        label_str
    }
}

impl Display for LinkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "linkStyle {} stroke:{},stroke-width:{}px",
            self.link_id, self.stroke_color, self.stroke_width
        )
    }
}

impl Default for Link {
    fn default() -> Self {
        Link {
            link_id: 0,
            shape: LinkShape::default(),
            label: None,
            config: LinkConfig { link_id: 0, stroke_color: Colors::default(), stroke_width: 1 },
        }
    }
}
