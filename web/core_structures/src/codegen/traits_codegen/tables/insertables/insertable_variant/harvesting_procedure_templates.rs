impl<ProcedureTemplate> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type Row = crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute,
    >;
}
impl<ProcedureTemplate> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplate;
}
#[cfg(feature = "backend")]
impl<ProcedureTemplate> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplate,
        Row = crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("harvesting_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplate = self
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
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <ProcedureTemplate as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::sample_models::SampleModel: web_common_traits::database::Read<
        C,
    >,
{
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_sample_source_model,
        ) = self.procedure_template_sample_source_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_sample_source_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateSettable>::sample_source_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let Some(sample_model) = self.sample_model {
            let sample_models = crate::codegen::structs_codegen::tables::sample_models::SampleModel::read(
                sample_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateSettable>::sample_source_model(
                self,
                sample_models.sample_source_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_sample_model,
        ) = self.procedure_template_sample_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_sample_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateSettable>::sample_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let sample_source_model = self
            .sample_source_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute::SampleSourceModel,
                ),
            )?;
        let sample_model = self
            .sample_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute::SampleModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_template_sample_source_model = match self
            .procedure_template_sample_source_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_sample_source_model,
            ) => {
                procedure_template_sample_source_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_sample_source_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute::ProcedureTemplateSampleSourceModel,
                        )
                    })?;
                procedure_template_sample_source_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute::ProcedureTemplateSampleSourceModel,
                        )
                    })?
            }
        };
        let procedure_template_sample_model = match self.procedure_template_sample_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_sample_model,
            ) => {
                procedure_template_sample_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_sample_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute::ProcedureTemplateSampleModel,
                        )
                    })?;
                procedure_template_sample_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureTemplateAttribute::ProcedureTemplateSampleModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            sample_source_model,
            procedure_template_sample_source_model,
            sample_model,
            procedure_template_sample_model,
        })
    }
}
