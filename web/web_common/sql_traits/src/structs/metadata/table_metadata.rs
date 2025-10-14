//! Submodule defining a generic `TableMetadata` struct.

use std::rc::Rc;

use crate::traits::TableLike;

#[derive(Debug, Clone)]
/// Metadata about a database table.
pub struct TableMetadata<T: TableLike> {
    /// The columns of the table.
    columns: Vec<Rc<T::Column>>,
    /// The check constraints of the table.
    check_constraints: Vec<Rc<T::CheckConstraint>>,
    /// The unique indices of the table.
    unique_indices: Vec<Rc<T::UniqueIndex>>,
    /// The foreign keys of the table.
    foreign_keys: Vec<Rc<T::ForeignKey>>,
    /// The columns composing the primary key of the table.
    primary_key: Vec<Rc<T::Column>>,
}

impl<T: TableLike> Default for TableMetadata<T> {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            check_constraints: Vec::new(),
            unique_indices: Vec::new(),
            foreign_keys: Vec::new(),
            primary_key: Vec::new(),
        }
    }
}

impl<T: TableLike> TableMetadata<T> {
    /// Returns an iterator over the references of columns of the table.
    pub fn columns(&self) -> impl Iterator<Item = &T::Column> {
        self.columns.iter().map(|col| col.as_ref())
    }

    /// Returns an iterator over the Rc of columns of the table.
    pub fn column_rcs(&self) -> impl Iterator<Item = &Rc<T::Column>> {
        self.columns.iter()
    }

    /// Returns an iterator over the check constraints of the table.
    pub fn check_constraints(&self) -> impl Iterator<Item = &T::CheckConstraint> {
        self.check_constraints.iter().map(|cc| cc.as_ref())
    }

    /// Returns an iterator over the unique indices of the table.
    pub fn unique_indices(&self) -> impl Iterator<Item = &T::UniqueIndex> {
        self.unique_indices.iter().map(|idx| idx.as_ref())
    }

    /// Returns an iterator over the Rc of unique indices of the table.
    pub fn unique_index_rcs(&self) -> impl Iterator<Item = &Rc<T::UniqueIndex>> {
        self.unique_indices.iter()
    }

    /// Returns an iterator over the foreign keys of the table.
    pub fn foreign_keys(&self) -> impl Iterator<Item = &T::ForeignKey> {
        self.foreign_keys.iter().map(|fk| fk.as_ref())
    }

    /// Returns an iterator over the Rc of foreign keys of the table.
    pub fn foreign_key_rcs(&self) -> impl Iterator<Item = &Rc<T::ForeignKey>> {
        self.foreign_keys.iter()
    }

    /// Returns an iterator over the columns composing the primary key of the
    /// table.
    pub fn primary_key_columns(&self) -> impl Iterator<Item = &T::Column> {
        self.primary_key.iter().map(|col| col.as_ref())
    }

    /// Adds a column to the table metadata.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to add.
    pub fn add_column(&mut self, column: Rc<T::Column>) {
        self.columns.push(column);
    }

    /// Adds a check constraint to the table metadata.
    ///
    /// # Arguments
    ///
    /// * `constraint` - The check constraint to add.
    pub fn add_check_constraint(&mut self, constraint: Rc<T::CheckConstraint>) {
        self.check_constraints.push(constraint);
    }

    /// Adds a unique index to the table metadata.
    ///
    /// # Arguments
    ///
    /// * `index` - The unique index to add.
    pub fn add_unique_index(&mut self, index: Rc<T::UniqueIndex>) {
        self.unique_indices.push(index);
    }

    /// Adds a foreign key to the table metadata.
    ///
    /// # Arguments
    ///
    /// * `fk` - The foreign key to add.
    pub fn add_foreign_key(&mut self, fk: Rc<T::ForeignKey>) {
        self.foreign_keys.push(fk);
    }

    /// Sets the columns composing the primary key of the table.
    ///
    /// # Arguments
    ///
    /// * `pk_columns` - The columns composing the primary key.
    pub fn set_primary_key(&mut self, pk_columns: Vec<Rc<T::Column>>) {
        self.primary_key = pk_columns;
    }
}
