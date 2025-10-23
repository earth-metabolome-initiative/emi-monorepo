//! Submodule for the `information_schema.character_sets` view schema.

diesel::table! {
    /// `information_schema.character_sets` — view containing one row for each character set
    /// available in the current database. Provides information about character sets
    /// that can be used for character data types, including their catalog, schema,
    /// repertoire, form of use, and default collation.
    information_schema.character_sets (character_set_catalog, character_set_schema, character_set_name) {
        /// Name of the database containing the character set (always the current database).
        character_set_catalog -> Nullable<Text>,
        /// Name of the schema containing the character set.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set.
        character_set_name -> Nullable<Text>,
        /// Identifies the character repertoire or the method used to define the character set;
        /// typically describes the Unicode standard or character encoding method.
        character_repertoire -> Nullable<Text>,
        /// Form of use for the character set; describes how the character set is organized
        /// or presented (e.g., "UCS" for Unicode Character Set).
        form_of_use -> Nullable<Text>,
        /// Catalog name of the default collation for this character set; `NULL` if not applicable.
        default_collate_catalog -> Nullable<Text>,
        /// Schema name of the default collation for this character set; `NULL` if not applicable.
        default_collate_schema -> Nullable<Text>,
        /// Name of the default collation for this character set; `NULL` if not applicable.
        default_collate_name -> Nullable<Text>,
    }
}
