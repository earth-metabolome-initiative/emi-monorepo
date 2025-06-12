#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableUserOrganizationAttributes {
    UserId,
    OrganizationId,
}
impl core::fmt::Display for InsertableUserOrganizationAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableUserOrganizationAttributes::UserId => write!(f, "user_id"),
            InsertableUserOrganizationAttributes::OrganizationId => {
                write!(f, "organization_id")
            }
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
    user_id: i32,
    organization_id: i16,
}
impl InsertableUserOrganization {
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
}
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableUserOrganizationBuilder {
    pub(crate) user_id: Option<i32>,
    pub(crate) organization_id: Option<i16>,
}
impl InsertableUserOrganizationBuilder {
    pub fn user_id<P>(
        mut self,
        user_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserOrganizationAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let user_id = user_id.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableUserOrganizationAttributes::UserId)
        })?;
        self.user_id = Some(user_id);
        Ok(self)
    }
    pub fn organization_id<P>(
        mut self,
        organization_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableUserOrganizationAttributes>>
    where
        P: TryInto<i16>,
        <P as TryInto<i16>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let organization_id =
            organization_id.try_into().map_err(|err: <P as TryInto<i16>>::Error| {
                Into::into(err).rename_field(InsertableUserOrganizationAttributes::OrganizationId)
            })?;
        self.organization_id = Some(organization_id);
        Ok(self)
    }
}
impl TryFrom<InsertableUserOrganizationBuilder> for InsertableUserOrganization {
    type Error = common_traits::prelude::BuilderError<InsertableUserOrganizationAttributes>;
    fn try_from(
        builder: InsertableUserOrganizationBuilder,
    ) -> Result<InsertableUserOrganization, Self::Error> {
        Ok(Self {
            user_id: builder.user_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserOrganizationAttributes::UserId,
                ),
            )?,
            organization_id: builder.organization_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserOrganizationAttributes::OrganizationId,
                ),
            )?,
        })
    }
}
