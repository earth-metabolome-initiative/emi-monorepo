# SynQL Value Settable

Crate providing tooling to generate a `*ValueSettable` trait for SynQL models.

These settable traits allow for setting values which are not keys, such as TEXT, INTEGER, BOOLEAN, etc, which
are not foreign keys or relations. This trait is most commonly then implemented for both the `New*` struct as well as
the associated `*Builder` struct, allowing for easy construction of new records with scalar values.
