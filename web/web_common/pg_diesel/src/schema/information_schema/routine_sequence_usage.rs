//! Submodule for the `information_schema.routine_sequence_usage` view schema.

diesel::table! {
    /// `information_schema.routine_sequence_usage` — view containing one row for each
    /// sequence used by a routine in the current database.
    information_schema.routine_sequence_usage (specific_catalog, specific_schema, specific_name, routine_catalog, routine_schema, routine_name, sequence_catalog, sequence_schema, sequence_name) {
        /// Catalog (database) containing the specific routine.
        specific_catalog -> Nullable<Text>,
        /// Schema containing the specific routine.
        specific_schema -> Nullable<Text>,
        /// Specific name of the routine.
        specific_name -> Nullable<Text>,
        /// Catalog (database) containing the routine.
        routine_catalog -> Nullable<Text>,
        /// Schema containing the routine.
        routine_schema -> Nullable<Text>,
        /// Name of the routine.
        routine_name -> Nullable<Text>,
        /// Catalog (database) containing the sequence.
        sequence_catalog -> Nullable<Text>,
        /// Schema containing the sequence.
        sequence_schema -> Nullable<Text>,
        /// Name of the sequence.
        sequence_name -> Nullable<Text>,
    }
}
