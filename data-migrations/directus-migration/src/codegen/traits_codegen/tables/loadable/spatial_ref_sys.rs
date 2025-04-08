#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = i32;
    async fn load(
        srid: &i32,
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy::table()
            .find(srid)
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy::table()
            .load::<Self>(conn)
            .await
    }
}
