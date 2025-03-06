#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = (uuid::Uuid, i32);
    async fn load(
        (sample_id, taxon_id): &(uuid::Uuid, i32),
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa::table()
            .find((sample_id, taxon_id))
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::sample_taxa::SampleTaxa::table()
            .load::<Self>(conn)
            .await
    }
}
