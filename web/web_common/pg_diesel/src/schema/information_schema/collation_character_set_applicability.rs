//! Submodule for the `information_schema.collation_character_set_applicability`
//! view schema.

diesel::table! {
    /// `information_schema.collation_character_set_applicability` — view containing one row for each
    /// collation that is applicable to a character set. This view establishes the relationship
    /// between collations and the character sets they can be used with, providing essential
    /// information for understanding which collations are valid for specific character encodings.
    information_schema.collation_character_set_applicability (collation_catalog, collation_schema, collation_name, character_set_catalog, character_set_schema, character_set_name) {
        /// Catalog (database) containing the collation.
        collation_catalog -> Nullable<Text>,
        /// Schema containing the collation.
        collation_schema -> Nullable<Text>,
        /// Name of the collation.
        collation_name -> Nullable<Text>,
        /// Catalog (database) containing the character set that this collation applies to.
        character_set_catalog -> Nullable<Text>,
        /// Schema containing the character set that this collation applies to.
        character_set_schema -> Nullable<Text>,
        /// Name of the character set that this collation applies to.
        character_set_name -> Nullable<Text>,
    }
}
