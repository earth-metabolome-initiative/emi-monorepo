//! Implements offline variant of search functionality.

use web_common::database::{
    convert_search, AllRecords, Filtrable, SearchableStruct, SimilarityScore, Tabular,
};

const BATCH_SIZE: i64 = 100;

#[derive(PartialEq)]
struct MinNonNan<T>((T, f32));

impl<T: PartialEq> PartialOrd for MinNonNan<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0 .1.partial_cmp(&self.0 .1)
    }
}

impl<T: PartialEq> Ord for MinNonNan<T> {
    fn cmp(&self, other: &MinNonNan<T>) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: PartialEq> Eq for MinNonNan<T> {}

impl<T: PartialEq> From<(T, f32)> for MinNonNan<T> {
    fn from(value: (T, f32)) -> Self {
        MinNonNan(value)
    }
}

impl<T: PartialEq> AsRef<T> for MinNonNan<T> {
    fn as_ref(&self) -> &T {
        &self.0 .0
    }
}

/// Searches for records in the database.
///
/// # Arguments
/// * `lowercase_query` - The search query.
/// * `filter` - The filter to apply to the search.
/// * `limit` - The maximum number of records to return, defaults to 10.
/// * `offset` - The number of records to skip, defaults to 0.
/// * `connection` - The database connection.
pub(crate) async fn search_with_score<C, T>(
    lowercase_query: &str,
    filter: Option<&T::Filter>,
    limit: Option<i64>,
    offset: Option<i64>,
    connection: &mut gluesql::prelude::Glue<C>,
) -> Result<Vec<(T, f32)>, web_common::api::ApiError>
where
    C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    T: Filtrable + Tabular + SimilarityScore + AllRecords,
{
    let limit = limit.unwrap_or(10);
    let mut offset = offset.unwrap_or(0);

    // In order to return the struct as a sorted list, we use a min-heap.
    let mut heap = std::collections::BinaryHeap::with_capacity((offset + limit) as usize);

    // In order to avoid potentially memory peak, we batch the search results and
    // load them in chunks of BATCH_SIZE records.
    let mut number_of_batches = 0;
    loop {
        let records = T::all_records(
            filter,
            Some(BATCH_SIZE),
            Some(BATCH_SIZE * number_of_batches),
            connection,
        )
        .await?;
        let number_of_records = records.len() as i64;

        for record in records {
            let score = record.similarity_score(&lowercase_query) as f32;
            let similarity: MinNonNan<T> = (record, score).into();

            // If the heap is not full, we push the record, otherwise we compare the similarity score
            // with the smallest element in the heap.
            if heap.len() < limit as usize {
                heap.push(std::cmp::Reverse(similarity));
            } else if heap.peek().unwrap().0 < similarity {
                heap.pop();
                heap.push(std::cmp::Reverse(similarity));
            }
        }

        if number_of_records < BATCH_SIZE {
            break;
        }

        number_of_batches += 1;
    }

    // We extract the records from the heap and return them as a sorted list.
    let mut records = Vec::with_capacity(limit as usize);
    while let Some(std::cmp::Reverse(similarity)) = heap.pop() {
        if offset > 0 {
            offset -= 1;
            continue;
        }
        records.push(similarity.0);
    }

    Ok(records)
}

/// Searches for records in the database.
///
/// # Arguments
/// * `lowercase_query` - The search query.
/// * `filter` - The filter to apply to the search.
/// * `limit` - The maximum number of records to return, defaults to 10.
/// * `offset` - The number of records to skip, defaults to 0.
/// * `connection` - The database connection.
pub(crate) async fn search<C, T>(
    lowercase_query: &str,
    filter: Option<&T::Filter>,
    limit: Option<i64>,
    offset: Option<i64>,
    connection: &mut gluesql::prelude::Glue<C>,
) -> Result<Vec<T>, web_common::api::ApiError>
where
    C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    T: Filtrable + Tabular + SimilarityScore + AllRecords,
{
    Ok(
        search_with_score(lowercase_query, filter, limit, offset, connection)
            .await?
            .into_iter()
            .map(|(record, _)| record)
            .collect(),
    )
}

async fn extend_search_heap<C, T>(
    heap: &mut std::collections::BinaryHeap<std::cmp::Reverse<MinNonNan<SearchableStruct>>>,
    lowercase_query: &str,
    limit: i64,
    offset: i64,
    connection: &mut gluesql::prelude::Glue<C>,
) -> Result<(), web_common::api::ApiError>
where
    C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
    T: Filtrable + Tabular + SimilarityScore + AllRecords,
    SearchableStruct: From<T>,
{
    for entry in convert_search::<T, T>(
        lowercase_query,
        search_with_score::<C, T>(
            &lowercase_query,
            None,
            Some(limit),
            Some(offset),
            connection,
        )
        .await?,
    ) {
        let entry: MinNonNan<SearchableStruct> = entry.into();
        if heap.len() < limit as usize {
            heap.push(std::cmp::Reverse(entry));
        } else if heap.peek().unwrap().0 < entry {
            heap.pop();
            heap.push(std::cmp::Reverse(entry));
        }
    }

    Ok(())
}

/// Macro to apply the extend search heap on the vector of provided vector of struct data types.
macro_rules! extend_search_heap_macro {
    ($heap:ident, $lowercase_query:ident, $limit:ident, $offset:ident, $connection:ident, $($T:ty),*) => {
        $(
            extend_search_heap::<_, $T>($heap, &$lowercase_query, $limit, $offset, $connection).await?;
        )*
    };
}

pub(crate) async fn search_all<C>(
    query: &str,
    limit: Option<i64>,
    offset: Option<i64>,
    connection: &mut gluesql::prelude::Glue<C>,
) -> Result<Vec<SearchableStruct>, web_common::api::ApiError>
where
    C: gluesql::core::store::GStore + gluesql::core::store::GStoreMut,
{
    let mut offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(10);
    let mut heap = std::collections::BinaryHeap::with_capacity((offset + limit) as usize);
    let mutable_heap = &mut heap;
    let lowercase_query = query.to_lowercase();
    let lowercase_query_ref = &lowercase_query;

    extend_search_heap_macro!(
        mutable_heap,
        lowercase_query_ref,
        limit,
        offset,
        connection,
        web_common::database::NestedUser,
        web_common::database::NestedProject,
        web_common::database::NestedObservation,
        web_common::database::NestedOrganism,
        web_common::database::NestedSample,
        web_common::database::NestedNameplate,
        web_common::database::NestedSampleContainer,
        web_common::database::NestedTeam,
        web_common::database::NestedBioOttRank,
        web_common::database::NestedBioOttTaxonItem
    );

    // We extract the records from the heap and return them as a sorted list.
    let mut records = Vec::with_capacity(limit as usize);
    while let Some(std::cmp::Reverse(similarity)) = heap.pop() {
        if offset > 0 {
            offset -= 1;
            continue;
        }
        records.push(similarity.0 .0);
    }

    Ok(records)
}
