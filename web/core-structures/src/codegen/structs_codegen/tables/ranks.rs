#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::ranks::ranks)]
pub struct Rank {
    pub name: String,
    pub description: String,
    pub id: i16,
}
impl Rank {
    #[cfg(feature = "postgres")]
    pub async fn from_name(
        name: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::ranks::ranks::name,
                name,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
    #[cfg(feature = "postgres")]
    pub async fn from_description(
        description: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::ExpressionMethods::eq(
                crate::codegen::diesel_codegen::tables::ranks::ranks::description,
                description,
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
