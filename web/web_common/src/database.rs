//! Submodule providing structs relative to the database.
pub mod selects;
pub use selects::*;
pub mod flat_variants;
pub use flat_variants::*;
pub mod operations;
pub use operations::*;
pub mod notification_message;
pub use notification_message::NotificationMessage;
pub mod nested_variants;
pub use nested_variants::*;
pub mod markers;
pub use markers::*;
pub mod table_names;
pub use table_names::*;
pub mod search_tables;
pub use search_tables::*;
pub mod new_variants;
pub use new_variants::*;
pub mod update_variants;
pub use update_variants::*;
pub mod filter_variants;
pub mod model_impls;
pub use filter_variants::*;

/// Converts into a vector of SearchableStructs.
pub fn convert_search<B, F>(
    query: &str,
    backends: Vec<(B, f32)>,
) -> Vec<(crate::database::SearchableStruct, f32)>
where
    F: From<B> + crate::database::Tabular,
    crate::database::SearchableStruct: From<F>,
{
    // We compute the normalized Damerau-Levenshtein similarity between the query and the name of the
    // table associated to the backend.
    let table_name = F::TABLE.as_ref();
    let similarity = strsim::normalized_damerau_levenshtein(query, &table_name) as f32;

    backends
        .into_iter()
        .map(|(backend, distance)| {
            (
                crate::database::SearchableStruct::from(F::from(backend)),
                (1.0 + similarity) / (1.0 + distance),
            )
        })
        .collect()
}
