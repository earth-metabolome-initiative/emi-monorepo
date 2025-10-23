//! Submodule providing the `PgSubscriptionRel` struct representing a row of the
//! `pg_subscription_rel` table in `PostgreSQL`.

use diesel::{Identifiable, Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_subscription_rel` table.
///
/// The `pg_subscription_rel` table contains the state for each replicated
/// relation in each subscription.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-subscription-rel.html).
#[derive(
    Identifiable, Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_subscription_rel::pg_subscription_rel)]
#[diesel(primary_key(srrelid, srsubid))]
pub struct PgSubscriptionRel {
    /// Subscription OID.
    pub srsubid: u32,
    /// Relation OID.
    pub srrelid: u32,
    /// Subscription state.
    pub srsubstate: String,
    /// Subscription LSN.
    pub srsublsn: Option<u64>,
}
