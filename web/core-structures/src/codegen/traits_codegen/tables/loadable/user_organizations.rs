#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::user_organizations::UserOrganization
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = (i32, i16);
    async fn load(
        (user_id, organization_id): &(i32, i16),
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{associations::HasTable, OptionalExtension, QueryDsl};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::user_organizations::UserOrganization::table()
            .find((user_id, organization_id))
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::user_organizations::UserOrganization::table()
            .load::<Self>(conn)
            .await
    }
}
