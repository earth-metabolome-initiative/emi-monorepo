//! A trait defining a `Read` table entry.

pub mod dispatch;
use diesel::{
    Identifiable, OptionalExtension, QueryDsl, RunQueryDsl,
    associations::HasTable,
    query_dsl::methods::{FindDsl, LimitDsl, LoadQuery, OffsetDsl},
};
pub use dispatch::{BoundedReadDispatch, ReadDispatch};
/// The `Read` trait
pub trait Read<C>: Sized + Identifiable {
    /// Loads the row in a table.
    ///
    /// # Arguments
    ///
    /// * `primary_key` - The primary key of the row to be loaded.
    /// * `conn` - A mutable reference to a load connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn read(primary_key: Self::Id, conn: &mut C) -> Result<Option<Self>, diesel::result::Error>;
}

/// The `BoundedRead` trait
pub trait BoundedRead<C>: Sized {
    /// Loads the rows in the provided range.
    ///
    /// # Arguments
    ///
    /// * `offeset` - The offset of the first row to be loaded.
    /// * `limit` - The maximum number of rows to be loaded.
    /// * `conn` - A mutable reference to a load connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if loading the row fails.
    fn bounded_read(
        offset: u16,
        limit: u16,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error>;
}

impl<C, T> Read<C> for T
where
    T: Identifiable,
    T::Table: FindDsl<<T as Identifiable>::Id>,
    <T::Table as FindDsl<<T as Identifiable>::Id>>::Output: LimitDsl + RunQueryDsl<C>,
    <<T::Table as FindDsl<<T as Identifiable>::Id>>::Output as LimitDsl>::Output:
        for<'a> LoadQuery<'a, C, T>,
{
    fn read(primary_key: Self::Id, conn: &mut C) -> Result<Option<Self>, diesel::result::Error> {
        RunQueryDsl::first(QueryDsl::find(T::table(), primary_key), conn).optional()
    }
}

impl<C, T> BoundedRead<C> for T
where
    T: HasTable + Sized,
    T::Table: OffsetDsl + RunQueryDsl<C>,
    <T::Table as OffsetDsl>::Output: LimitDsl,
    <<T::Table as OffsetDsl>::Output as LimitDsl>::Output: for<'a> LoadQuery<'a, C, T>,
{
    fn bounded_read(
        offset: u16,
        limit: u16,
        conn: &mut C,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        RunQueryDsl::load(
            LimitDsl::limit(OffsetDsl::offset(T::table(), i64::from(offset)), i64::from(limit)),
            conn,
        )
    }
}
