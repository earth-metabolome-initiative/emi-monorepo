#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Loadable
for crate::codegen::structs_codegen::tables::directus_migrations::DirectusMigration {
    type Conn = diesel_async::AsyncPgConnection;
    type PrimaryKey = String;
    async fn load(
        version: &String,
        conn: &mut Self::Conn,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        use diesel::{QueryDsl, OptionalExtension};
        crate::codegen::structs_codegen::tables::directus_migrations::DirectusMigration::table()
            .find(version)
            .first::<Self>(conn)
            .await
            .optional()
    }
    async fn load_all(
        conn: &mut Self::Conn,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel_async::RunQueryDsl;
        use diesel::associations::HasTable;
        crate::codegen::structs_codegen::tables::directus_migrations::DirectusMigration::table()
            .load::<Self>(conn)
            .await
    }
}
