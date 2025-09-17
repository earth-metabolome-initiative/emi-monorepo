//! Submodule defining the `Ancestor` trait for struct tables.

use diesel::{
    backend::DieselReserveSpecialization,
    connection::LoadConnection,
    deserialize::FromSqlRow,
    sql_types::{Bool, HasSqlType, Untyped},
};

use crate::prelude::TableName;

#[derive(diesel::QueryableByName, Debug)]
/// Represents a row returned by the SQL query checking for ancestor existence.
pub struct AncestorExists {
    #[diesel(sql_type = Bool)]
    /// Indicates whether the ancestor exists in the lineage of the descendant.
    exists: bool,
}

/// Trait defining the methods for managing ancestor relationships in struct
/// tables.
pub trait Ancestor<C>
where
    Self: TableName + Sized,
    C: LoadConnection,
    <C as diesel::Connection>::Backend:
        DieselReserveSpecialization + HasSqlType<<Self as Ancestor<C>>::SqlType> + 'static,
    AncestorExists: FromSqlRow<Untyped, <C as diesel::Connection>::Backend>,
    for<'a> &'a Self: diesel::Identifiable,
    for<'a> <&'a Self as diesel::Identifiable>::Id:
        diesel::serialize::ToSql<<Self as Ancestor<C>>::SqlType, C::Backend>,
{
    /// Name of column representing the parent ID in the table.
    const PARENT_ID: &'static str;
    /// Name of the primary key column in the table.
    const ID: &'static str;
    /// Type of the primary key in the table.
    type SqlType: diesel::query_builder::QueryId;

    /// Returns whether the current struct is an ancestor of the given
    /// descendant.
    ///
    /// # Arguments
    ///
    /// * `other`: The descendant to check against.
    /// * `conn`: The database connection to use for the query.
    ///
    /// # Errors
    ///
    /// * If the query fails, an error is returned.
    fn is_ancestor_of<T>(&self, other: &T, conn: &mut C) -> Result<bool, diesel::result::Error>
    where
        T: Descendant<Self> + ?Sized,
        for<'a> &'a T: diesel::Identifiable<Id = <&'a Self as diesel::Identifiable>::Id>,
    {
        use diesel::{Identifiable, RunQueryDsl};
        let Some(parent) = other.parent() else {
            // If the other does not have a parent ID, it cannot be an descendant.
            return Ok(false);
        };
        let ancestor_id = self.id();
        if ancestor_id == parent {
            // If the ancestor is the same as the parent, it is trivially an ancestor.
            return Ok(true);
        }

        // Otherwise, we need to execute a recursive query to check if the ancestor is
        // in the lineage of the descendant.

        let query = format!(
            "
			WITH RECURSIVE ancestors({primary_key}) AS (
				SELECT {parent} FROM {table}
				WHERE {primary_key} = $1
				UNION ALL
				SELECT {table}.{parent} FROM {table}
				JOIN ancestors ON {table}.{primary_key} = ancestors.{primary_key}
				WHERE {table}.{parent} IS NOT NULL
			)
			SELECT EXISTS (
				SELECT 1 FROM ancestors WHERE {primary_key} = $2
			) as exists
			",
            primary_key = Self::ID,
            parent = Self::PARENT_ID,
            table = Self::TABLE_NAME
        );

        let result: AncestorExists = diesel::sql_query(query)
            .bind::<Self::SqlType, _>(parent)
            .bind::<Self::SqlType, _>(ancestor_id)
            .get_result(conn)?;

        Ok(result.exists)
    }
}

/// Trait defining that a struct table can have descendants.
pub trait Descendant<T>
where
    for<'a> &'a Self: diesel::Identifiable,
{
    /// Returns the ID of the parent of this descendant, if it exists.
    fn parent(&self) -> Option<<&Self as diesel::Identifiable>::Id>;
}
