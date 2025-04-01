#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "32-column-tables",
    derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)
)]
#[cfg_attr(feature = "32-column-tables", diesel(primary_key(collection)))]
#[cfg_attr(
    feature = "32-column-tables",
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::directus_collections::directus_collections
    )
)]
pub struct DirectusCollection {
    pub collection: String,
    pub icon: Option<String>,
    pub note: Option<String>,
    pub display_template: Option<String>,
    pub hidden: bool,
    pub singleton: bool,
    pub translations: Option<serde_json::Value>,
    pub archive_field: Option<String>,
    pub archive_app_filter: bool,
    pub archive_value: Option<String>,
    pub unarchive_value: Option<String>,
    pub sort_field: Option<String>,
    pub accountability: Option<String>,
    pub color: Option<String>,
    pub item_duplication_fields: Option<serde_json::Value>,
    pub sort: Option<i32>,
    pub group: Option<String>,
    pub collapse: String,
    pub preview_url: Option<String>,
    pub versioning: bool,
}
impl DirectusCollection {
    #[cfg(feature = "postgres")]
    pub async fn group(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<
        Option<
            crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
        >,
        diesel::result::Error,
    > {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::QueryDsl;
        let Some(group) = self.group.as_ref() else {
            return Ok(None);
        };
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection::table()
            .find(group)
            .first::<
                crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
            >(conn)
            .await
            .map(Some)
    }
    #[cfg(feature = "postgres")]
    pub async fn from_group(
        conn: &mut diesel_async::AsyncPgConnection,
        group: &crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, ExpressionMethods};
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::directus_collections::directus_collections::dsl::group
                    .eq(&group.collection),
            )
            .load::<Self>(conn)
            .await
    }
}
