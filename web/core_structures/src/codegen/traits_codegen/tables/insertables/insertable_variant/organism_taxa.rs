impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon,
    >,
{
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon = self
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
            crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute,
        >,
    > {
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::CreatedAt,
                ),
            )?;
        let organism_id = self
            .organism_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::OrganismId,
                ),
            )?;
        let taxon_id = self
            .taxon_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::TaxonId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            created_by,
            created_at,
            organism_id,
            taxon_id,
        })
    }
}
