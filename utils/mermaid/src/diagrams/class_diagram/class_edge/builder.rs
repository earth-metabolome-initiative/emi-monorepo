//! Submodule defining a builder for class edges in class diagrams
//! in Mermaid syntax.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::Builder,
};

use crate::{
    diagrams::class_diagram::{
        class_edge::{ClassEdge, multiplicity::Multiplicity},
        class_node::ClassNode,
    },
    errors::EdgeError,
    shared::generic_edge::{GenericEdgeAttribute, GenericEdgeBuilder},
    traits::EdgeBuilder,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating a `ClassEdge`.
pub struct ClassEdgeBuilder {
    /// Underlying generic edge builder.
    edge_builder: GenericEdgeBuilder<ClassNode>,
    /// Left Multiplicity of the edge.
    left_multiplicity: Option<Multiplicity>,
    /// Right Multiplicity of the edge.
    right_multiplicity: Option<Multiplicity>,
}

impl ClassEdgeBuilder {
    /// Sets the left multiplicity of the edge.
    pub fn left_multiplicity(&mut self, multiplicity: Multiplicity) -> &mut Self {
        self.left_multiplicity = Some(multiplicity);
        self
    }

    /// Sets the right multiplicity of the edge.
    pub fn right_multiplicity(&mut self, multiplicity: Multiplicity) -> &mut Self {
        self.right_multiplicity = Some(multiplicity);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of attributes that can be set on a `ClassEdgeAttribute`.
pub enum ClassEdgeAttribute {
    /// Underlying generic edge attributes.
    Generic(GenericEdgeAttribute),
    /// Left multiplicity of the edge.
    LeftMultiplicity,
    /// Right multiplicity of the edge.
    RightMultiplicity,
}

impl From<GenericEdgeAttribute> for ClassEdgeAttribute {
    fn from(attr: GenericEdgeAttribute) -> Self {
        ClassEdgeAttribute::Generic(attr)
    }
}

impl Display for ClassEdgeAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassEdgeAttribute::Generic(attr) => write!(f, "{attr}"),
            ClassEdgeAttribute::LeftMultiplicity => write!(f, "left_multiplicity"),
            ClassEdgeAttribute::RightMultiplicity => write!(f, "right_multiplicity"),
        }
    }
}

impl IsCompleteBuilder for ClassEdgeBuilder {
    fn is_complete(&self) -> bool {
        self.edge_builder.is_complete()
    }
}

impl Attributed for ClassEdgeBuilder {
    type Attribute = ClassEdgeAttribute;
}

impl Builder for ClassEdgeBuilder {
    type Error = EdgeError<Self::Attribute>;
    type Object = ClassEdge;

    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(ClassEdge {
            edge: self.edge_builder.build()?,
            left_multiplicity: self.left_multiplicity,
            right_multiplicity: self.right_multiplicity,
        })
    }
}

impl EdgeBuilder for ClassEdgeBuilder {
    type Edge = ClassEdge;
    type Node = ClassNode;

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
