#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = i16;
    async fn load(id: &i16, conn: &mut Self::Conn) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory::table()
            .find(id)
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::nameplate_categories::NameplateCategory::table()
            .load::<Self>(conn)
            .await
    }
}
