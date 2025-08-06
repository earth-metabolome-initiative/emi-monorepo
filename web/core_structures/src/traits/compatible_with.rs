#![cfg(feature = "postgres")]

//! Submodule defining the `AppendProcedureModel` trait.

use rosetta_uuid::Uuid;
use web_common_traits::{
    database::{InsertError, Insertable, InsertableVariant},
    prelude::ExtensionTable,
};

use crate::{
    CompatibilityRule, Trackable, tables::insertables::InsertableCompatibilityRuleAttributes,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure models.
pub trait CompatibleWith: ExtensionTable<Trackable>
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a Uuid>,
{
    /// Creates a new `CompatibilityRule` linking the current trackable with
    /// another
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another trackable item that this one is
    ///   compatible with.
    /// * `user` - The user performing the operation, used for tracking who
    ///   created the compatibility rule.
    /// * `conn` - A mutable reference to the database connection where the
    ///   operation will be performed.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn compatible_with<T>(
        &self,
        other: &T,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<CompatibilityRule, InsertError<InsertableCompatibilityRuleAttributes>>
    where
        T: ExtensionTable<Trackable>,
        for<'a> &'a T: diesel::Identifiable<Id = &'a Uuid>,
    {
        use diesel::Identifiable;

        // Then, we create a new NextProcedureModel entry linking the parent
        // procedure to the new child procedure.
        CompatibilityRule::new()
            .left_trackable(*self.id())?
            .right_trackable(*other.id())?
            .created_by(user.id)?
            .insert(user.id, conn)
    }

    /// Creates a new `CompatibilityRule` linking the current trackable with
    /// another
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another trackable item that this one is
    ///   compatible with.
    /// * `quantity` - An integer representing the quantity of the compatibility
    ///   rule.
    /// * `user` - The user performing the operation, used for tracking who
    ///   created the compatibility rule.
    /// * `conn` - A mutable reference to the database connection where the
    ///   operation will be performed.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, an `InsertError` is returned.
    fn compatible_with_quantity<T>(
        &self,
        other: &T,
        quantity: i16,
        user: &crate::User,
        conn: &mut diesel::PgConnection,
    ) -> Result<CompatibilityRule, InsertError<InsertableCompatibilityRuleAttributes>>
    where
        T: ExtensionTable<Trackable>,
        for<'a> &'a T: diesel::Identifiable<Id = &'a Uuid>,
    {
        use diesel::Identifiable;

        // Then, we create a new NextProcedureModel entry linking the parent
        // procedure to the new child procedure.
        CompatibilityRule::new()
            .left_trackable(*self.id())?
            .right_trackable(*other.id())?
            .quantity(quantity)?
            .created_by(user.id)?
            .insert(user.id, conn)
    }
}

impl<T> CompatibleWith for T
where
    T: ExtensionTable<Trackable>,
    for<'a> &'a T: diesel::Identifiable<Id = &'a Uuid>,
{
}
