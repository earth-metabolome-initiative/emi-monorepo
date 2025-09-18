//! Submodule defining a builder for guided procedures.

use std::fmt::Display;

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use core_structures::User;

use crate::{
    GuidedProcedure, ProcedureTemplateGraph, guided_procedure::GPBListener, ptg_visitor::PTGVisitor,
};

#[derive(Debug)]
/// Struct representing a builder for guided procedures.
pub struct GuidedProcedureBuilder<'graph, C> {
    graph: Option<&'graph ProcedureTemplateGraph>,
    connection: Option<&'graph mut C>,
    /// The user which is conducting the guided procedure.
    author: Option<&'graph User>,
}

impl<'graph, C> GuidedProcedureBuilder<'graph, C> {
    /// Sets the procedure template graph for the guided procedure.
    pub fn graph(mut self, graph: &'graph ProcedureTemplateGraph) -> Self {
        self.graph = Some(graph);
        self
    }

    /// Sets the connection for the guided procedure.
    pub fn connection(mut self, connection: &'graph mut C) -> Self {
        self.connection = Some(connection);
        self
    }

    /// Sets the user ID for the guided procedure.
    pub fn author(mut self, author: &'graph User) -> Self {
        self.author = Some(author);
        self
    }
}

impl<C> Default for GuidedProcedureBuilder<'_, C> {
    fn default() -> Self {
        Self { graph: None, connection: None, author: None }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Attributes that can be set in the `GuidedProcedureBuilder`.
pub enum GuidedProcedureAttribute {
    Graph,
    Connection,
    Author,
}

impl Display for GuidedProcedureAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuidedProcedureAttribute::Graph => write!(f, "graph"),
            GuidedProcedureAttribute::Connection => write!(f, "connection"),
            GuidedProcedureAttribute::Author => write!(f, "author"),
        }
    }
}

impl<C> Attributed for GuidedProcedureBuilder<'_, C> {
    type Attribute = GuidedProcedureAttribute;
}

impl<C> IsCompleteBuilder for GuidedProcedureBuilder<'_, C> {
    fn is_complete(&self) -> bool {
        self.graph.is_some() && self.connection.is_some() && self.author.is_some()
    }
}

impl<'graph, C> Builder for GuidedProcedureBuilder<'graph, C> {
    type Error = BuilderError<GuidedProcedureAttribute>;
    type Object = GuidedProcedure<'graph, C>;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let graph =
            self.graph.ok_or(BuilderError::IncompleteBuild(GuidedProcedureAttribute::Graph))?;
        let author =
            self.author.ok_or(BuilderError::IncompleteBuild(GuidedProcedureAttribute::Author))?;
        let connection = self
            .connection
            .ok_or(BuilderError::IncompleteBuild(GuidedProcedureAttribute::Connection))?;
        Ok(GuidedProcedure {
            visitor: PTGVisitor::new(graph, GPBListener::new(graph, author, connection)),
        })
    }
}
