#![cfg(feature = "sqlite")]
//! Blanked implementations of the `Read` and `BoundedRead` traits for SQLite.

use diesel::{
    Identifiable, OptionalExtension, QueryDsl, RunQueryDsl, SqliteConnection,
    associations::HasTable,
    query_dsl::methods::{FindDsl, LimitDsl, LoadQuery, OffsetDsl},
};

use super::{BoundedRead, Read};

impl<T> Read<SqliteConnection> for T
where
    T: Identifiable,
    T::Table: FindDsl<<T as Identifiable>::Id>,
    <T::Table as FindDsl<<T as Identifiable>::Id>>::Output:
        LimitDsl + RunQueryDsl<SqliteConnection>,
    <<T::Table as FindDsl<<T as Identifiable>::Id>>::Output as LimitDsl>::Output:
        for<'a> LoadQuery<'a, SqliteConnection, T>,
{
    fn read(
        primary_key: Self::Id,
        conn: &mut SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        RunQueryDsl::first(QueryDsl::find(T::table(), primary_key), conn).optional()
    }
}

impl<T> BoundedRead<SqliteConnection> for T
where
    T: HasTable + Sized,
    T::Table: OffsetDsl + RunQueryDsl<SqliteConnection>,
    <T::Table as OffsetDsl>::Output: LimitDsl,
    <<T::Table as OffsetDsl>::Output as LimitDsl>::Output:
        for<'a> LoadQuery<'a, SqliteConnection, T>,
{
    fn bounded_read(
        offset: u16,
        limit: u16,
        conn: &mut SqliteConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        RunQueryDsl::load(
            LimitDsl::limit(OffsetDsl::offset(T::table(), offset as i64), limit as i64),
            conn,
        )
    }
}
