impl<PhysicalAssetModel> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder<
        PhysicalAssetModel,
    >
{
    type Row = crate::codegen::structs_codegen::tables::sample_models::SampleModel;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableSampleModel;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSampleModelBuilder<
    PhysicalAssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::sample_models::SampleModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSampleModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::sample_models::SampleModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::sample_models::SampleModel,
    >,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::SampleModelExtensionAttribute: From<
        <PhysicalAssetModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("sample_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSampleModel = self
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
    ) -> Result<
        Self::InsertableVariant,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute,
        >,
    > {
        let sample_source_model = self
            .sample_source_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute::SampleSourceModel,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::SampleModelAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        Ok(Self::InsertableVariant {
            id,
            sample_source_model,
        })
    }
}
