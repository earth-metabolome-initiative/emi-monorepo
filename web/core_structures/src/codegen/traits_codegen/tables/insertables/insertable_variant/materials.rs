impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableMaterialBuilder
{
    type Row = crate::codegen::structs_codegen::tables::materials::Material;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::MaterialAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableMaterialBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableMaterial;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableMaterialBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableMaterialBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::materials::Material as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableMaterial as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::materials::Material as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::materials::Material,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableMaterial,
        Row = crate::codegen::structs_codegen::tables::materials::Material,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::MaterialAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableMaterial = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableMaterialBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::materials::Material as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableMaterial as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::materials::Material as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::materials::Material,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::MaterialAttribute::Name,
                ),
            )?;
        let description = self
            .description
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::MaterialAttribute::Description,
                ),
            )?;
        let icon = self
            .icon
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::MaterialAttribute::Icon,
                ),
            )?;
        let color_id = self
            .color_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::MaterialAttribute::ColorId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            name,
            description,
            icon,
            color_id,
        })
    }
}
