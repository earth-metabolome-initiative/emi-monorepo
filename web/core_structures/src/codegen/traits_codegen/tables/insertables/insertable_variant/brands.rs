impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableBrandBuilder
{
    type Row = crate::codegen::structs_codegen::tables::brands::Brand;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::BrandAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableBrandBuilder
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBrand;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableBrandBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableBrandBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableBrand as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::brands::Brand,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBrand,
        Row = crate::codegen::structs_codegen::tables::brands::Brand,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BrandAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableBrand = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableBrandBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableBrand as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::brands::Brand as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::brands::Brand,
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
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::Name,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::CreatedAt,
                ),
            )?;
        let updated_by = self
            .updated_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::UpdatedBy,
                ),
            )?;
        let updated_at = self
            .updated_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BrandAttribute::UpdatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            name,
            created_by,
            created_at,
            updated_by,
            updated_at,
        })
    }
}
