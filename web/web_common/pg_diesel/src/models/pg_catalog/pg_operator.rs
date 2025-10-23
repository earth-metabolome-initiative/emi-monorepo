use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

use super::{PgExtension, PgProc, PgType};

mod cached_queries;

/// Represents the `pg_operator` system catalog table in `PostgreSQL`.
/// This table stores information about operators.
#[derive(Queryable, QueryableByName, Selectable, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_operator::pg_operator)]
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
    pub fn function(&self, conn: &mut PgConnection) -> Result<PgProc, diesel::result::Error> {
        cached_queries::function(self, conn)
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
    pub fn extension(&self, conn: &mut PgConnection) -> Result<PgExtension, diesel::result::Error> {
        cached_queries::extension(self, conn)
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
    pub fn left_operand_type(
        &self,
        conn: &mut PgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        cached_queries::left_operand_type(self, conn)
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
    pub fn right_operand_type(
        &self,
        conn: &mut PgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        cached_queries::right_operand_type(self, conn)
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
    pub fn result_type(&self, conn: &mut PgConnection) -> Result<PgType, diesel::result::Error> {
        cached_queries::result_type(self, conn)
    }
}
