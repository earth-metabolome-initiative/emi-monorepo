//! Submodule for the `pg_catalog.pg_statistic` table schema.

diesel::table! {
    /// `pg_catalog.pg_statistic` — table storing planner statistics.
    /// Each row represents statistics for a single ordinary relation column (or combination of columns).
    /// Uses composite primary key (starelid, staattnum, stainherit).
    pg_catalog.pg_statistic (starelid, staattnum, stainherit) {
        /// The table or index that the described column belongs to.
        starelid -> Oid,
        /// The number of the described column.
        staattnum -> SmallInt,
        /// If true, the stats include values only from child tables, not the named table.
        stainherit -> Bool,
        /// The fraction of the column's entries that are null.
        stanullfrac -> Float,
        /// The average stored width, in bytes, of nonnull entries.
        stawidth -> Integer,
        /// The number of distinct nonnull data values in the column.
        stadistinct -> Float,
        /// A code number indicating the kind of statistics stored in the first slot.
        stakind1 -> SmallInt,
        /// A code number indicating the kind of statistics stored in the second slot.
        stakind2 -> SmallInt,
        /// A code number indicating the kind of statistics stored in the third slot.
        stakind3 -> SmallInt,
        /// A code number indicating the kind of statistics stored in the fourth slot.
        stakind4 -> SmallInt,
        /// A code number indicating the kind of statistics stored in the fifth slot.
        stakind5 -> SmallInt,
        /// An operator used to derive the statistics stored in the first slot.
        staop1 -> Oid,
        /// An operator used to derive the statistics stored in the second slot.
        staop2 -> Oid,
        /// An operator used to derive the statistics stored in the third slot.
        staop3 -> Oid,
        /// An operator used to derive the statistics stored in the fourth slot.
        staop4 -> Oid,
        /// An operator used to derive the statistics stored in the fifth slot.
        staop5 -> Oid,
        /// A collation used to derive the statistics stored in the first slot.
        stacoll1 -> Oid,
        /// A collation used to derive the statistics stored in the second slot.
        stacoll2 -> Oid,
        /// A collation used to derive the statistics stored in the third slot.
        stacoll3 -> Oid,
        /// A collation used to derive the statistics stored in the fourth slot.
        stacoll4 -> Oid,
        /// A collation used to derive the statistics stored in the fifth slot.
        stacoll5 -> Oid,
        /// Numerical statistics of the appropriate kind for the first slot, or null if the slot kind does not involve numerical values.
        stanumbers1 -> Nullable<Array<Float>>,
        /// Numerical statistics of the appropriate kind for the second slot, or null if the slot kind does not involve numerical values.
        stanumbers2 -> Nullable<Array<Float>>,
        /// Numerical statistics of the appropriate kind for the third slot, or null if the slot kind does not involve numerical values.
        stanumbers3 -> Nullable<Array<Float>>,
        /// Numerical statistics of the appropriate kind for the fourth slot, or null if the slot kind does not involve numerical values.
        stanumbers4 -> Nullable<Array<Float>>,
        /// Numerical statistics of the appropriate kind for the fifth slot, or null if the slot kind does not involve numerical values.
        stanumbers5 -> Nullable<Array<Float>>,
    }
}
