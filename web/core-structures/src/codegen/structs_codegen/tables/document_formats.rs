#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(primary_key(id))]
#[diesel(
    table_name = crate::codegen::diesel_codegen::tables::document_formats::document_formats
)]
pub struct DocumentFormat {
    pub extension: String,
    pub mime_type: String,
    pub description: String,
    pub icon: String,
    pub color: String,
    pub id: i16,
}
impl diesel::Identifiable for DocumentFormat {
    type Id = i16;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl DocumentFormat {
    #[cfg(feature = "postgres")]
    pub async fn from_extension(
        extension: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::document_formats::document_formats::extension
                    .eq(extension),
            )
            .first::<Self>(conn)
            .await
            .optional()
    }
}
