#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganism
{
    type Row = crate::codegen::structs_codegen::tables::organisms::Organism;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder;
    type Conn = diesel_async::AsyncPgConnection;
    async fn insert(
        self,
        conn: &mut Self::Conn,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            <Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute,
        >,
    > {
        use diesel::associations::HasTable;
        use diesel_async::RunQueryDsl;
        Ok(diesel::insert_into(
            crate::codegen::structs_codegen::tables::organisms::Organism::table(),
        )
        .values(self)
        .get_result(conn)
        .await?)
    }
}
