//! Submodule defining illegal graph states that, if reached, indicate a bug in
//! some implementation of the graph traits.

use numeric_common_traits::prelude::Bounded;

use crate::traits::BipartiteGraph;

#[derive(Clone, PartialEq, Eq)]
/// Error enumeration relative to illegal graph states.
pub enum IllegalBipartiteGraphState<G: BipartiteGraph + ?Sized> {
    /// The maximal number of left nodes of the graph is larger than the number
    /// of nodes that can be represented by the graph's left node ID type.
    /// This should be impossible to reach, and indicates some bug in the
    /// implementation of the graph traits.
    TooManyLeftNodes {
        /// The number of left nodes that was reported.
        number_of_left_nodes: usize,
    },
    /// The maximal number of right nodes of the graph is larger than the number
    /// of nodes that can be represented by the graph's right node ID type.
    /// This should be impossible to reach, and indicates some bug in the
    /// implementation of the graph traits.
    TooManyRightNodes {
        /// The number of right nodes that was reported.
        number_of_right_nodes: usize,
    },
    /// `PhantomPlaceholder`
    PhantomPlaceholder(core::marker::PhantomData<G>),
}

impl<G: BipartiteGraph + ?Sized> core::fmt::Display for IllegalBipartiteGraphState<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            IllegalBipartiteGraphState::TooManyLeftNodes { number_of_left_nodes } => {
                write!(
                    f,
                    concat!(
                        "The maximal number of left nodes of the graph is larger than the number of left nodes that can be ",
                        "represented by the graph's node ID type ({}). This should be impossible to reach, and indicates ",
                        "some bug in the implementation of the graph traits. The number of left nodes that was reported ",
                        "was {}."
                    ),
                    G::LeftNodeId::MAX,
                    number_of_left_nodes
                )
            }
            IllegalBipartiteGraphState::TooManyRightNodes { number_of_right_nodes } => {
                write!(
                    f,
                    concat!(
                        "The maximal number of right nodes of the graph is larger than the number of right nodes that can be ",
                        "represented by the graph's node ID type ({}). This should be impossible to reach, and indicates ",
                        "some bug in the implementation of the graph traits. The number of right nodes that was reported ",
                        "was {}."
                    ),
                    G::RightNodeId::MAX,
                    number_of_right_nodes
                )
            }
            IllegalBipartiteGraphState::PhantomPlaceholder(_) => {
                unreachable!()
            }
        }
    }
}

impl<G: BipartiteGraph> core::error::Error for IllegalBipartiteGraphState<G> {}

impl<G: BipartiteGraph + ?Sized> From<IllegalBipartiteGraphState<G>>
    for crate::errors::BipartiteError<G>
{
    fn from(error: IllegalBipartiteGraphState<G>) -> Self {
        Self::IllegalGraphState(error)
    }
}

impl<G: BipartiteGraph + ?Sized> core::fmt::Debug for IllegalBipartiteGraphState<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}
