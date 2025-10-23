//! Submodule for the `pg_catalog.pg_locks` table schema.

diesel::table! {
    /// `pg_catalog.pg_locks` — view showing information about locks held by active processes.
    /// This is a system view (not a table), providing real-time information about database locks.
    /// Uses `locktype` as a nominal primary key for Diesel compatibility, though the view
    /// doesn't have a true unique identifier.
    pg_catalog.pg_locks (locktype) {
        /// Type of lockable object (relation, extend, page, tuple, transactionid, virtualxid, object, userlock, advisory).
        locktype -> Nullable<Text>,
        /// OID of the database in which the object exists, or 0 if the object is a shared object.
        database -> Nullable<Oid>,
        /// OID of the relation (table, index, etc.) on which the lock is held, or null if not applicable.
        relation -> Nullable<Oid>,
        /// Page number within the relation, or null if not applicable.
        page -> Nullable<Integer>,
        /// Tuple number within the page, or null if not applicable.
        tuple -> Nullable<SmallInt>,
        /// Virtual transaction ID of the target transaction, or null if not applicable.
        virtualxid -> Nullable<Text>,
        /// ID of the target transaction, or null if not applicable.
        transactionid -> Nullable<Oid>,
        /// OID of the system catalog containing the target object, or null if not applicable.
        classid -> Nullable<Oid>,
        /// OID of the target object within its system catalog, or null if not applicable.
        objid -> Nullable<Oid>,
        /// Sub-object number (e.g., column number for a column), or 0 if not applicable.
        objsubid -> Nullable<SmallInt>,
        /// Virtual transaction ID of the holder or awaiter of this lock.
        virtualtransaction -> Nullable<Text>,
        /// Process ID of the server process holding or awaiting this lock.
        pid -> Nullable<Integer>,
        /// Name of the lock mode held or desired by this process.
        mode -> Nullable<Text>,
        /// `true` if lock is held, `false` if lock is awaited.
        granted -> Nullable<Bool>,
        /// `true` if lock was taken via fast path, `false` if taken via main lock table.
        fastpath -> Nullable<Bool>,
        /// Time when the server process started waiting for this lock, or null if the lock is held.
        waitstart -> Nullable<Timestamp>,
    }
}
