impl<
    C: diesel::connection::LoadConnection,
    PhysicalAsset,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSampleBuilder<
    PhysicalAsset,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::samples::Sample as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSample as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::samples::Sample as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::samples::Sample,
    >,
    C: diesel::connection::LoadConnection,
    PhysicalAsset: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::SampleSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::SampleAttribute,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_models::SampleModel: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::SampleAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        PhysicalAsset,
        EffectiveExtensionAttribute = <PhysicalAsset as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::samples::Sample;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSample;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SampleAttribute,
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
        self.set_most_concrete_table("samples");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSample = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::Read;
        if let Some(model) = self.model {
            let sample_models = crate::codegen::structs_codegen::tables::sample_models::SampleModel::read(
                model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SampleSettable>::sample_source_model(
                self,
                sample_models.sample_source_model,
            )?;
        }
        if let Some(sample_source) = self.sample_source {
            let assets = crate::codegen::structs_codegen::tables::assets::Asset::read(
                sample_source,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SampleSettable>::sample_source_model(
                self,
                assets.model,
            )?;
        }
        let model = self
            .model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SampleAttribute::Model,
                ),
            )?;
        let sample_source = self
            .sample_source
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SampleAttribute::SampleSource,
                ),
            )?;
        let sample_source_model = self
            .sample_source_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SampleAttribute::SampleSourceModel,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    <crate::codegen::structs_codegen::tables::insertables::SampleAttribute as web_common_traits::database::FromExtensionAttribute<
                        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
                        PhysicalAsset,
                    >>::from_extension_attribute(attribute)
                })
            })?;
        Ok(Self::InsertableVariant {
            id,
            model,
            sample_source,
            sample_source_model,
        })
    }
}
