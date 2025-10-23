//! Schema definitions for `pg_stat_statements` in the `pg_catalog` schema.

diesel::table! {
    /// `pg_stat_statements` — a statistics view provided by the `pg_stat_statements` extension.
    /// Contains one row per distinct normalized query, with execution counts, planning/execution times,
    /// block I/O, and other performance statistics.
    pg_catalog.pg_stat_statements (userid, dbid, queryid) {
        /// OID of the role that executed the statement.
        userid -> diesel::sql_types::Oid,
        /// OID of the database in which the statement was executed.
        dbid -> diesel::sql_types::Oid,
        /// `true` if this entry tracks only top-level statements (not nested ones).
        toplevel -> Bool,
        /// Internal identifier of the query (normalized query hash).
        queryid -> BigInt,
        /// Text of the normalized query (constants replaced with placeholders).
        query -> Text,
        /// Number of times the statement was executed.
        calls -> BigInt,
        /// Number of times the statement was planned.
        plans -> BigInt,
        /// Total time spent planning the statement, in milliseconds.
        total_plan_time -> Double,
        /// Minimum time spent planning, in milliseconds.
        min_plan_time -> Double,
        /// Maximum time spent planning, in milliseconds.
        max_plan_time -> Double,
        /// Mean time spent planning, in milliseconds.
        mean_plan_time -> Double,
        /// Population standard deviation of planning time, in milliseconds.
        stddev_plan_time -> Double,
        /// Total time spent executing the statement, in milliseconds.
        total_exec_time -> Double,
        /// Minimum time spent executing, in milliseconds.
        min_exec_time -> Double,
        /// Maximum time spent executing, in milliseconds.
        max_exec_time -> Double,
        /// Mean time spent executing, in milliseconds.
        mean_exec_time -> Double,
        /// Population standard deviation of execution time, in milliseconds.
        stddev_exec_time -> Double,
        /// Total number of rows retrieved or affected by the statement.
        rows -> BigInt,
        /// Number of shared block cache hits during execution.
        shared_blks_hit -> BigInt,
        /// Number of shared blocks read from disk.
        shared_blks_read -> BigInt,
        /// Number of shared blocks dirtied (modified but not written).
        shared_blks_dirtied -> BigInt,
        /// Number of shared blocks written to disk.
        shared_blks_written -> BigInt,
        /// Number of local block cache hits during execution.
        local_blks_hit -> BigInt,
        /// Number of local blocks read from disk.
        local_blks_read -> BigInt,
        /// Number of local blocks dirtied.
        local_blks_dirtied -> BigInt,
        /// Number of local blocks written to disk.
        local_blks_written -> BigInt,
        /// Number of temporary blocks read from disk.
        temp_blks_read -> BigInt,
        /// Number of temporary blocks written to disk.
        temp_blks_written -> BigInt,
        /// Time spent reading shared blocks, in milliseconds.
        shared_blk_read_time -> Double,
        /// Time spent writing shared blocks, in milliseconds.
        shared_blk_write_time -> Double,
        /// Time spent reading local blocks, in milliseconds.
        local_blk_read_time -> Double,
        /// Time spent writing local blocks, in milliseconds.
        local_blk_write_time -> Double,
    }
}
