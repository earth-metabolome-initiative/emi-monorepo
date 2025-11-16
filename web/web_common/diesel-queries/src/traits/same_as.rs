//! Handles same-as updates for Diesel model builders.

/// Trait defining the existance of a same-as relationship.
pub trait SameAs<Referenced: diesel::Column>: diesel::Column {}

/// All columns are same-as themselves.
impl<C: diesel::Column> SameAs<C> for C {}
