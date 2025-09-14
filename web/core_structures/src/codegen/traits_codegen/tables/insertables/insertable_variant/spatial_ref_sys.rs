impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder
{
    type Row = crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy,
    >,
{
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpatialRefSyAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<
        Self::InsertableVariant,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpatialRefSyAttribute,
        >,
    > {
        let srid = self
            .srid
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SpatialRefSyAttribute::Srid,
                ),
            )?;
        Ok(Self::InsertableVariant {
            srid,
            auth_name: self.auth_name,
            auth_srid: self.auth_srid,
            srtext: self.srtext,
            proj4text: self.proj4text,
        })
    }
}
