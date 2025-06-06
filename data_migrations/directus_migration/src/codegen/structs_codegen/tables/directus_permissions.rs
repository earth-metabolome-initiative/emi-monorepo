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
    table_name = crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions
)]
pub struct DirectusPermission {
    pub id: i32,
    pub collection: String,
    pub action: String,
    pub permissions: Option<::serde_json::Value>,
    pub validation: Option<::serde_json::Value>,
    pub presets: Option<::serde_json::Value>,
    pub fields: Option<String>,
    pub policy: ::rosetta_uuid::Uuid,
}
impl web_common_traits::prelude::TableName for DirectusPermission {
    const TABLE_NAME: &'static str = "directus_permissions";
}
impl diesel::Identifiable for DirectusPermission {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DirectusPermission {
    pub fn policy<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::directus_policies::DirectusPolicy::table(),
                self.policy,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_collection(
        collection: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions;
        Self::table()
            .filter(directus_permissions::collection.eq(collection))
            .order_by(directus_permissions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_action(
        action: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions;
        Self::table()
            .filter(directus_permissions::action.eq(action))
            .order_by(directus_permissions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_fields(
        fields: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions;
        Self::table()
            .filter(directus_permissions::fields.eq(fields))
            .order_by(directus_permissions::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_policy(
        policy: &::rosetta_uuid::Uuid,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::directus_permissions::directus_permissions;
        Self::table()
            .filter(directus_permissions::policy.eq(policy))
            .order_by(directus_permissions::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<DirectusPermission> for DirectusPermission {
    fn as_ref(&self) -> &DirectusPermission {
        self
    }
}
