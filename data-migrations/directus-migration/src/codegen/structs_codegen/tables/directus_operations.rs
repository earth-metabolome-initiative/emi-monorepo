#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_operations::directus_operations
)]
pub struct DirectusOperation {
    pub id: rosetta_uuid::Uuid,
    pub name: Option<String>,
    pub key: String,
    pub r#type: String,
    pub position_x: i32,
    pub position_y: i32,
    pub options: Option<serde_json::Value>,
    pub resolve: Option<rosetta_uuid::Uuid>,
    pub reject: Option<rosetta_uuid::Uuid>,
    pub flow: rosetta_uuid::Uuid,
    pub date_created: Option<rosetta_timestamp::TimestampUTC>,
    pub user_created: Option<rosetta_uuid::Uuid>,
}
impl diesel::Identifiable for DirectusOperation {
    type Id = rosetta_uuid::Uuid;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusOperation {
    #[cfg(feature = "postgres")]
    pub async fn resolve(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(resolve) = self.resolve.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::id
                    .eq(resolve),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn reject(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(reject) = self.reject.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::id
                    .eq(reject),
            )
            .first::<
                crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn flow(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_flows::directus_flows::dsl::id
                    .eq(&self.flow),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow>(conn)
            .await
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
    pub async fn from_resolve(
        conn: &mut diesel_async::AsyncPgConnection,
        resolve: &crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::resolve
                    .eq(resolve.id),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_reject(
        conn: &mut diesel_async::AsyncPgConnection,
        reject: &crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::reject
                    .eq(reject.id),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_flow(
        conn: &mut diesel_async::AsyncPgConnection,
        flow: &crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::flow
                    .eq(flow.id),
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
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::user_created
                    .eq(user_created.id),
            )
            .load::<Self>(conn)
            .await
    }
}
