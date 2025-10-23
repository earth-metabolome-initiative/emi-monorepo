//! Submodule providing the `PgSubscription` struct representing a row of the
//! `pg_subscription` table in `PostgreSQL`.

use diesel::{Identifiable, Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_subscription` table.
///
/// The `pg_subscription` table contains information about logical replication
/// subscriptions.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-subscription.html).
#[derive(
    Identifiable, Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_subscription::pg_subscription)]
#[diesel(primary_key(oid))]
pub struct PgSubscription {
    /// Object identifier.
    pub oid: u32,
    /// Database OID.
    pub subdbid: u32,
    /// Skip LSN.
    pub subskiplsn: u64,
    /// Subscription name.
    pub subname: String,
    /// Owner OID.
    pub subowner: u32,
    /// Enabled flag.
    pub subenabled: bool,
    /// Binary format flag.
    pub subbinary: bool,
    /// Stream mode.
    pub substream: String,
    /// Two-phase state.
    pub subtwophasestate: String,
    /// Disable on error flag.
    pub subdisableonerr: bool,
    /// Password required flag.
    pub subpasswordrequired: bool,
    /// Run as owner flag.
    pub subrunasowner: bool,
    /// Failover flag.
    pub subfailover: bool,
    /// Connection info.
    pub subconninfo: String,
    /// Slot name.
    pub subslotname: Option<String>,
    /// Sync commit setting.
    pub subsynccommit: String,
    /// Publications.
    pub subpublications: Vec<String>,
    /// Origin name.
    pub suborigin: Option<String>,
}
