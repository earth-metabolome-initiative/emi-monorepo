//! A trait defining a `Read` table entry.

use diesel::{
    Identifiable, QueryDsl, RunQueryDsl,
    query_dsl::methods::{FindDsl, LimitDsl, LoadQuery},
};
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
    fn read(primary_key: Self::Id, conn: &mut C) -> Result<Self, diesel::result::Error>;
}

impl<C, T> Read<C> for T
where
    T: Identifiable,
    T::Table: FindDsl<<T as Identifiable>::Id>,
    <T::Table as FindDsl<<T as Identifiable>::Id>>::Output: LimitDsl + RunQueryDsl<C>,
    <<T::Table as FindDsl<<T as Identifiable>::Id>>::Output as LimitDsl>::Output:
        for<'a> LoadQuery<'a, C, T>,
{
    fn read(primary_key: Self::Id, conn: &mut C) -> Result<Self, diesel::result::Error> {
        RunQueryDsl::first(QueryDsl::find(T::table(), primary_key), conn)
    }
}
