#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_access::directus_access
)]
pub struct DirectusAccess {
    pub id: rosetta_uuid::Uuid,
    pub role: Option<rosetta_uuid::Uuid>,
    pub user: Option<rosetta_uuid::Uuid>,
    pub policy: rosetta_uuid::Uuid,
    pub sort: Option<i32>,
}
impl DirectusAccess {
    #[cfg(feature = "postgres")]
    pub async fn role(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(role) = self.role.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_roles::directus_roles::dsl::id
                    .eq(role),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn user(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        let Some(user) = self.user.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn policy(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_policies::directus_policies::dsl::id
                    .eq(&self.policy),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_role(
        conn: &mut diesel_async::AsyncPgConnection,
        role: &crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_access::directus_access::dsl::role
                    .eq(role.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user(
        conn: &mut diesel_async::AsyncPgConnection,
        user: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_access::directus_access::dsl::user
                    .eq(user.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_policy(
        conn: &mut diesel_async::AsyncPgConnection,
        policy: &crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_access::directus_access::dsl::policy
                    .eq(policy.id),
            )
            .load::<Self>(conn)
            .await
    }
}
