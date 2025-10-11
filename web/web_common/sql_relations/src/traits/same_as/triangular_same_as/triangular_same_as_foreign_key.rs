//! Submodule providing the `TriangularSameAsForeignKeyLike` trait for
//! determining whether a foreign key relationship is a triangular same-as
//! relationship.

use sql_traits::traits::TableLike;

use crate::traits::{
    HorizontalSameAsForeignKeyLike, same_as::horizontal_same_as::HorizontalSameAsTableLike,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Informations regarding the triangular same-as relationship.
pub struct Triangular<'db, FK: HorizontalSameAsForeignKeyLike + ?Sized> {
    /// Horizontal same-as foreign key which forms the base of the triangular
    /// relationship. When the relationship is discretionary, this is `None`.
    horizontal_same_as: Option<&'db FK>,
    /// The hypothenuse foreign key which forms the triangular relationship.
    hypothenuse_same_as: &'db FK,
}

impl<'db, FK: HorizontalSameAsForeignKeyLike + ?Sized> Triangular<'db, FK> {
    /// Returns the horizontal same-as foreign key which forms the base of the
    /// triangular relationship, if any.
    pub fn horizontal_same_as(&self) -> Option<&'db FK> {
        self.horizontal_same_as
    }

    /// Returns the hypothenuse foreign key which forms the triangular
    /// relationship.
    pub fn hypothenuse_same_as(&self) -> &'db FK {
        self.hypothenuse_same_as
    }

    /// Returns whether the triangular same-as relationship is mandatory,
    /// i.e. whether the horizontal same-as foreign key is present.
    pub fn is_mandatory(&self) -> bool {
        self.horizontal_same_as.is_some()
    }
}

