//! Submodule defining the struct for building a flowchart edge.

use std::rc::Rc;

use crate::{
    diagrams::flowchart::{
        curve_styles::CurveStyle, flowchart_edge::FlowchartEdge, flowchart_node::FlowchartNode,
    },
    errors::EdgeError,
    shared::{StyleClass, StyleClassError, StyleProperty, generic_edge::GenericEdgeBuilder},
    traits::EdgeBuilder,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating a `FlowchartEdge`.
pub struct FlowchartEdgeBuilder {
    /// Unique identifier for the edge.
    id: Option<usize>,
    /// Underlying generic edge builder.
    edge_builder: GenericEdgeBuilder<FlowchartNode>,
    /// Classes associated with the edge.
    style_classes: Vec<Rc<StyleClass>>,
    /// Style properties for the edge.
    style_properties: Vec<StyleProperty>,
    /// The curve style for the edge.
    curve_style: CurveStyle,
    /// Length of the edge.
    length: u8,
}

impl FlowchartEdgeBuilder {
    #[must_use]
    /// Creates a new `FlowchartEdgeBuilder`.
    pub fn id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    /// Adds a style class to the edge builder.
    ///
    /// # Arguments
    ///
    /// * `class`: The style class to be added
    ///
    /// # Errors
    ///
    /// * If the class is already present, an error is returned.
    pub fn style_class(mut self, class: Rc<StyleClass>) -> Result<Self, StyleClassError> {
        if self.style_classes.iter().any(|c| c.name() == class.name()) {
            return Err(StyleClassError::DuplicateClass(class.name().to_string()));
        }
        self.style_classes.push(class);
        Ok(self)
    }

    /// Adds a style property to the edge builder.
    ///
    /// # Arguments
    ///
    /// * `property`: The style property to be added.
    ///
    /// # Errors
    ///
    /// * If the property is already present, an error is returned.
    pub fn style_property(mut self, property: StyleProperty) -> Result<Self, StyleClassError> {
        if self.style_properties.iter().any(|p| p.is_same_type(property)) {
            return Err(StyleClassError::DuplicateProperty(property));
        }
        self.style_properties.push(property);
        Ok(self)
    }

    #[must_use]
    /// Sets the curve style for the edge.
    pub fn curve_style(mut self, style: CurveStyle) -> Self {
        self.curve_style = style;
        self
    }

    #[must_use]
    /// Sets the length of the edge.
    pub fn length(mut self, length: u8) -> Self {
        self.length = length;
        self
    }
}

impl Default for FlowchartEdgeBuilder {
    fn default() -> Self {
        Self {
            id: None,
            edge_builder: GenericEdgeBuilder::default(),
            style_classes: Vec::new(),
            style_properties: Vec::new(),
            curve_style: CurveStyle::default(),
            length: 1,
        }
    }
}

impl TryFrom<FlowchartEdgeBuilder> for FlowchartEdge {
    type Error = EdgeError;

    fn try_from(builder: FlowchartEdgeBuilder) -> Result<Self, Self::Error> {
        if builder.length == 0 {
            return Err(EdgeError::InvalidLength);
        }

        Ok(FlowchartEdge {
            id: builder.id.ok_or(EdgeError::MissingId)?,
            edge: builder.edge_builder.try_into()?,
            style_classes: builder.style_classes,
            style_properties: builder.style_properties,
            curve_style: builder.curve_style,
            length: builder.length,
        })
    }
}

impl EdgeBuilder for FlowchartEdgeBuilder {
    type Edge = FlowchartEdge;
    type Node = FlowchartNode;
    type Error = EdgeError;

    fn build(self) -> Result<Self::Edge, Self::Error> {
        self.try_into()
    }

    fn source(mut self, node: std::rc::Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.source(node)?;
        Ok(self)
    }

    fn destination(mut self, node: std::rc::Rc<Self::Node>) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.destination(node)?;
        Ok(self)
    }

    fn label<S: ToString>(mut self, label: S) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.label(label)?;
        Ok(self)
    }

    fn line_style(mut self, style: crate::shared::LineStyle) -> Self {
        self.edge_builder = self.edge_builder.line_style(style);
        self
    }

    fn left_arrow_shape(mut self, shape: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.left_arrow_shape(shape)?;
        Ok(self)
    }

    fn right_arrow_shape(mut self, shape: crate::shared::ArrowShape) -> Result<Self, Self::Error> {
        self.edge_builder = self.edge_builder.right_arrow_shape(shape)?;
        Ok(self)
    }
}
