#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::organizations::Organization,
        foreign_key = organization_id
    )
)]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::users::User,
        foreign_key = user_id
    )
)]
#[diesel(primary_key(user_id, organization_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::user_organizations::user_organizations
)]
pub struct UserOrganization {
    pub user_id: i32,
    pub organization_id: i16,
}
impl web_common_traits::prelude::TableName for UserOrganization {
    const TABLE_NAME: &'static str = "user_organizations";
}
impl<'a> From<&'a UserOrganization>
    for web_common_traits::database::IdOrBuilder<
        (i32, i16),
        crate::codegen::structs_codegen::tables::insertables::InsertableUserOrganizationBuilder,
    >
{
    fn from(value: &'a UserOrganization) -> Self {
        web_common_traits::database::IdOrBuilder::Id((value.user_id, value.organization_id))
    }
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::user_organizations::UserOrganization,
    > for UserOrganization
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a (i32, i16)>,
{
}
impl diesel::Identifiable for UserOrganization {
    type Id = (i32, i16);
    fn id(self) -> Self::Id {
        (self.user_id, self.organization_id)
    }
}
impl UserOrganization {
    pub fn organization<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::organizations::Organization,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::organizations::Organization:
            web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::organizations::Organization::read(
            self.organization_id,
            conn,
        )
    }
    pub fn user<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::users::User, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::users::User: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::users::User::read(self.user_id, conn)
    }
}
impl AsRef<UserOrganization> for UserOrganization {
    fn as_ref(&self) -> &UserOrganization {
        self
    }
}
