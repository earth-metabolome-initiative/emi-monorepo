# SynQL Scalar Settable

Crate providing tooling to generate a `*ScalarSettable` trait for SynQL models.

These settable traits allow for setting values which are not pointers, such as TEXT, INTEGER, BOOLEAN, etc, which
are not foreign keys or relations. This trait is most commonly then implemented for both the `New*` struct as well as
the associated `*Builder` struct, allowing for easy construction of new records with scalar values.
