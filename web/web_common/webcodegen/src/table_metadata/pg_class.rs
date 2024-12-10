//! Submodule providing the struct `PgClass` and associated methods.

use diesel::{Queryable, QueryableByName, Selectable};

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::pg_class)]
pub struct PGClass {
    pub oid: i32,
    pub relname: String,
    pub relnamespace: i32,
    pub reltype: i32,
    pub reloftype: i32,
    pub relowner: i32,
    pub relam: i32,
    pub relfilenode: i32,
    pub reltablespace: i32,
    pub relpages: i32,
    pub reltuples: f32,
    pub relallvisible: i32,
    pub reltoastrelid: i32,
    pub relhasindex: bool,
    pub relisshared: bool,
    pub relpersistence: char,
    pub relkind: char,
    pub relnatts: i16,
    pub relchecks: i16,
    pub relhasrules: bool,
    pub relhastriggers: bool,
    pub relhassubclass: bool,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relispopulated: bool,
    pub relreplident: char,
    pub relispartition: bool,
    pub relrewrite: i32,
}
