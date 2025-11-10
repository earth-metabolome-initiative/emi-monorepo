# SynQL Insertable Key Settable

Crate providing tooling to generate a `*InsertableKeySettable` trait for SynQL models.

These settable traits allow for setting keys involves in foreign keys or relations. This trait is most commonly then implemented for both the `New*` struct, allowing for easy construction of new records with related keys.
