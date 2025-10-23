//! Submodule for the `pg_catalog.pg_cursors` view schema.

diesel::table! {
    /// `pg_catalog.pg_cursors` — view showing all currently available cursors.
    /// Each row represents an open cursor in the current session, providing details
    /// about the cursor's name, SQL statement, and properties.
    pg_catalog.pg_cursors (name) {
        /// Name of the cursor.
        name -> Nullable<Text>,
        /// The SQL statement that created the cursor.
        statement -> Nullable<Text>,
        /// `true` if the cursor is holdable (can be used after transaction commit).
        is_holdable -> Nullable<Bool>,
        /// `true` if the cursor is declared as a binary cursor.
        is_binary -> Nullable<Bool>,
        /// `true` if the cursor is scrollable (can move backward).
        is_scrollable -> Nullable<Bool>,
        /// Time at which the cursor was created.
        creation_time -> Nullable<Timestamp>,
    }
}
