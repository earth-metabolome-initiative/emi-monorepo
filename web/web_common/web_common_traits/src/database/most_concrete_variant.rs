//! Submodule providing traits to execute queries to load the most concrete
//! variant of a table DAG.

/// Trait defining metadata for the most concrete variant of a table DAG.
pub trait MostConcreteVariantMetadata {
    /// Type of the enumeration representing the possible variants
    /// of the table DAG.
    type Variant;
}

/// Trait defining the ability to load the most concrete variant of a table DAG.
pub trait MostConcreteVariant<C>: MostConcreteVariantMetadata {
    /// Returns the most concrete variant of the table DAG for the current
    /// instance.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a database connection.
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the most concrete variant.
    fn most_concrete_variant(&self, conn: &mut C) -> Result<Self::Variant, diesel::result::Error>;
}
