#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::team_members::TeamMember
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = (i32, i32);
    async fn load(
        (team_id, member_id): &(i32, i32),
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::team_members::TeamMember::table()
            .find((team_id, member_id))
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::team_members::TeamMember::table()
            .load::<Self>(conn)
            .await
    }
}
