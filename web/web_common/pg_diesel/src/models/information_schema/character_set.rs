//! Struct representing a row in the `information_schema.character_sets` table.

use diesel::{Queryable, QueryableByName, Selectable};

/// Struct defining the `information_schema.character_sets` table.
///
/// The `character_sets` view contains one row for each character set available
/// in the current database. This includes information about character sets that
/// can be used for character data types, providing details about their catalog,
/// schema, repertoire, form of use, and default collation settings.
///
/// Character sets define the encoding and representation of text data, and this
/// view is essential for understanding what character encodings are available
/// and their associated properties within the database system.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::character_sets::character_sets)]
pub struct CharacterSet {
    /// Name of the database containing the character set (always the current
    /// database). For PostgreSQL, this is typically the name of the current
    /// database.
    pub character_set_catalog: Option<String>,
    /// Name of the schema containing the character set.
    /// This indicates the schema where the character set is defined.
    pub character_set_schema: Option<String>,
    /// Name of the character set.
    /// This is the identifier for the specific character encoding (e.g.,
    /// "UTF8", "SQL_ASCII").
    pub character_set_name: Option<String>,
    /// Identifies the character repertoire or the method used to define the
    /// character set. Typically describes the Unicode standard or character
    /// encoding method used. Common values include "UCS" (Universal
    /// Character Set) or specific encoding descriptions.
    pub character_repertoire: Option<String>,
    /// Form of use for the character set; describes how the character set is
    /// organized or presented. For Unicode-based character sets, this is
    /// often "UCS" (Unicode Character Set). This field provides additional
    /// context about the character set's structure.
    pub form_of_use: Option<String>,
    /// Catalog name of the default collation for this character set; `NULL` if
    /// not applicable. Identifies the catalog containing the default
    /// collation definition.
    pub default_collate_catalog: Option<String>,
    /// Schema name of the default collation for this character set; `NULL` if
    /// not applicable. Identifies the schema containing the default
    /// collation definition.
    pub default_collate_schema: Option<String>,
    /// Name of the default collation for this character set; `NULL` if not
    /// applicable. The default collation defines how text data using this
    /// character set is sorted and compared.
    pub default_collate_name: Option<String>,
}
