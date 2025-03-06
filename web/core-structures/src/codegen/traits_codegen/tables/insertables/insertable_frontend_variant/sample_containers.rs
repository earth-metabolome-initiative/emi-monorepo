#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableSampleContainer {
    type Row = crate::codegen::structs_codegen::tables::sample_containers::SampleContainer;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableSampleContainerBuilder;
    type Conn = diesel::PgConnection;
    fn insert(
        self,
        conn: &mut Self::Conn,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            <Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        Ok(
            diesel::insert_into(
                    crate::codegen::structs_codegen::tables::sample_containers::SampleContainer::table(),
                )
                .values(self)
                .get_result(conn)?,
        )
    }
}
