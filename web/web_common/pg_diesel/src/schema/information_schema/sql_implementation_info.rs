//! `sql_implementation_info` view from `information_schema`.

diesel::table! {
    /// `information_schema.sql_implementation_info` — view containing information
    /// about implementation-specific characteristics of the SQL processor.
    /// Provides details about database implementation limits, defaults, and behaviors.
    information_schema.sql_implementation_info (implementation_info_id) {
        /// Identifier of the implementation information item.
        implementation_info_id -> Nullable<Text>,
        /// Descriptive name of the implementation information item.
        implementation_info_name -> Nullable<Text>,
        /// Integer value for the implementation characteristic (if applicable).
        integer_value -> Nullable<Integer>,
        /// Character value for the implementation characteristic (if applicable).
        character_value -> Nullable<Text>,
        /// Additional comments about the implementation characteristic.
        comments -> Nullable<Text>,
    }
}
