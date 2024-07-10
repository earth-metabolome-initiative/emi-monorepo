use crate::database::*;
use diesel::RunQueryDsl;
use web_common::api::ApiError;
use web_common::database::convert_search;
use web_common::database::search_tables::SearchableStruct;

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
    threshold: f32,
    connection: &mut diesel::r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::PgConnection>,
    >,
) -> Result<(), web_common::api::ApiError> {
    diesel::sql_query(format!(
        "SET pg_trgm.strict_word_similarity_threshold = {}",
        threshold
    ))
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

    results.extend(
        convert_search::<NestedUser, web_common::database::NestedUser>(
            query,
            NestedUser::strict_word_similarity_search_with_score_viewable(
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
        NestedProject,
        web_common::database::NestedProject,
    >(
        query,
        NestedProject::strict_word_similarity_search_with_score_viewable(
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
        NestedSample,
        web_common::database::NestedSample,
    >(
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(
        convert_search::<NestedTeam, web_common::database::NestedTeam>(
            query,
            NestedTeam::strict_word_similarity_search_with_score_viewable(
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
        NestedRank,
        web_common::database::NestedRank,
    >(
        query,
        NestedRank::strict_word_similarity_search_with_score_viewable(
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
        set_threshold(results.last().unwrap().1, connection)?;
    }

    results.extend(convert_search::<
        NestedTaxon,
        web_common::database::NestedTaxon,
    >(
        query,
        NestedTaxon::strict_word_similarity_search_with_score_viewable(
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
