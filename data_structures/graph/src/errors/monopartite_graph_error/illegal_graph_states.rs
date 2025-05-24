//! Submodule defining illegal graph states that, if reached, indicate a bug in
//! some implementation of the graph traits.

use numeric_common_traits::prelude::Bounded;

use crate::traits::MonopartiteGraph;

#[derive(Clone, PartialEq, Eq)]
/// Error enumeration relative to illegal graph states.
pub enum IllegalMonopartiteGraphState<G: MonopartiteGraph + ?Sized> {
    /// The maximal number of nodes of the graph is larger than the number of
    /// nodes that can be represented by the graph's node ID type. This
    /// should be impossible to reach, and indicates some bug in the
    /// implementation of the graph traits.
    TooManyNodes {
        /// The number of nodes that was reported.
        number_of_nodes: usize,
    },
    /// `PhantomPlaceholder`
    PhantomPlaceholder(core::marker::PhantomData<G>),
}

impl<G: MonopartiteGraph + ?Sized> core::fmt::Display for IllegalMonopartiteGraphState<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            IllegalMonopartiteGraphState::TooManyNodes { number_of_nodes } => {
                write!(
                    f,
                    concat!(
                        "The maximal number of nodes of the graph is larger than the number of nodes that can be ",
                        "represented by the graph's node ID type ({}). This should be impossible to reach, and indicates ",
                        "some bug in the implementation of the graph traits. The number of nodes that was reported ",
                        "was {}."
                    ),
                    G::NodeId::MAX,
                    number_of_nodes
                )
            }
            IllegalMonopartiteGraphState::PhantomPlaceholder(_) => {
                unreachable!()
            }
        }
    }
}

impl<G: MonopartiteGraph> core::error::Error for IllegalMonopartiteGraphState<G> {}

impl<G: MonopartiteGraph + ?Sized> From<IllegalMonopartiteGraphState<G>>
    for crate::errors::MonopartiteError<G>
{
    fn from(error: IllegalMonopartiteGraphState<G>) -> Self {
        Self::IllegalGraphState(error)
    }
}

impl<G: MonopartiteGraph + ?Sized> core::fmt::Debug for IllegalMonopartiteGraphState<G> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}
