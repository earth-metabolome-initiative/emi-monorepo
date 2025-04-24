#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::document_formats::document_formats
)]
pub struct DocumentFormat {
    pub extension: String,
    pub mime_type: String,
    pub description: String,
    pub icon_id: i16,
    pub color: String,
    pub id: i16,
}
impl DocumentFormat {
    #[cfg(feature = "postgres")]
    pub async fn icon(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::icons::Icon, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::icons::Icon::table()
            .filter(crate::codegen::diesel_codegen::tables::icons::icons::dsl::id.eq(&self.icon_id))
            .first::<crate::codegen::structs_codegen::tables::icons::Icon>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_icon_id(
        conn: &mut diesel_async::AsyncPgConnection,
        icon_id: &crate::codegen::structs_codegen::tables::icons::Icon,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::document_formats::document_formats::dsl::icon_id
                    .eq(icon_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_extension(
        extension: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::document_formats::document_formats::extension,
                    extension,
                ),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
