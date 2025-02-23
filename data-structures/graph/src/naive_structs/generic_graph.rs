//! Simple generic graph implementation.

use crate::{
    errors::builder::graph::GraphBuilderError,
    traits::{BidirectionalVocabulary, Edges, Graph, Vocabulary, VocabularyRef},
};

/// A generic graph.
pub struct GenericGraph<Sources, Destinations, Edges> {
    /// The sources of the graph.
    sources: Sources,
    /// The destinations of the graph.
    destinations: Destinations,
    /// The edges of the graph.
    edges: Edges,
}

impl<S, D, E> TryFrom<(S, D, E)> for GenericGraph<S, D, E>
where
    Self: Graph,
{
    type Error = GraphBuilderError<GenericGraph<S, D, E>>;

    fn try_from((sources, destinations, edges): (S, D, E)) -> Result<Self, Self::Error> {
        // TODO! CHECK COMPATIBILITY!
        Ok(Self { sources, destinations, edges })
    }
}

impl<S, D, E> Graph for GenericGraph<S, D, E>
where
    S: BidirectionalVocabulary + VocabularyRef + Vocabulary<SourceSymbol = E::SourceNodeId>,
    D: BidirectionalVocabulary + VocabularyRef + Vocabulary<SourceSymbol = E::DestinationNodeId>,
    E: Edges,
{
    type SourceNodeId = E::SourceNodeId;
    type DestinationNodeId = E::DestinationNodeId;
    type SourceNodeSymbol = S::DestinationSymbol;
    type DestinationNodeSymbol = D::DestinationSymbol;
    type Sources = S;
    type Destinations = D;
    type Edges = E;
    type EdgeId = E::EdgeId;
    type Edge = E::Edge;

    fn source_vocabulary(&self) -> &Self::Sources {
        &self.sources
    }

    fn destination_vocabulary(&self) -> &Self::Destinations {
        &self.destinations
    }

    fn edges(&self) -> &Self::Edges {
        &self.edges
    }
}
