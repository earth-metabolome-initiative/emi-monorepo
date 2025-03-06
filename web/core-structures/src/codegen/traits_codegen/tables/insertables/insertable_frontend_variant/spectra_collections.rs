#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollection {
    type Row = crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection;
    type InsertableBuilder = crate::codegen::structs_codegen::tables::insertables::InsertableSpectraCollectionBuilder;
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
                    crate::codegen::structs_codegen::tables::spectra_collections::SpectraCollection::table(),
                )
                .values(self)
                .get_result(conn)?,
        )
    }
}
