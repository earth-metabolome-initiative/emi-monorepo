#[derive(Debug, Clone, PartialEq, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(diesel::Selectable, diesel::Insertable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(user_id, organization_id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::user_organizations::user_organizations
)]
pub struct UserOrganization {
    pub user_id: i32,
    pub organization_id: i16,
}
impl diesel::Identifiable for UserOrganization {
    type Id = (i32, i16);
    fn id(self) -> Self::Id {
        (self.user_id, self.organization_id)
    }
}
impl UserOrganization {
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
    #[cfg(feature = "postgres")]
    pub async fn from_user_id(
        conn: &mut diesel_async::AsyncPgConnection,
        user_id: &crate::codegen::structs_codegen::tables::users::User,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::dsl::user_id
                    .eq(user_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_organization_id(
        conn: &mut diesel_async::AsyncPgConnection,
        organization_id: &crate::codegen::structs_codegen::tables::organizations::Organization,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::user_organizations::user_organizations::dsl::organization_id
                    .eq(organization_id.id),
            )
            .load::<Self>(conn)
            .await
    }
}
