//! Submodule providing the struct `PgClass` and associated methods.

use diesel::{Queryable, QueryableByName, Selectable};

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::pg_class)]
#[allow(clippy::struct_excessive_bools)]
pub struct PGClass {
    pub oid: u32,
    pub relname: String,
    pub relnamespace: u32,
    pub reltype: u32,
    pub reloftype: u32,
    pub relowner: u32,
    pub relam: u32,
    pub relfilenode: u32,
    pub reltablespace: u32,
    pub relpages: i32,
    pub reltuples: f32,
    pub relallvisible: i32,
    pub reltoastrelid: u32,
    pub relhasindex: bool,
    pub relisshared: bool,
    pub relpersistence: String,
    pub relkind: String,
    pub relnatts: i16,
    pub relchecks: i16,
    pub relhasrules: bool,
    pub relhastriggers: bool,
    pub relhassubclass: bool,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relispopulated: bool,
    pub relreplident: String,
    pub relispartition: bool,
    pub relrewrite: u32,
}
