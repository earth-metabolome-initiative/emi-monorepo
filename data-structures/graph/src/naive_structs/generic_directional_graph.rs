//! Simple generic graph implementation.

use crate::{
    errors::builder::graph::GraphBuilderError,
    traits::{
        BidirectionalVocabulary, DirectedEdges, Graph, Vocabulary, VocabularyRef,
    },
};

/// A generic graph.
pub struct GenericDirectionalGraph<Sources, Edges> {
    /// The sources of the graph.
    sources: Sources,
    /// The edges of the graph.
    edges: Edges,
}

impl<S, E> TryFrom<(S, E)> for GenericDirectionalGraph<S, E>
where
    Self: Graph,
{
    type Error = GraphBuilderError<GenericDirectionalGraph<S, E>>;

    fn try_from((sources, edges): (S, E)) -> Result<Self, Self::Error> {
        // TODO! CHECK COMPATIBILITY!
        Ok(Self { sources, edges })
    }
}

impl<S, E> Graph for GenericDirectionalGraph<S, E>
where
    S: BidirectionalVocabulary + VocabularyRef + Vocabulary<SourceSymbol = E::NodeId>,
    E: DirectedEdges,
{
    type SourceNodeId = E::NodeId;
    type DestinationNodeId = E::NodeId;
    type SourceNodeSymbol = S::DestinationSymbol;
    type DestinationNodeSymbol = S::DestinationSymbol;
    type Sources = S;
    type Destinations = S;
    type Edges = E;
    type EdgeId = E::EdgeId;
    type Edge = E::Edge;

    fn source_vocabulary(&self) -> &Self::Sources {
        &self.sources
    }

    fn destination_vocabulary(&self) -> &Self::Destinations {
        &self.sources
    }

    fn edges(&self) -> &Self::Edges {
        &self.edges
    }
}
