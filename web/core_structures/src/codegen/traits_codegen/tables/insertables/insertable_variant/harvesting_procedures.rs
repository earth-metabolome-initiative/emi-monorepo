impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder<
        Procedure,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedure,
        Row = crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("harvesting_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedure = self
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
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableHarvestingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <Procedure as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
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
        if let Some(procedure_template) = self.procedure_template {
            let harvesting_procedure_templates = crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable>::procedure_template_sample_model(
                self,
                harvesting_procedure_templates.procedure_template_sample_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable>::procedure_template_sample_source_model(
                self,
                harvesting_procedure_templates.procedure_template_sample_source_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_sample_source) = self
            .procedure_sample_source
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_sample_source,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable>::procedure_template_sample_source_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable>::sample_source(
                    self,
                    asset,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_sample) = self
            .procedure_sample
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_sample,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable>::procedure_template_sample_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureSettable>::sample(
                    self,
                    asset,
                )?;
            }
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let sample_source = self
            .sample_source
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::SampleSource,
                ),
            )?;
        let procedure_template_sample_source_model = self
            .procedure_template_sample_source_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::ProcedureTemplateSampleSourceModel,
                ),
            )?;
        let sample = self
            .sample
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::Sample,
                ),
            )?;
        let procedure_template_sample_model = self
            .procedure_template_sample_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::ProcedureTemplateSampleModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_sample_source = match self.procedure_sample_source {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_sample_source,
            ) => {
                procedure_sample_source = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_sample_source,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::ProcedureSampleSource,
                        )
                    })?;
                procedure_sample_source
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::ProcedureSampleSource,
                        )
                    })?
            }
        };
        let procedure_sample = match self.procedure_sample {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(mut procedure_sample) => {
                procedure_sample = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_sample,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::ProcedureSample,
                        )
                    })?;
                procedure_sample
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::HarvestingProcedureAttribute::ProcedureSample,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            sample_source,
            procedure_template_sample_source_model,
            procedure_sample_source,
            sample,
            procedure_template_sample_model,
            procedure_sample,
        })
    }
}
