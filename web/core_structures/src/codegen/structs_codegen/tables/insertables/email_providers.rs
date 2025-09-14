#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EmailProviderAttribute {
    EmailId,
    LoginProviderId,
}
impl core::str::FromStr for EmailProviderAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EmailId" => Ok(Self::EmailId),
            "LoginProviderId" => Ok(Self::LoginProviderId),
            "email_id" => Ok(Self::EmailId),
            "login_provider_id" => Ok(Self::LoginProviderId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl common_traits::builder::Attributed
    for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
{
    type Attribute = EmailProviderAttribute;
}
impl core::fmt::Display for EmailProviderAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::EmailId => write!(f, "email_providers.email_id"),
            Self::LoginProviderId => write!(f, "email_providers.login_provider_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::email_providers::email_providers
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableEmailProvider {
    pub(crate) email_id: i32,
    pub(crate) login_provider_id: i16,
}
impl InsertableEmailProvider {
    pub fn email<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::user_emails::UserEmail,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::user_emails::UserEmail:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::user_emails::UserEmail::read(self.email_id, conn)
    }
    pub fn login_provider<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::login_providers::LoginProvider::read(
            self.login_provider_id,
            conn,
        )
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`EmailProvider`](crate::codegen::structs_codegen::tables::email_providers::EmailProvider).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::EmailProvider;
/// use core_structures::tables::insertables::EmailProviderSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let email_provider = EmailProvider::new()
///    // Set mandatory fields
///    .email(email_id)?
///    .login_provider(login_provider_id)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertableEmailProviderBuilder {
    pub(crate) email_id: Option<i32>,
    pub(crate) login_provider_id: Option<i16>,
}
impl common_traits::builder::IsCompleteBuilder
    for crate::codegen::structs_codegen::tables::insertables::InsertableEmailProviderBuilder
{
    fn is_complete(&self) -> bool {
        self.email_id.is_some() && self.login_provider_id.is_some()
    }
}
/// Trait defining setters for attributes of an instance of `EmailProvider` or
/// descendant tables.
pub trait EmailProviderSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.email_providers.email_id` column.
    ///
    /// # Arguments
    /// * `email_id`: The value to set for the `public.email_providers.email_id`
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
    fn email<EI>(
        self,
        email_id: EI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        EI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>;
    /// Sets the value of the `public.email_providers.login_provider_id` column.
    ///
    /// # Arguments
    /// * `login_provider_id`: The value to set for the
    ///   `public.email_providers.login_provider_id` column.
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
    /// * If the provided value cannot be converted to the required type `i16`.
    /// * If the provided value does not pass schema-defined validation.
    fn login_provider<LPI>(
        self,
        login_provider_id: LPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        LPI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>;
}
impl EmailProviderSettable for InsertableEmailProviderBuilder {
    type Attributes = crate::codegen::structs_codegen::tables::insertables::EmailProviderAttribute;
    /// Sets the value of the `public.email_providers.email_id` column.
    fn email<EI>(
        mut self,
        email_id: EI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        EI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        let email_id = <EI as web_common_traits::database::PrimaryKeyLike>::primary_key(&email_id);
        self.email_id = Some(email_id);
        Ok(self)
    }
    /// Sets the value of the `public.email_providers.login_provider_id` column.
    fn login_provider<LPI>(
        mut self,
        login_provider_id: LPI,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        LPI: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i16>,
    {
        let login_provider_id =
            <LPI as web_common_traits::database::PrimaryKeyLike>::primary_key(&login_provider_id);
        self.login_provider_id = Some(login_provider_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableEmailProviderBuilder {
    type PrimaryKey = (i32, i16);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableEmailProviderBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::email_providers::EmailProvider,
            Attribute = EmailProviderAttribute,
        >,
{
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attribute>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::email_providers::EmailProvider =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
