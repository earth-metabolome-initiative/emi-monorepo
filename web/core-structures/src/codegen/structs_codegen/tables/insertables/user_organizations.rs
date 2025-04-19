#[derive(Clone, core::fmt::Debug)]
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
    #[cfg(feature = "postgres")]
    pub async fn user(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::users::User::table()
            .filter(crate::codegen::diesel_codegen::tables::users::users::dsl::id.eq(&self.user_id))
            .first::<crate::codegen::structs_codegen::tables::users::User>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn organization(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organizations::Organization,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::organizations::Organization::table()
            .filter(
                crate::codegen::diesel_codegen::tables::organizations::organizations::dsl::id
                    .eq(&self.organization_id),
            )
            .first::<crate::codegen::structs_codegen::tables::organizations::Organization>(conn)
            .await
    }
}
#[derive(Default)]
pub struct InsertableUserOrganizationBuilder {
    user_id: Option<i32>,
    organization_id: Option<i16>,
}
impl InsertableUserOrganizationBuilder {
    pub fn user_id(
        mut self,
        user_id: i32,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.user_id = Some(user_id);
        Ok(self)
    }
    pub fn organization_id(
        mut self,
        organization_id: i16,
    ) -> Result<Self, <Self as common_traits::prelude::Builder>::Error> {
        self.organization_id = Some(organization_id);
        Ok(self)
    }
}
impl common_traits::prelude::Builder for InsertableUserOrganizationBuilder {
    type Error = web_common_traits::database::InsertError<InsertableUserOrganizationAttributes>;
    type Object = InsertableUserOrganization;
    type Attribute = InsertableUserOrganizationAttributes;
    fn build(self) -> Result<Self::Object, Self::Error> {
        Ok(Self::Object {
            user_id: self.user_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserOrganizationAttributes::UserId,
                )
            })?,
            organization_id: self.organization_id.ok_or_else(|| {
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableUserOrganizationAttributes::OrganizationId,
                )
            })?,
        })
    }
}
impl TryFrom<InsertableUserOrganization> for InsertableUserOrganizationBuilder {
    type Error = <Self as common_traits::prelude::Builder>::Error;
    fn try_from(insertable_variant: InsertableUserOrganization) -> Result<Self, Self::Error> {
        Ok(Self::default()
            .user_id(insertable_variant.user_id)?
            .organization_id(insertable_variant.organization_id)?)
    }
}
