#![cfg(feature = "postgres")]
//! Blanked implementations of the `Read` and `BoundedRead` traits for
//! PostgreSQL.

use diesel::{
    Identifiable, OptionalExtension, QueryDsl,
    associations::HasTable,
    query_dsl::methods::{FindDsl, LimitDsl, OffsetDsl},
};
use diesel_async::{AsyncPgConnection, RunQueryDsl, methods::LoadQuery};

use super::{AsyncBoundedRead, AsyncRead};

impl<T> AsyncRead<AsyncPgConnection> for T
where
    T: Identifiable + Send,
    T::Table: FindDsl<<T as Identifiable>::Id>,
    <T::Table as FindDsl<<T as Identifiable>::Id>>::Output:
        LimitDsl + Send + RunQueryDsl<AsyncPgConnection>,
    <<T::Table as FindDsl<<T as Identifiable>::Id>>::Output as LimitDsl>::Output:
        Send + for<'a> LoadQuery<'a, AsyncPgConnection, T>,
{
    async fn read_async(
        primary_key: Self::Id,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        RunQueryDsl::first(QueryDsl::find(T::table(), primary_key), conn).await.optional()
    }
}

impl<T> AsyncBoundedRead<AsyncPgConnection> for T
where
    T: HasTable + Send,
    T::Table: OffsetDsl + Send + RunQueryDsl<AsyncPgConnection>,
    <T::Table as OffsetDsl>::Output: LimitDsl + Send,
    <<T::Table as OffsetDsl>::Output as LimitDsl>::Output:
        Send + for<'a> LoadQuery<'a, AsyncPgConnection, T>,
{
    async fn bounded_read_async(
        offset: u64,
        limit: u64,
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        RunQueryDsl::load(
            LimitDsl::limit(OffsetDsl::offset(T::table(), offset as i64), limit as i64),
            conn,
        )
        .await
    }
}
