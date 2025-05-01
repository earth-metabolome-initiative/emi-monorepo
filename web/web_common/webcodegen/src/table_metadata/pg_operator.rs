use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, Queryable,
    QueryableByName, Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::{PgExtension, PgProc, PgType};

/// Represents the `pg_operator` system catalog table in `PostgreSQL`.
/// This table stores information about operators.
#[derive(Queryable, QueryableByName, Selectable, Clone, Debug, PartialEq, Eq, Hash)]
#[diesel(table_name = crate::schema::pg_operator)]
pub struct PgOperator {
    /// The OID of the operator.
    pub oid: u32,
    /// The name of the operator.
    pub oprname: String,
    /// The OID of the namespace that contains this operator.
    pub oprnamespace: u32,
    /// The OID of the owner of the operator.
    pub oprowner: u32,
    /// The kind of operator ('l' for left unary, 'r' for right unary, 'b' for
    /// binary).
    pub oprkind: String,
    /// True if the operator can be merged.
    pub oprcanmerge: bool,
    /// True if the operator can be hashed.
    pub oprcanhash: bool,
    /// The OID of the left operand type.
    pub oprleft: u32,
    /// The OID of the right operand type.
    pub oprright: u32,
    /// The OID of the result type.
    pub oprresult: u32,
    /// The OID of the commutator operator, or 0 if none.
    pub oprcom: u32,
    /// The OID of the negator operator, or 0 if none.
    pub oprnegate: u32,
    /// The OID of the function implementing the operator.
    pub oprcode: u32,
    /// The OID of the restriction selectivity estimator function, or 0 if none.
    pub oprrest: u32,
    /// The OID of the join selectivity estimator function, or 0 if none.
    pub oprjoin: u32,
}

impl PgOperator {
    /// Returns the rust [`BinOp`](syn::BinOp) corresponding to this operator,
    /// if any.
    ///
    /// # Returns
    ///
    /// * `Some(BinOp)` if the operator is a binary operator
    /// * `None` if the operator is not a binary operator
    pub fn rust_binary_operator(&self) -> Result<syn::BinOp, syn::Error> {
        syn::parse_str(&self.oprname)
    }

    /// Returns whether the current operator has a direct counter-part in Rust.
    pub fn has_rust_operator(&self) -> bool {
        self.rust_binary_operator().is_ok()
    }

    /// Returns the symbol of the operator.
    pub fn symbol(&self) -> &str {
        &self.oprname
    }

    /// Returns the [`PgProc`] associated with this operator.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub async fn function(&self, conn: &mut AsyncPgConnection) -> Result<PgProc, diesel::result::Error> {
        use crate::schema::pg_proc;
        pg_proc::table
            .filter(pg_proc::oid.eq(self.oprcode))
            .select(PgProc::as_select())
            .first::<PgProc>(conn).await
    }

    /// Returns the [`PgExtension`] that contains this function, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a
    ///   [`PgConnection`](diesel::PgConnection).
    ///
    /// # Errors
    ///
    /// * If the function is not contained in an extension
    pub async fn extension(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<PgExtension>, diesel::result::Error> {
        use crate::schema::{pg_depend, pg_extension};
        pg_extension::table
            .inner_join(pg_depend::table.on(pg_extension::oid.eq(pg_depend::refobjid)))
            .filter(pg_depend::objid.eq(self.oid))
            .select(PgExtension::as_select())
            .first::<PgExtension>(conn).await
            .optional()
    }

    /// Returns the left operand type [`PgType`] of the operator.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub async fn left_operand_type(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        use crate::schema::pg_type;
        pg_type::table
            .filter(pg_type::oid.eq(self.oprleft))
            .select(PgType::as_select())
            .first::<PgType>(conn).await
    }

    /// Returns the right operand type [`PgType`] of the operator.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub async fn right_operand_type(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        use crate::schema::pg_type;
        pg_type::table
            .filter(pg_type::oid.eq(self.oprright))
            .select(PgType::as_select())
            .first::<PgType>(conn).await
    }

    /// Returns the result type [`PgType`] of the operator.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub async fn result_type(&self, conn: &mut AsyncPgConnection) -> Result<PgType, diesel::result::Error> {
        use crate::schema::pg_type;
        pg_type::table
            .filter(pg_type::oid.eq(self.oprresult))
            .select(PgType::as_select())
            .first::<PgType>(conn).await
    }
}
