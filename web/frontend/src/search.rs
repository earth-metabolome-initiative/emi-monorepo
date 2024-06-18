//! Implements offline variant of search functionality.

use web_common::database::{Filtrable, SimilarityScore, Tabular};

const BATCH_SIZE: i64 = 100;

pub(crate) fn search<C, T>(
    query: &str,
    filter: Option<&T::Filter>,
    limit: Option<i64>,
    offset: Option<i64>,
    connection: &mut gluesql::prelude::Glue<C>,
) -> Result<Vec<T>, web_common::api::ApiError>
where
    C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    T: Filtrable + Tabular + SimilarityScore,
{
    // We always consider the query as lowercase.
    let query = query.to_lowercase();

    // In order to return the struct as a sorted list, we use a min-heap.
    let mut heap = std::collections::BinaryHeap::new();

    // In order to avoid potentially memory peak, we batch the search results and
    // load them in chunks of BATCH_SIZE records.
    let mut number_of_batches = 0;
    loop {
        let records = T::all(filter, limit, offset, connection)?;
    }
    // We use the SimilarityScore trait to calculate the similarity score.
}
