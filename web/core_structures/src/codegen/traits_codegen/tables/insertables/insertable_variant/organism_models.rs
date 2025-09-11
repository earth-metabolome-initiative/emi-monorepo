impl<
    C: diesel::connection::LoadConnection,
    SampleSourceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder<
    SampleSourceModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organism_models::OrganismModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organism_models::OrganismModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    >,
    C: diesel::connection::LoadConnection,
    SampleSourceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute,
        SampleSourceModel,
        EffectiveExtensionAttribute = <SampleSourceModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::organism_models::OrganismModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute,
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
        self.set_most_concrete_table("organism_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel = self
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
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    <crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute as web_common_traits::database::FromExtensionAttribute<
                        crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute,
                        SampleSourceModel,
                    >>::from_extension_attribute(attribute)
                })
            })?;
        Ok(Self::InsertableVariant { id })
    }
}
