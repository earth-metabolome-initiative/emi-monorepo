//! Submodule providing the diesel structs and method relative to postgres
//! triggers.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a `PostgreSQL` trigger, as defined in the `pg_trigger` system
/// catalog table. This struct corresponds to the internal representation of
/// triggers within `PostgreSQL`.
///
/// Triggers in `PostgreSQL` are special procedures that are automatically
/// executed in response to certain events on a particular table or view.
///
/// This struct provides metadata about triggers, including their function,
/// enabling state, constraints, and additional attributes.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Hash, Clone)]
#[diesel(table_name = crate::schema::pg_trigger)]
pub struct PgTrigger {
    /// The object identifier (OID) of the trigger.
    pub oid: u32,
    /// The OID of the relation (table or view) that the trigger is associated
    /// with.
    pub tgrelid: u32,
    /// The OID of the parent trigger, if this trigger is inherited; otherwise,
    /// zero.
    pub tgparentid: u32,
    /// The name of the trigger.
    pub tgname: String,
    /// The OID of the function that is executed when the trigger fires.
    pub tgfoid: u32,
    /// An integer bitmask representing trigger firing conditions (BEFORE,
    /// AFTER, INSTEAD OF, etc.).
    pub tgtype: i16,
    /// A string indicating whether the trigger is enabled ('O' for enabled, 'D'
    /// for disabled, etc.).
    pub tgenabled: String,
    /// A boolean indicating whether the trigger is internally created by
    /// PostgreSQL (`true`) or user-defined (`false`).
    pub tgisinternal: bool,
    /// The OID of the related table for a constraint trigger, or zero if not
    /// applicable.
    pub tgconstrrelid: u32,
    /// The OID of the index used for a constraint trigger, or zero if not
    /// applicable.
    pub tgconstrindid: u32,
    /// The OID of the associated constraint, or zero if not applicable.
    pub tgconstraint: u32,
    /// A boolean indicating whether the trigger is deferrable.
    pub tgdeferrable: bool,
    /// A boolean indicating whether the trigger is initially deferred.
    pub tginitdeferred: bool,
    /// The number of argument bytes passed to the trigger function.
    pub tgnargs: i16,
    /// A vector of attribute (column) numbers on which the trigger acts, if
    /// applicable.
    pub tgattr: Vec<i16>,
    /// A vector of argument bytes supplied to the trigger function.
    pub tgargs: Vec<u8>,
    /// The name of the OLD transition table, if applicable; otherwise, `None`.
    pub tgoldtable: Option<String>,
    /// The name of the NEW transition table, if applicable; otherwise, `None`.
    pub tgnewtable: Option<String>,
}
