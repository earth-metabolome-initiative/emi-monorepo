#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::directus_roles::directus_roles
)]
pub struct DirectusRole {
    pub id: rosetta_uuid::Uuid,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    pub parent: Option<rosetta_uuid::Uuid>,
}
impl DirectusRole {
    #[cfg(feature = "postgres")]
    pub async fn parent(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>,
        diesel::result::Error,
    > {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        let Some(parent) = self.parent.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_roles::DirectusRole::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_roles::directus_roles::dsl::id
                    .eq(parent),
            )
            .first::<crate::codegen::structs_codegen::tables::directus_roles::DirectusRole>(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_parent(
        conn: &mut diesel_async::AsyncPgConnection,
        parent: &crate::codegen::structs_codegen::tables::directus_roles::DirectusRole,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_roles::directus_roles::dsl::parent
                    .eq(parent.id),
            )
            .load::<Self>(conn)
            .await
    }
}
