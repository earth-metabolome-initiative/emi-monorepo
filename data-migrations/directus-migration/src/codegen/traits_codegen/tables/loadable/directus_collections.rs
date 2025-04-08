#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = String;
    async fn load(
        collection: &String,
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection::table()
            .find(collection)
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::directus_collections::DirectusCollection::table()
            .load::<Self>(conn)
            .await
    }
}
