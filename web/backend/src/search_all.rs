use crate::models::*;
use crate::nested_models::*;
use strsim::normalized_damerau_levenshtein;
use web_common::database::search_tables::SearchableStruct;

/// Converts into a vector of SearchableStructs.
fn convert<B, F>(query: &str, backends: Vec<(B, f32)>) -> Vec<(SearchableStruct, f32)>
where
    F: From<B> + web_common::database::Tabular,
    SearchableStruct: From<F>,
{
    // We compute the normalized Damerau-Levenshtein similarity between the query and the name of the
    // table associated to the backend.
    let table_name = F::TABLE.as_ref();
    let similarity = normalized_damerau_levenshtein(query, &table_name) as f32;

    backends
        .into_iter()
        .map(|(backend, distance)| {
            (
                SearchableStruct::from(F::from(backend)),
                (1.0 + similarity) / (1.0 + distance),
            )
        })
        .collect()
}

/// Returns a list of all records that match the query.
///
/// # Arguments
/// * `author_user_id` - The ID of the author of the current request, if online.
/// * `query` - The search query.
/// * `limit` - The maximum number of records to return. By default, this is 10.
pub(crate) fn search_all(
    author_user_id: Option<i32>,
    query: &str,
    limit: Option<i64>,
    connection: &mut diesel::r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::PgConnection>,
    >,
) -> Result<Vec<SearchableStruct>, web_common::api::ApiError> {
    let mut results: Vec<(SearchableStruct, f32)> = Vec::new();

    if query.is_empty() {
        return Ok(Vec::new());
    }

    results.extend(convert::<
        NestedBioOttRank,
        web_common::database::NestedBioOttRank,
    >(
        query,
        NestedBioOttRank::strict_word_similarity_search_with_score_viewable(
            query, limit, None, connection,
        )?,
    ));

    results.extend(convert::<User, web_common::database::User>(
        query,
        User::strict_word_similarity_search_with_score_viewable(query, limit, None, connection)?,
    ));

    results.extend(
        convert::<NestedProject, web_common::database::NestedProject>(
            query,
            NestedProject::strict_word_similarity_search_with_score_viewable(
                author_user_id,
                query,
                limit,
                None,
                connection,
            )?,
        ),
    );

    results.extend(convert::<
        NestedObservation,
        web_common::database::NestedObservation,
    >(
        query,
        NestedObservation::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            limit,
            None,
            connection,
        )?,
    ));

    results.extend(convert::<
        NestedOrganism,
        web_common::database::NestedOrganism,
    >(
        query,
        NestedOrganism::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            limit,
            None,
            connection,
        )?,
    ));

    results.extend(convert::<NestedSample, web_common::database::NestedSample>(
        query,
        NestedSample::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            limit,
            None,
            connection,
        )?,
    ));

    results.extend(convert::<
        NestedBioOttTaxonItem,
        web_common::database::NestedBioOttTaxonItem,
    >(
        query,
        NestedBioOttTaxonItem::strict_word_similarity_search_with_score_viewable(
            query, limit, None, connection,
        )?,
    ));

    results.extend(convert::<
        NestedNameplate,
        web_common::database::NestedNameplate,
    >(
        query,
        NestedNameplate::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            limit,
            None,
            connection,
        )?,
    ));

    results.extend(convert::<
        NestedSampleContainer,
        web_common::database::NestedSampleContainer,
    >(
        query,
        NestedSampleContainer::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            limit,
            None,
            connection,
        )?,
    ));

    results.extend(convert::<NestedTeam, web_common::database::NestedTeam>(
        query,
        NestedTeam::strict_word_similarity_search_with_score_viewable(
            query, limit, None, connection,
        )?,
    ));

    // Finally, we sort the results by relevance.
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    Ok(results
        .into_iter()
        .take(limit.unwrap_or(10) as usize)
        .map(|(result, _)| result)
        .collect())
}
