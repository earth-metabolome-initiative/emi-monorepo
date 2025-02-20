//! Submodule defining illegal graph states that, if reached, indicate a bug in some implementation
//! of the graph traits.

use crate::traits::Graph;
use algebra::prelude::Bounded;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error enumeration relative to illegal graph states.
pub enum IllegalGraphState<G: Graph + ?Sized> {
    /// The maximal number of nodes of the graph is larger than the number of nodes that can be
    /// represented by the graph's node ID type. This should be impossible to reach, and indicates
    /// some bug in the implementation of the graph traits.
    TooManyNodes {
        /// The number of nodes that was reported.
        number_of_nodes: usize,
    },
    /// `PhantomPlaceholder`
    PhantomPlaceholder(core::marker::PhantomData<G>),
}

impl<G: Graph> core::fmt::Display for IllegalGraphState<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            IllegalGraphState::TooManyNodes { number_of_nodes } => {
                write!(
                    f,
                    concat!(
					"The maximal number of nodes of the graph is larger than the number of nodes that can be ",
					"represented by the graph's node ID type ({}). This should be impossible to reach, and indicates ",
					"some bug in the implementation of the graph traits. The number of nodes that was reported ",
					"was {}."
				), G::NodeId::MAX, number_of_nodes)
            }
            IllegalGraphState::PhantomPlaceholder(_) => {
                unreachable!()
            }
        }
    }
}

impl<G: Graph> core::error::Error for IllegalGraphState<G> {}

impl<G: Graph + ?Sized> From<IllegalGraphState<G>> for super::GraphError<G> {
    fn from(error: IllegalGraphState<G>) -> Self {
        super::GraphError::IllegalGraphState(error)
    }
}

impl<G: Graph + ?Sized> From<IllegalGraphState<G>> for crate::errors::Error<G> {
    fn from(error: IllegalGraphState<G>) -> Self {
        crate::errors::Error::GraphError(super::GraphError::IllegalGraphState(error))
    }
}
