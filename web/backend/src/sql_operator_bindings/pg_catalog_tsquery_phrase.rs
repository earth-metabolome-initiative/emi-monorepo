//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PgCatalogTsqueryPhrase, " <-> ", diesel_full_text_search::TsQuery, backend: diesel::pg::Pg);
/// Trait for the `<->` operator.
pub trait HasPgCatalogTsqueryPhrase:
    Sized + diesel::expression::Expression<SqlType = diesel_full_text_search::TsQuery>
{
    /// The function to create the `PgCatalogTsqueryPhrase` struct representing the `<->` operator.
    fn pg_catalog_tsquery_phrase<Rhs>(
        self,
        rhs: Rhs,
    ) -> PgCatalogTsqueryPhrase<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel_full_text_search::TsQuery>,
    {
        PgCatalogTsqueryPhrase::new(self, rhs.as_expression())
    }
}

impl<T> HasPgCatalogTsqueryPhrase for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel_full_text_search::TsQuery>
{
}