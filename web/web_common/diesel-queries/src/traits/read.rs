//! A trait defining a `Read` table entry.

use diesel::{
    Identifiable, QueryDsl, RunQueryDsl,
    associations::HasTable,
    query_dsl::methods::{FindDsl, LimitDsl, LoadQuery},
};
/// The `Read` trait
pub trait Read<C>: Sized
where
    for<'a> &'a Self: Identifiable,
{
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
    fn read(
        primary_key: <&Self as Identifiable>::Id,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error>;
}

impl<C, T> Read<C> for T
where
    for<'a> &'a T: Identifiable,
    for<'a> <&'a T as HasTable>::Table: FindDsl<<&'a T as Identifiable>::Id>,
    for<'a> <<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output: LimitDsl + RunQueryDsl<C>,
    for<'a> <<<&'a T as HasTable>::Table as FindDsl<<&'a T as Identifiable>::Id>>::Output as LimitDsl>::Output:
        LoadQuery<'a, C, T>,
{
    fn read(
        primary_key: <&Self as Identifiable>::Id,
        conn: &mut C,
    ) -> Result<Self, diesel::result::Error> {
        RunQueryDsl::first(QueryDsl::find(<&T as HasTable>::table(), primary_key), conn)
    }
}
