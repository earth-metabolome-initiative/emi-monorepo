//! Submodule for the `InsertOperation` trait and related traits.

use common_traits::prelude::{Basic, Builder};

/// Trait for types that can be inserted into the database.
pub trait InsertableVariant: Basic {
    /// The associated type that is inserted into the database.
    type Row: InsertableRow<New = Self>;
    /// The associated builder type.
    type Builder: InsertableVariantBuilder<New = Self>;

    #[cfg(feature = "backend")]
    /// Insert the variant into the database.
    fn insert(
        self,
        conn: &mut crate::types::DBConn,
    ) -> impl std::future::Future<Output = Result<Self::Row, diesel::result::Error>> + '_;
}

/// Trait defining a builder for the insertable variant.
pub trait InsertableVariantBuilder:
    Builder<Object = <Self as InsertableVariantBuilder>::New>
{
    /// The associated type that is inserted into the database.
    type New: InsertableVariant<Builder = Self>;
}

/// Trait for types that have a new variant that can be inserted into the
/// database.
pub trait InsertableRow: Basic {
    /// The associated type that is inserted into the database.
    type New: InsertableVariant<Row = Self>;
    /// The associated builder type.
    type Builder: InsertableVariantBuilder<New = Self::New>;

    /// Returns a new insertable variant builder.
    fn insert_builder() -> Self::Builder {
        Default::default()
    }
}
