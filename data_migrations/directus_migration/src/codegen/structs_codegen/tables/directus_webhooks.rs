#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks
)]
pub struct DirectusWebhook {
    pub id: i32,
    pub name: String,
    pub method: String,
    pub url: String,
    pub status: String,
    pub data: bool,
    pub actions: String,
    pub collections: String,
    pub headers: Option<::serde_json::Value>,
    pub was_active_before_deprecation: bool,
    pub migrated_flow: Option<::rosetta_uuid::Uuid>,
}
impl web_common_traits::prelude::TableName for DirectusWebhook {
    const TABLE_NAME: &'static str = "directus_webhooks";
}
impl diesel::Identifiable for DirectusWebhook {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusWebhook {
    pub fn migrated_flow<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow>,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        let Some(migrated_flow) = self.migrated_flow else {
            return Ok(None);
        };
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::directus_flows::DirectusFlow::table(),
                migrated_flow,
            ),
            conn,
        )
        .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub fn from_name(
        name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::name.eq(name))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_method(
        method: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::method.eq(method))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_url(
        url: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::url.eq(url))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_status(
        status: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::status.eq(status))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_data(
        data: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::data.eq(data))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_actions(
        actions: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::actions.eq(actions))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_collections(
        collections: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::collections.eq(collections))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_was_active_before_deprecation(
        was_active_before_deprecation: &bool,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(
                directus_webhooks::was_active_before_deprecation.eq(was_active_before_deprecation),
            )
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_migrated_flow(
        migrated_flow: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_webhooks::directus_webhooks;
        Self::table()
            .filter(directus_webhooks::migrated_flow.eq(migrated_flow))
            .order_by(directus_webhooks::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusWebhook> for DirectusWebhook {
    fn as_ref(&self) -> &DirectusWebhook {
        self
    }
}
