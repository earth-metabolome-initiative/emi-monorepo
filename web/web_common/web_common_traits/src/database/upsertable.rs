//! A trait defining a `Upsertable` table entry.

use super::Row;

/// The `Upsertable` trait, which provides a method for upserting records in a
/// SQLite database.
pub trait Upsertable<C: diesel::Connection>: Sized + diesel::associations::HasTable {
    /// The `upsert` method is used to insert a new record or update an existing
    /// one in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// The method returns a `Result` containing an `Option<Self>`, which is the
    /// upserted record, if an update occurred, or `None` if a record was
    /// already identical to one currently in the database.
    ///
    /// # Errors
    ///
    /// * If the upsert operation fails, a `diesel::result::Error` is returned.
    fn upsert(&self, conn: &mut C) -> Result<Option<Self>, diesel::result::Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct UpsertCounts {
    /// The number of rows inserted.
    pub inserted: usize,
    /// The number of rows updated.
    pub updated: usize,
}

impl From<UpsertCounts> for bool {
    fn from(counts: UpsertCounts) -> Self {
        counts.inserted > 0 || counts.updated > 0
    }
}

/// Trait for upserting a vector of records.
pub trait UpsertVec {
    /// The type of the rows in the vector.
    type Row: Row;

    /// Updates the content of the vector with the result of the upsert
    /// operation. This operation assumes that the vector is sorted.
    ///
    ///
    /// # Arguments
    ///
    /// * `sorted_rows` - A sorted iterator over the rows to be upserted.
    ///
    /// # Returns
    ///
    /// The method returns an `UpsertCounts` struct containing the number of
    /// inserted and updated rows.
    ///
    /// # Panics
    ///
    /// * Panics if the vector is not sorted.
    fn upsert_sorted_vec<I>(&mut self, sorted_rows: I) -> UpsertCounts
    where
        I: IntoIterator<Item = Self::Row>;
}

impl<R> UpsertVec for Vec<R>
where
    R: Row,
{
    type Row = R;

    fn upsert_sorted_vec<I>(&mut self, sorted_rows: I) -> UpsertCounts
    where
        I: IntoIterator<Item = Self::Row>,
    {
        debug_assert!(
            self.is_sorted_by(|a, b| a.primary_key() <= b.primary_key()),
            "The vector is not sorted. Please sort the vector before calling upsert_sorted_vec."
        );

        let mut inserted = 0;
        let mut updated = 0;
        // We start by creating a new vector which at least has the same capacity
        // as the current vector.
        let current_vector = core::mem::replace(self, Vec::with_capacity(self.len()));

        // Next, we iterate over the sorted rows and the current vector
        // simultaneously, using two peekable iterators.
        let mut sorted_iter = sorted_rows.into_iter().peekable();
        let mut current_iter = current_vector.into_iter().peekable();

        while let (Some(sorted_row), Some(current_row)) = (sorted_iter.peek(), current_iter.peek())
        {
            if sorted_row.primary_key() == current_row.primary_key() {
                // If the primary keys are equal, and the rows are not equal,
                // we update the current row and insert it into the vector.
                if sorted_row != current_row {
                    updated += 1;
                }
                self.push(sorted_iter.next().unwrap());
                current_iter.next();
            } else if sorted_row.primary_key() < current_row.primary_key() {
                // If the sorted row's primary key is less than the current
                // row's primary key, we insert the sorted row into the vector.
                inserted += 1;
                self.push(sorted_iter.next().unwrap());
            } else {
                // If the sorted row's primary key is greater than the current
                // row's primary key, we insert the current row into the vector.
                self.push(current_iter.next().unwrap());
            }
        }

        debug_assert!(
            self.is_sorted_by(|a, b| a.primary_key() <= b.primary_key()),
            "The vector is not sorted after upsert. Please check the implementation of upsert_sorted_vec."
        );

        UpsertCounts { inserted, updated }
    }
}
