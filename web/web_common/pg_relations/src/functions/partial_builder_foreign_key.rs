use diesel::PgConnection;
use pg_diesel::models::{Column, KeyColumnUsage};

use crate::traits::Extension;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The kind of partial builder constraint
pub enum PartialBuilderKind {
    /// The partial builder constraint is discretionary, i.e. the user
    /// may provide either the primary key or the builder of the associated
    /// table when creating a new instance of the host table.
    Discretional,
    /// The partial builder constraint is mandatory, i.e. the user must
    /// use a partial builder of the associated table when creating a new
    /// instance of the host table.
    Mandatory,
}

/// Returns whether this key column usage is a potential partial builder
/// same-as constraint.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `PgConnection`
///
/// # Implementation details
///
/// A potential partial builder constraint is a constraint that
/// differs from a partial builder constraint in that the host
/// table does not have the same-as constraint which closes the triangular
/// relationship, and therefore the associated table may require the primary
/// key of an ancestor of the host table to be built, but it also may not.
/// Such distintion is modelled by requiring these structs to have as type
/// parameter `IdOrBuilder<PK, B>`, where `PK` is the primary key type of
/// the associated table, and `B` is the builder type of the associated
/// table. This way, the user of the API can choose to provide either
/// the primary key or the builder type when creating a new instance of
/// the associated table.
///
/// # Errors
///
/// * If an error occurs while querying the database
pub(crate) fn is_partial_builder_constraint(
    foreign_key: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Option<(PartialBuilderKind, KeyColumnUsage)>, diesel::result::Error> {
    // A partial builder constraint must be a foreign primary key.
    // and if the foreign key is self-referential, we do not consider it
    // a partial builder constraint.
    if foreign_key.is_self_referential(conn)? || !foreign_key.is_foreign_primary_key(conn)? {
        return Ok(None);
    }

    let foreign_table = foreign_key.foreign_table(conn)?;
    let host_table = foreign_key.local_table(conn)?;
    // If the source table is an extension of the foreign table,
    // we do not consider it a partial builder constraint.
    if host_table.is_extension_of(&foreign_table, conn)?
        || host_table.shares_ancestors_with(&foreign_table, conn)?
    {
        return Ok(None);
    }

    // At this point, we need to identify foreign keys in the
    // foreign table which point to ancestors of the current table.
    let mut keys_to_local_ancestors = Vec::new();
    let primary_key_columns = host_table.primary_key_columns(conn)?;
    for foreign_key in foreign_table.foreign_keys(conn)? {
        let foreign_columns = foreign_key.foreign_columns(conn)?;

        if foreign_columns.len() != primary_key_columns.len() {
            continue;
        }

        if foreign_columns.iter().zip(primary_key_columns.iter()).all(
            |(foreign_column, primary_key_column)| {
                primary_key_column.is_ancestrally_same_as(foreign_column, conn).unwrap_or(false)
            },
        ) {
            keys_to_local_ancestors.push(foreign_key.clone());
        }
    }

    if keys_to_local_ancestors.is_empty() {
        return Ok(None);
    }

    // While it is conceivable to define partial builders on the foreign keys
    // and not on the columns themselves, at this time we are proceeding solely
    // with a column-based approach. Hence, we only support single-column foreign
    // keys.
    if foreign_table.has_composite_primary_key(conn)? {
        unimplemented!(
            "Partial builders from table {host_table} to table {foreign_table} on composite foreign keys are not supported yet"
        );
    }

    // As described in the method documentation, what distinguishes
    // a potential partial builder constraint from a partial builder
    // constraint is the absence of same-as constraints in the host table
    // pointing to foreign keys in the foreign table which point to
    // ancestors of the host table.

    // We determine the local columns of the constraints pointing to
    // ancestors of the host table.
    let columns_to_local_ancestors = keys_to_local_ancestors
        .iter()
        .map(|key| key.local_columns(conn))
        .collect::<Result<Vec<Vec<Column>>, diesel::result::Error>>()?;

    let table = foreign_key.local_table(conn)?;
    let local_columns = foreign_key.local_columns(conn)?;
    let foreign_columns = foreign_key.foreign_columns(conn)?;

    // For each foreign key in the host table, we check whether it contains
    // references to the specific ID contained in the local & foreign columns
    // of the current constraint, which implicitly also checks whether the
    // foreign key points to the same foreign table as the current constraint.
    // Next, we check whether the foreign key's foreign columns contain
    // any of the columns pointing to ancestors of the host table described
    // in constraints to ancestors of the host table which we determined above.
    for foreign_key in table.foreign_keys(conn)?.as_ref() {
        // We retrieve the local columns of the foreign key we are checking.
        let fk_local_columns = foreign_key.columns(conn)?;
        // If all of the columns involved in the current constraint are
        // present in the local columns of the foreign key, we proceed
        // to check the foreign columns.
        if !local_columns.iter().all(|c| fk_local_columns.contains(c)) {
            continue;
        }

        let fk_foreign_columns = foreign_key.foreign_columns(conn)?;
        // All of the foreign columns of the current constraint must
        // be present in the foreign columns of the foreign key.
        if !foreign_columns.iter().all(|c| fk_foreign_columns.contains(c)) {
            continue;
        }

        // Now that we have established that the foreign key involves
        // all of the local & foreign columns of the current constraint,
        // we need to find at least one case where the foreign key's
        // foreign columns contain all of the columns in a `columns_to_local_ancestors`.
        for columns_to_local_ancestor in &columns_to_local_ancestors {
            if columns_to_local_ancestor.iter().all(|c| fk_foreign_columns.contains(c)) {
                return Ok(Some((PartialBuilderKind::Mandatory, foreign_key.clone())));
            }
        }
    }

    // In the case of a discretional partial builder constraint, we also need
    // to check that there exist only one `keys_to_local_ancestors`, otherwise
    // it would not be clear which of these columns to set.
    assert_eq!(
        keys_to_local_ancestors.len(),
        1,
        "Discretional partial builder constraints must have exactly one key to a local ancestor."
    );

    Ok(Some((PartialBuilderKind::Discretional, keys_to_local_ancestors[0].clone())))
}
