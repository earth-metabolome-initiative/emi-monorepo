use crate::nested_models::*;
use diesel::RunQueryDsl;
use strsim::normalized_damerau_levenshtein;
use web_common::api::ApiError;
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

/// Set the strict word similarity threshold for the search.
///
/// # Arguments
/// * `threshold` - The new threshold.
/// * `connection` - The database connection.
///
/// # Implementative details
/// The threshold is stored in the `pg_trgm.strict_word_similarity_threshold` configuration
/// parameter. By default, this is 0.5.
fn set_threshold(
    threshold: f64,
    connection: &mut diesel::r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::PgConnection>,
    >,
) -> Result<(), web_common::api::ApiError> {
    diesel::sql_query("SET pg_trgm.strict_word_similarity_threshold = ?")
        .bind::<diesel::sql_types::Float8, _>(threshold)
        .execute(connection)
        .map_err(ApiError::from)?;
    Ok(())
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

    let limit = limit.unwrap_or(10);

    results.extend(convert::<NestedUser, web_common::database::NestedUser>(
        query,
        NestedUser::strict_word_similarity_search_with_score_viewable(
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(
        convert::<NestedProject, web_common::database::NestedProject>(
            query,
            NestedProject::strict_word_similarity_search_with_score_viewable(
                author_user_id,
                query,
                Some(limit),
                None,
                connection,
            )?,
        ),
    );

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<
        NestedObservation,
        web_common::database::NestedObservation,
    >(
        query,
        NestedObservation::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<
        NestedOrganism,
        web_common::database::NestedOrganism,
    >(
        query,
        NestedOrganism::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<NestedSample, web_common::database::NestedSample>(
        query,
        NestedSample::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<
        NestedNameplate,
        web_common::database::NestedNameplate,
    >(
        query,
        NestedNameplate::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<
        NestedSampleContainer,
        web_common::database::NestedSampleContainer,
    >(
        query,
        NestedSampleContainer::strict_word_similarity_search_with_score_viewable(
            author_user_id,
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<NestedTeam, web_common::database::NestedTeam>(
        query,
        NestedTeam::strict_word_similarity_search_with_score_viewable(
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<
        NestedBioOttRank,
        web_common::database::NestedBioOttRank,
    >(
        query,
        NestedBioOttRank::strict_word_similarity_search_with_score_viewable(
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    if results.len() as i64 >= limit {
        // We sort the results by relevance and take the last one to set the threshold.
        results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        // We only take the first `limit` results.
        results = results.into_iter().take(limit as usize).collect();
        // And we increase the threshold to the last result, so that the subsequent queries are
        // faster and stricter.
        set_threshold(results.last().unwrap().1 as f64, connection)?;
    }

    results.extend(convert::<
        NestedBioOttTaxonItem,
        web_common::database::NestedBioOttTaxonItem,
    >(
        query,
        NestedBioOttTaxonItem::strict_word_similarity_search_with_score_viewable(
            query,
            Some(limit),
            None,
            connection,
        )?,
    ));

    // Finally, we sort the results by relevance.
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    Ok(results
        .into_iter()
        .take(limit as usize)
        .map(|(result, _)| result)
        .collect())
}
