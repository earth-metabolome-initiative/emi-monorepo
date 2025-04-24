#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::countries::countries)]
pub struct Country {
    pub iso: String,
    pub emoji: String,
    pub unicode: String,
    pub name: String,
    pub id: i16,
}
impl Country {
    #[cfg(feature = "postgres")]
    pub async fn from_iso(
        iso: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::countries::countries::iso,
                iso,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_emoji(
        emoji: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::countries::countries::emoji,
                emoji,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_unicode(
        unicode: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::countries::countries::unicode,
                unicode,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::countries::countries::name,
                name,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
