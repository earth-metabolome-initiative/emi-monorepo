#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
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
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, RunQueryDsl};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.user_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_user_id(
        user_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::user_organizations::user_organizations;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(user_organizations::user_id.eq(user_id))
            .order_by((
                user_organizations::user_id.asc(),
                user_organizations::organization_id.asc(),
            ))
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_organization_id(
        organization_id: &i16,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::user_organizations::user_organizations;
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{ExpressionMethods, QueryDsl};
        Self::table()
            .filter(user_organizations::organization_id.eq(organization_id))
            .order_by((
                user_organizations::user_id.asc(),
                user_organizations::organization_id.asc(),
            ))
            .load::<Self>(conn)
    }
}
impl AsRef<UserOrganization> for UserOrganization {
    fn as_ref(&self) -> &UserOrganization {
        self
    }
}
