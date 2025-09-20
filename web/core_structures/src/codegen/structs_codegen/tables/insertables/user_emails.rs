#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UserEmailAttribute {
    Id,
    Email,
    CreatedBy,
    CreatedAt,
    PrimaryEmail,
}
impl core::str::FromStr for UserEmailAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Email" => Ok(Self::Email),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "PrimaryEmail" => Ok(Self::PrimaryEmail),
            "email" => Ok(Self::Email),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            "primary_email" => Ok(Self::PrimaryEmail),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
{
    type Attribute = UserEmailAttribute;
}
impl web_common_traits::database::TableField for UserEmailAttribute {}
impl web_common_traits::database::HasTableType for UserEmailAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl core::fmt::Display for UserEmailAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Id => write!(f, "user_emails.id"),
            Self::Email => write!(f, "user_emails.email"),
            Self::CreatedBy => write!(f, "user_emails.created_by"),
            Self::CreatedAt => write!(f, "user_emails.created_at"),
            Self::PrimaryEmail => write!(f, "user_emails.primary_email"),
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::user_emails::user_emails)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserEmail {
    pub(crate) email: String,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
    pub(crate) primary_email: bool,
}
impl InsertableUserEmail {
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.created_by, conn)
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`UserEmail`](crate::codegen::structs_codegen::tables::user_emails::UserEmail).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::UserEmail;
/// use core_structures::tables::insertables::UserEmailSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let user_email = UserEmail::new()
///    // Set mandatory fields
///    .created_by(created_by)?
///    .email(email)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .primary_email(primary_email)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableUserEmailBuilder {
    pub(crate) email: Option<String>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
    pub(crate) primary_email: Option<bool>,
}
impl diesel::associations::HasTable for InsertableUserEmailBuilder {
    type Table = crate::codegen::diesel_codegen::tables::user_emails::user_emails::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::user_emails::user_emails::table
    }
}
impl From<InsertableUserEmailBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertableUserEmailBuilder>
{
    fn from(builder: InsertableUserEmailBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl Default for InsertableUserEmailBuilder {
    fn default() -> Self {
        Self {
            email: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
            primary_email: Some(true),
        }
    }
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableUserEmailBuilder
{
    fn is_complete(&self) -> bool {
        self.email.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
            && self.primary_email.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `UserEmail` or
/// descendant tables.
pub trait UserEmailSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
    /// Sets the value of the `public.user_emails.email` column.
    ///
    /// # Arguments
    /// * `email`: The value to set for the `public.user_emails.email` column.
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
    fn email<E>(self, email: E) -> Result<Self, Self::Error>
    where
        E: TryInto<String>,
        validation_errors::SingleFieldError: From<<E as TryInto<String>>::Error>;
    /// Sets the value of the `public.user_emails.created_by` column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the `public.user_emails.created_by`
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
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_by<CB>(self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.user_emails.created_at` column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the `public.user_emails.created_at`
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
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
    /// Sets the value of the `public.user_emails.primary_email` column.
    ///
    /// # Arguments
    /// * `primary_email`: The value to set for the
    ///   `public.user_emails.primary_email` column.
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
    /// * If the provided value cannot be converted to the required type `bool`.
    /// * If the provided value does not pass schema-defined validation.
    fn primary_email<PE>(self, primary_email: PE) -> Result<Self, Self::Error>
    where
        PE: TryInto<bool>,
        validation_errors::SingleFieldError: From<<PE as TryInto<bool>>::Error>;
}
impl UserEmailSettable for InsertableUserEmailBuilder
where
    Self: common_traits::builder::Attributed<
            Attribute = crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute,
        >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    /// Sets the value of the `public.user_emails.email` column.
    fn email<E>(mut self, email: E) -> Result<Self, Self::Error>
    where
        E: TryInto<String>,
        validation_errors::SingleFieldError: From<<E as TryInto<String>>::Error>,
    {
        let email = email.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err).rename_field(UserEmailAttribute::Email)
        })?;
        pgrx_validation::must_be_email(email.as_ref()).map_err(|e| {
            e.rename_field(
                crate::codegen::structs_codegen::tables::insertables::UserEmailAttribute::Email,
            )
        })?;
        self.email = Some(email);
        Ok(self)
    }
    /// Sets the value of the `public.user_emails.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let created_by =
            <CB as web_common_traits::database::PrimaryKeyLike>::primary_key(&created_by);
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.user_emails.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(UserEmailAttribute::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
    /// Sets the value of the `public.user_emails.primary_email` column.
    fn primary_email<PE>(mut self, primary_email: PE) -> Result<Self, Self::Error>
    where
        PE: TryInto<bool>,
        validation_errors::SingleFieldError: From<<PE as TryInto<bool>>::Error>,
    {
        let primary_email = primary_email.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(UserEmailAttribute::PrimaryEmail)
        })?;
        self.primary_email = Some(primary_email);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableUserEmailBuilder {
    type PrimaryKey = i32;
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableUserEmailBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::user_emails::UserEmail,
            Error = web_common_traits::database::InsertError<UserEmailAttribute>,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<UserEmailAttribute>;
    fn mint_primary_key(self, user_id: i32, conn: &mut C) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
