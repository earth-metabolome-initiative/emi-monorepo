#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UserAttribute {
    Id,
    FirstName,
    LastName,
    CreatedAt,
    UpdatedAt,
}
impl core::str::FromStr for UserAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FirstName" => Ok(Self::FirstName),
            "LastName" => Ok(Self::LastName),
            "CreatedAt" => Ok(Self::CreatedAt),
            "UpdatedAt" => Ok(Self::UpdatedAt),
            "first_name" => Ok(Self::FirstName),
            "last_name" => Ok(Self::LastName),
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
{
    type Attribute = UserAttribute;
}
impl web_common_traits::database::TableField for UserAttribute {}
impl web_common_traits::database::HasTableType for UserAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl core::fmt::Display for UserAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "users.id"),
            Self::FirstName => write!(f, "users.first_name"),
            Self::LastName => write!(f, "users.last_name"),
            Self::CreatedAt => write!(f, "users.created_at"),
            Self::UpdatedAt => write!(f, "users.updated_at"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::users::users)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUser {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) updated_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableUser {}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`User`](crate::codegen::structs_codegen::tables::users::User).
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::User;
/// use core_structures::tables::insertables::UserSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let user = User::new()
///    // Set mandatory fields
///    .first_name(first_name)?
///    .last_name(last_name)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableUserBuilder {
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) updated_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl diesel::associations::HasTable for InsertableUserBuilder {
    type Table = crate::codegen::diesel_codegen::tables::users::users::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::users::users::table
    }
}
impl From<InsertableUserBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableUserBuilder>
{
    fn from(builder: InsertableUserBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableUserBuilder {
    fn default() -> Self {
        Self {
            first_name: Default::default(),
            last_name: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            updated_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserBuilder
{
    fn is_complete(&self) -> bool {
        self.first_name.is_some()
            && self.last_name.is_some()
            && self.created_at.is_some()
            && self.updated_at.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `User` or descendant
/// tables.
pub trait UserSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.users.first_name` column.
    ///
    /// # Arguments
    /// * `first_name`: The value to set for the `public.users.first_name`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn first_name<FN>(self, first_name: FN) -> Result<Self, Self::Error>
    where
        FN: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<FN as TryInto<String>>::Error>;
    /// Sets the value of the `public.users.last_name` column.
    ///
    /// # Arguments
    /// * `last_name`: The value to set for the `public.users.last_name` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `String`.
    /// * If the provided value does not pass schema-defined validation.
    fn last_name<LN>(self, last_name: LN) -> Result<Self, Self::Error>
    where
        LN: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<LN as TryInto<String>>::Error>;
    /// Sets the value of the `public.users.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the `public.users.created_at`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_at<CA>(self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.users.updated_at` column.
    ///
    /// # Arguments
    /// * `updated_at`: The value to set for the `public.users.updated_at`
    ///   column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn updated_at<UA>(self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
}
impl UserSettable for InsertableUserBuilder
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::UserAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.users.first_name` column.
    fn first_name<FN>(mut self, first_name: FN) -> Result<Self, Self::Error>
    where
        FN: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<FN as TryInto<String>>::Error>,
    {
        let first_name = first_name.try_into().map_err(|err| {
            validation_errors::prelude::SingleFieldError::from(err)
                .rename_field(UserAttribute::FirstName)
        })?;
        pgrx_validation::must_be_paragraph(first_name.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::UserAttribute::FirstName,
            )
        })?;
        self.first_name = Some(first_name);
        Ok(self)
    }
    /// Sets the value of the `public.users.last_name` column.
    fn last_name<LN>(mut self, last_name: LN) -> Result<Self, Self::Error>
    where
        LN: TryInto<String>,
        validation_errors::prelude::SingleFieldError: From<<LN as TryInto<String>>::Error>,
    {
        let last_name = last_name.try_into().map_err(|err| {
            validation_errors::prelude::SingleFieldError::from(err)
                .rename_field(UserAttribute::LastName)
        })?;
        pgrx_validation::must_be_paragraph(last_name.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::UserAttribute::LastName,
            )
        })?;
        self.last_name = Some(last_name);
        Ok(self)
    }
    /// Sets the value of the `public.users.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::prelude::SingleFieldError::from(err)
                .rename_field(UserAttribute::CreatedAt)
        })?;
        if let Some(updated_at) = self.updated_at {
            pgrx_validation::created_at
                <= updated_at.map_err(|e| {
                    e.rename_fields(
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::CreatedAt,
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::UpdatedAt,
                )
                })?;
        }
        self.created_at = Some(created_at);
        Ok(self)
    }
    /// Sets the value of the `public.users.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::prelude::SingleFieldError:
            From<<UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let updated_at = updated_at.try_into().map_err(|err| {
            validation_errors::prelude::SingleFieldError::from(err)
                .rename_field(UserAttribute::UpdatedAt)
        })?;
        if let Some(created_at) = self.created_at {
            pgrx_validation::created_at
                <= updated_at.map_err(|e| {
                    e.rename_fields(
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::CreatedAt,
                    crate::codegen::structs_codegen::tables::insertables::UserAttribute::UpdatedAt,
                )
                })?;
        }
        self.updated_at = Some(updated_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableUserBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableUserBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::users::User,
            Error = web_common_traits::database::InsertError<UserAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<UserAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
