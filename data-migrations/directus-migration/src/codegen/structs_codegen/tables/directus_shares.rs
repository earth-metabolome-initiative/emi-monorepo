#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_shares::directus_shares
)]
pub struct DirectusShare {
    pub id: rosetta_uuid::Uuid,
    pub name: Option<String>,
    pub collection: String,
    pub item: String,
    pub role: Option<rosetta_uuid::Uuid>,
    pub password: Option<String>,
    pub user_created: Option<rosetta_uuid::Uuid>,
    pub date_created: Option<rosetta_timestamp::TimestampUTC>,
    pub date_start: Option<rosetta_timestamp::TimestampUTC>,
    pub date_end: Option<rosetta_timestamp::TimestampUTC>,
    pub times_used: Option<i32>,
    pub max_uses: Option<i32>,
}
impl DirectusShare {
    #[cfg(feature = "postgres")]
    pub async fn collection(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_collections::directus_collections::dsl::collection
                    .eq(&self.collection),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn role(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(role) = self.role.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_roles::directus_roles::dsl::id
                    .eq(role),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn user_created(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(user_created) = self.user_created.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_users::directus_users::dsl::id
                    .eq(user_created),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_users::DirectusUser>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_collection(
        conn: &mut diesel_async::AsyncPgConnection,
        collection: &crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_shares::directus_shares::dsl::collection
                    .eq(&collection.collection),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_role(
        conn: &mut diesel_async::AsyncPgConnection,
        role: &crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_shares::directus_shares::dsl::role
                    .eq(role.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user_created(
        conn: &mut diesel_async::AsyncPgConnection,
        user_created: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_shares::directus_shares::dsl::user_created
                    .eq(user_created.id),
            )
            .load::<Self>(conn)
            .await
    }
}
