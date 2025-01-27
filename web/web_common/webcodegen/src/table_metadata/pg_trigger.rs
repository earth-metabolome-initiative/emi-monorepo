//! Submodule providing the diesel structs and method relative to postgres triggers.

use diesel::{Queryable, QueryableByName, Selectable};

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Hash, Clone)]
#[diesel(table_name = crate::schema::pg_trigger)]
pub struct PgTrigger {
    pub oid: u32,
    pub tgrelid: u32,
    pub tgparentid: u32,
    pub tgname: String,
    pub tgfoid: u32,
    pub tgtype: i16,
    pub tgenabled: String,
    pub tgisinternal: bool,
    pub tgconstrrelid: u32,
    pub tgconstrindid: u32,
    pub tgconstraint: u32,
    pub tgdeferrable: bool,
    pub tginitdeferred: bool,
    pub tgnargs: i16,
	pub tgattr: Vec<i16>,
    pub tgargs: Vec<u8>,
    pub tgoldtable: Option<String>,
    pub tgnewtable: Option<String>,
}
