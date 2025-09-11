impl<
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModelBuilder<
    PhysicalAssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel,
    >,
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
        PhysicalAssetModel,
        EffectiveExtensionAttribute = <PhysicalAssetModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::sample_source_models::SampleSourceModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute,
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
        self.set_most_concrete_table("sample_source_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSampleSourceModel = self
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
                    <crate::codegen::structs_codegen::tables::insertables::SampleSourceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
                        PhysicalAssetModel,
                    >>::from_extension_attribute(attribute)
                })
            })?;
        Ok(Self::InsertableVariant { id })
    }
}
