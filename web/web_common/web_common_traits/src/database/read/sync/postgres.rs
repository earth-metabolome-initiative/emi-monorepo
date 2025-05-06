#![cfg(feature = "postgres")]
//! Blanked implementations of the `Read` and `BoundedRead` traits for
//! `PostgreSQL`.

use diesel::{
    Identifiable, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
    associations::HasTable,
    query_dsl::methods::{FindDsl, LimitDsl, LoadQuery, OffsetDsl},
};

use super::{BoundedRead, Read};

impl<T> Read<PgConnection> for T
where
    T: Identifiable,
    T::Table: FindDsl<<T as Identifiable>::Id>,
    <T::Table as FindDsl<<T as Identifiable>::Id>>::Output: LimitDsl + RunQueryDsl<PgConnection>,
    <<T::Table as FindDsl<<T as Identifiable>::Id>>::Output as LimitDsl>::Output:
        for<'a> LoadQuery<'a, PgConnection, T>,
{
    fn read(
        primary_key: Self::Id,
        conn: &mut PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        RunQueryDsl::first(QueryDsl::find(T::table(), primary_key), conn).optional()
    }
}

impl<T> BoundedRead<PgConnection> for T
where
    T: HasTable + Sized,
    T::Table: OffsetDsl + RunQueryDsl<PgConnection>,
    <T::Table as OffsetDsl>::Output: LimitDsl,
    <<T::Table as OffsetDsl>::Output as LimitDsl>::Output: for<'a> LoadQuery<'a, PgConnection, T>,
{
    fn bounded_read(
        offset: u16,
        limit: u16,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        RunQueryDsl::load(
            LimitDsl::limit(OffsetDsl::offset(T::table(), i64::from(offset)), i64::from(limit)),
            conn,
        )
    }
}
