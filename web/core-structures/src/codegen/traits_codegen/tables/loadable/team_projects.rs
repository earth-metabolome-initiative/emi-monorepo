#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
    for crate::codegen::structs_codegen::tables::team_projects::TeamProject
{
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = (i32, i32);
    async fn load(
        (team_id, project_id): &(i32, i32),
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::team_projects::TeamProject::table()
            .find((team_id, project_id))
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(conn: &mut Self::Conn) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::team_projects::TeamProject::table()
            .load::<Self>(conn)
            .await
    }
}
