impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder
{
    type Row = crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SpatialRefSyAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSyBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy,
        Row = crate::codegen::structs_codegen::tables::spatial_ref_sys::SpatialRefSy,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SpatialRefSyAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSpatialRefSy = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
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
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
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
