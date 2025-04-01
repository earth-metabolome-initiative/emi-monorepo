#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_operations::directus_operations
)]
pub struct DirectusOperation {
    pub id: uuid::Uuid,
    pub name: Option<String>,
    pub key: String,
    pub r#type: String,
    pub position_x: i32,
    pub position_y: i32,
    pub options: Option<serde_json::Value>,
    pub resolve: Option<uuid::Uuid>,
    pub reject: Option<uuid::Uuid>,
    pub flow: uuid::Uuid,
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    pub user_created: Option<uuid::Uuid>,
}
impl DirectusOperation {
    #[cfg(feature = "postgres")]
    pub async fn resolve(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
        >,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(resolve) = self.resolve.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation::table()
            .find(resolve)
            .first::<
                crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_resolve(
        conn: &mut diesel_async::AsyncPgConnection,
        resolve: &crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::resolve
                    .eq(&resolve.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn reject(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
        >,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(reject) = self.reject.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation::table()
            .find(reject)
            .first::<
                crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_reject(
        conn: &mut diesel_async::AsyncPgConnection,
        reject: &crate::codegen::structs_codegen::tables::directus_operations::DirectusOperation,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::reject
                    .eq(&reject.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn flow(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow::table()
            .find(&self.flow)
            .first::<
                crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
            >(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_flow(
        conn: &mut diesel_async::AsyncPgConnection,
        flow: &crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::flow
                    .eq(&flow.id),
            )
            .load::<Self>(conn)
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
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(user_created) = self.user_created.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_users::DirectusUser::table()
            .find(user_created)
            .first::<
                crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_user_created(
        conn: &mut diesel_async::AsyncPgConnection,
        user_created: &crate::codegen::structs_codegen::tables::directus_users::DirectusUser,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_operations::directus_operations::dsl::user_created
                    .eq(&user_created.id),
            )
            .load::<Self>(conn)
            .await
    }
}
