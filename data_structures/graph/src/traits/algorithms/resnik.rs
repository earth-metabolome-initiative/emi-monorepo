//! Submodule providing `Resnik` trait
//! implementation.

mod error;
use error::ResnikError;
use functional_properties::prelude::ScalarSimilarity;

use crate::traits::MonoplexMonopartiteGraph;

/// Struct to provide methods to compute Resnik Similarity Score
pub struct ResnikResult<'graph, G: ?Sized> {
    /// The graph to be analyzed
    graph: &'graph G,

}

impl<'graph, G> ScalarSimilarity<G::NodeId, G::NodeId> for ResnikResult<'graph, G> where G: MonoplexMonopartiteGraph {
    type Similarity = f64;
    fn similarity(&self, left: &G::NodeId, right: &G::NodeId) -> Self::Similarity {
        1.0
    }   
}

/// Trait providing the `Resnik` method 
/// TODO: Finish
pub trait Resnik {
    /// The method for applying the Resnik
    /// 
    /// # Errors 
    /// - If the graph is not a dag
    fn resnik(&self) -> Result<ResnikResult<'_, Self>, ResnikError> {
        
        Ok(ResnikResult { graph: self })
    }


}

impl <G>Resnik for G {
    
}