/// Trait for foreign keys that can be checked for being triangular same-as
/// relationships.
pub trait TriangularSameAsForeignKeyLike: HorizontalSameAsForeignKeyLike {
    /// Returns whether this foreign key represents a triangular same-as
    /// relationship from the given host table to its referenced table.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables.
    /// * `host_table` - The table containing the foreign key.
    fn is_triangular_same_as<'db>(
        &self,
        database: &'db Self::Database,
        host_table: &'db Self::Table,
    ) -> bool {
        self.triangular_same_as(database, host_table).is_some()
    }

    /// Returns the kind of triangular same-as relationship this foreign key
    /// represents, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables.
    /// * `host_table` - The table containing the foreign key.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE grandparent (id INT PRIMARY KEY);
    /// CREATE TABLE parent (id INT PRIMARY KEY REFERENCES grandparent(id));
    /// CREATE TABLE grandparent_hyphotenuse (
    /// 	id INT PRIMARY KEY,
    /// 	grandparent_id INT REFERENCES grandparent(id),
    /// 	UNIQUE(id, grandparent_id)
    /// );
    /// CREATE TABLE parent_hyphotenuse (
    /// 	id INT PRIMARY KEY,
    /// 	parent_id INT REFERENCES parent(id),
    /// 	UNIQUE(id, parent_id)
    /// );
    /// CREATE TABLE nephew (
    /// 	id INT PRIMARY KEY REFERENCES parent(id),
    ///     mandatory_triangular_grandparent_id INT REFERENCES grandparent_hyphotenuse(id),
    ///     mandatory_triangular_parent_id INT REFERENCES parent_hyphotenuse(id),
    ///     discretionary_triangular_grandparent_id INT REFERENCES grandparent_hyphotenuse(id),
    ///     discretionary_triangular_parent_id INT REFERENCES parent_hyphotenuse(id),
    ///     FOREIGN KEY (mandatory_triangular_grandparent_id, id) REFERENCES grandparent_hyphotenuse(id, grandparent_id),
    ///     FOREIGN KEY (mandatory_triangular_parent_id, id) REFERENCES parent_hyphotenuse(id, parent_id)
    /// );
    /// "#,
    /// )?;
    ///
    /// let nephew = db.table(None, "nephew");
    /// let grandparent_hyphotenuse = db.table(None, "grandparent_hyphotenuse");
    /// let parent_hyphotenuse = db.table(None, "parent_hyphotenuse");
    ///
    /// let [
    ///     extension_primary_key,
    ///     mandatory_triangular_grandparent,
    ///     mandatory_triangular_parent,
    ///     discretionary_triangular_grandparent,
    ///     discretionary_triangular_parent,
    ///     horizontal_grandparent,
    ///     horizontal_parent,
    /// ] = nephew.foreign_keys(&db).collect::<Vec<_>>()[..]
    /// else {
    ///     panic!("Expected exactly 7 foreign keys in nephew table");
    /// };
    ///
    /// assert!(
    ///     extension_primary_key.is_extension_foreign_key(&db, nephew),
    ///     "Expected extension primary key"
    /// );
    ///
    /// let [grandparent_fk] = grandparent_hyphotenuse.foreign_keys(&db).collect::<Vec<_>>()[..] else {
    ///     panic!("Expected exactly 1 foreign key in grandparent_hyphotenuse table");
    /// };
    /// let [parent_fk] = parent_hyphotenuse.foreign_keys(&db).collect::<Vec<_>>()[..] else {
    ///     panic!("Expected exactly 1 foreign key in parent_hyphotenuse table");
    /// };
    ///
    /// assert!(
    ///     !grandparent_fk.is_triangular_same_as(&db, &grandparent_hyphotenuse),
    ///     "Expected {grandparent_fk} to not be triangular same-as"
    /// );
    /// assert!(
    ///     !horizontal_grandparent.is_triangular_same_as(&db, &nephew),
    ///     "Expected {horizontal_grandparent} to not be triangular same-as"
    /// );
    /// assert!(
    ///     horizontal_grandparent.is_horizontal_same_as(&db, &nephew),
    ///     "Expected {horizontal_grandparent} to be horizontal same-as"
    /// );
    /// assert!(
    ///     !parent_fk.is_triangular_same_as(&db, &parent_hyphotenuse),
    ///     "Expected {parent_fk} to not be triangular same-as"
    /// );
    /// assert!(
    ///     !horizontal_parent.is_triangular_same_as(&db, &nephew),
    ///     "Expected {horizontal_parent} to not be triangular same-as"
    /// );
    /// assert!(
    ///     horizontal_parent.is_horizontal_same_as(&db, &nephew),
    ///     "Expected {horizontal_parent} to be horizontal same-as"
    /// );
    ///
    /// for (fk, horizontal_same_as, hypothenuse_same_as) in [
    ///     (mandatory_triangular_grandparent, Some(horizontal_grandparent), grandparent_fk),
    ///     (mandatory_triangular_parent, Some(horizontal_parent), parent_fk),
    ///     (discretionary_triangular_grandparent, None, grandparent_fk),
    ///     (discretionary_triangular_parent, None, parent_fk),
    /// ] {
    ///     let Some(triangle) = fk.triangular_same_as(&db, &nephew) else {
    ///         panic!("Expected triangular same-as relationship: `{}`", fk.to_string());
    ///     };
    ///     assert_eq!(
    ///         triangle.horizontal_same_as(),
    ///         horizontal_same_as,
    ///         "Expected horizontal same-as foreign key to match"
    ///     );
    ///     assert_eq!(
    ///         triangle.hypothenuse_same_as(),
    ///         hypothenuse_same_as,
    ///         "Expected hypothenuse same-as foreign key to match"
    ///     );
    /// }
    ///
    /// Ok(())
    /// # }
    /// ```
    fn triangular_same_as<'db>(
        &self,
        database: &'db Self::Database,
        host_table: &'db Self::Table,
    ) -> Option<Triangular<'db, Self>> {
        // A triangular constraint must be a foreign primary key.
        // and if the foreign key is self-referential, we do not consider it
        // a triangular constraint.
        if self.is_self_referential(database, host_table)
            || !self.is_referenced_primary_key(database)
        {
            return None;
        }

        let foreign_table = self.referenced_table(database);
        // If the source table is a descendant of the foreign table,
        // we do not consider it a triangular constraint.
        if host_table.shares_ancestors_with(database, &foreign_table) {
            return None;
        }

        // At this point, we need to identify foreign keys in the
        // foreign table which point to ancestors of the current table.
        let hypothenuses_same_as =
            foreign_table.foreign_keys_to_ancestors_of(database, host_table).collect::<Vec<_>>();

        if hypothenuses_same_as.is_empty() {
            return None;
        }

        let candidate_host_columns = self.host_columns(database, host_table).collect::<Vec<_>>();
        let candidate_referenced_columns = self.referenced_columns(database).collect::<Vec<_>>();

        // For each foreign key in the host table, we check whether it contains
        // references to the specific ID contained in the local & foreign columns
        // of the current constraint, which implicitly also checks whether the
        // foreign key points to the same foreign table as the current constraint.
        // Next, we check whether the foreign key's foreign columns contain
        // any of the columns pointing to ancestors of the host table described
        // in constraints to ancestors of the host table which we determined above.
        for horizontal_same_as in host_table.horizontal_same_as_foreign_keys(database) {
            // We retrieve the local columns of the foreign key we are checking.
            let fk_local_columns =
                horizontal_same_as.host_columns(database, host_table).collect::<Vec<_>>();
            // If all of the columns involved in the current constraint are
            // present in the local columns of the foreign key, we proceed
            // to check the foreign columns.
            if !candidate_host_columns.iter().all(|c| fk_local_columns.contains(c)) {
                continue;
            }

            let fk_referenced_columns =
                horizontal_same_as.referenced_columns(database).collect::<Vec<_>>();
            // All of the referenced columns of the current constraint must
            // be present in the referenced columns of the foreign key.
            if !candidate_referenced_columns.iter().all(|c| fk_referenced_columns.contains(c)) {
                continue;
            }

            // Now that we have established that the foreign key involves
            // all of the local & foreign columns of the current constraint,
            // we need to find at least one case where the foreign key's
            // referenced columns contain all of the columns in a
            // `columns_to_local_ancestors`.
            for hypothenuse_same_as in &hypothenuses_same_as {
                if hypothenuse_same_as
                    .host_columns(database, foreign_table)
                    .all(|c: &Self::Column| fk_referenced_columns.contains(&c))
                {
                    return Some(Triangular {
                        horizontal_same_as: Some(horizontal_same_as),
                        hypothenuse_same_as,
                    });
                }
            }
        }

        Some(Triangular { horizontal_same_as: None, hypothenuse_same_as: &hypothenuses_same_as[0] })
    }
}

impl<T> TriangularSameAsForeignKeyLike for T where T: HorizontalSameAsForeignKeyLike {}
