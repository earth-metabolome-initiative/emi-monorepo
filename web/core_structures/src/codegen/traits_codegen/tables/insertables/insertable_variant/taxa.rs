impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
{
    type Row = crate::codegen::structs_codegen::tables::taxa::Taxon;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::TaxonAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTaxon;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTaxon as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::taxa::Taxon,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTaxon,
        Row = crate::codegen::structs_codegen::tables::taxa::Taxon,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::TaxonAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableTaxon = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableTaxonBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableTaxon as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::taxa::Taxon as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::taxa::Taxon,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let id = self
            .id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TaxonAttribute::Id,
                ),
            )?;
        let name = self
            .name
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TaxonAttribute::Name,
                ),
            )?;
        let rank_id = self
            .rank_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::TaxonAttribute::RankId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            id,
            name,
            parent_id: self.parent_id,
            rank_id,
        })
    }
}
