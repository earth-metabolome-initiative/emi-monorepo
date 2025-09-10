impl<
    C: diesel::connection::LoadConnection,
    SampleSource,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
    SampleSource,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganism as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organisms::Organism,
    >,
    C: diesel::connection::LoadConnection,
    SampleSource: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::organisms::Organism;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableOrganism;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("organisms");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganism = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let model = self
            .model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismAttribute::Model,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::OrganismAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::OrganismExtensionAttribute::SampleSource(
                        crate::codegen::structs_codegen::tables::insertables::SampleSourceAttribute::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            model,
        })
    }
}
