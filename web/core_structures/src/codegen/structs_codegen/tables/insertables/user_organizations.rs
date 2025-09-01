#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUserOrganizationAttributes {
    UserId,
    OrganizationId,
}
impl core::str::FromStr for InsertableUserOrganizationAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UserId" => Ok(Self::UserId),
            "OrganizationId" => Ok(Self::OrganizationId),
            "user_id" => Ok(Self::UserId),
            "organization_id" => Ok(Self::OrganizationId),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for InsertableUserOrganizationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::UserId => write!(f, "user_id"),
            Self::OrganizationId => write!(f, "organization_id"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::user_organizations::user_organizations
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserOrganization {
    pub(crate) user_id: i32,
    pub(crate) organization_id: i16,
}
impl InsertableUserOrganization {
    pub fn user<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.user_id,
            ),
            conn,
        )
    }
    pub fn organization<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organizations::Organization,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::organizations::Organization: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::organizations::Organization as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::organizations::Organization as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::organizations::Organization as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::organizations::Organization,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::organizations::Organization::table(),
                self.organization_id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserOrganizationBuilder {
    pub(crate) user_id: Option<i32>,
    pub(crate) organization_id: Option<i16>,
}
/// Trait defining setters for attributes of an instance of `UserOrganization`
/// or descendant tables.
pub trait UserOrganizationBuildable: std::marker::Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the `public.user_organizations.user_id` column.
    ///
    /// # Arguments
    /// * `user_id`: The value to set for the
    ///   `public.user_organizations.user_id` column.
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
    fn user(
        self,
        user_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.user_organizations.organization_id`
    /// column.
    ///
    /// # Arguments
    /// * `organization_id`: The value to set for the
    ///   `public.user_organizations.organization_id` column.
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
    fn organization(
        self,
        organization_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
}
impl UserOrganizationBuildable for InsertableUserOrganizationBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationAttributes;
    /// Sets the value of the `public.user_organizations.user_id` column.
    fn user(
        mut self,
        user_id: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let user_id = user_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableUserOrganizationAttributes::UserId)
        })?;
        self.user_id = Some(user_id);
        Ok(self)
    }
    /// Sets the value of the `public.user_organizations.organization_id`
    /// column.
    fn organization(
        mut self,
        organization_id: i16,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        let organization_id = organization_id.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(InsertableUserOrganizationAttributes::OrganizationId)
        })?;
        self.organization_id = Some(organization_id);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableUserOrganizationBuilder {
    type PrimaryKey = (i32, i16);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableUserOrganizationBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
            Error = web_common_traits::database::InsertError<InsertableUserOrganizationAttributes>,
        >,
{
    type Attributes = InsertableUserOrganizationAttributes;
    fn is_complete(&self) -> bool {
        self.user_id.is_some() && self.organization_id.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::user_organizations::UserOrganization = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
