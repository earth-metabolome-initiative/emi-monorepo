impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure,
        Row = crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("packaging_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure,
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
    Self: crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate: web_common_traits::database::Read<
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
        if let Some(procedure_template_id) = self.procedure_template {
            let packaging_procedure_templates = crate::codegen::structs_codegen::tables::packaging_procedure_templates::PackagingProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::procedure_template_sample_model(
                self,
                packaging_procedure_templates.procedure_template_sample_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::procedure_template_packaged_with_model(
                self,
                packaging_procedure_templates.procedure_template_packaged_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_sample) = self
            .procedure_sample
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_sample,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::procedure_template_sample_model(
                self,
                procedure_assets.procedure_template_asset_model_id,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::sample(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::sample_model(
                self,
                procedure_assets.asset_model_id,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_packaged_with) = self
            .procedure_packaged_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_packaged_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::packaged_with_model(
                self,
                procedure_assets.asset_model_id,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::procedure_template_packaged_with_model(
                self,
                procedure_assets.procedure_template_asset_model_id,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let sample = self
            .sample
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::Sample,
                ),
            )?;
        let sample_model = self
            .sample_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::SampleModel,
                ),
            )?;
        let procedure_template_sample_model = self
            .procedure_template_sample_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedureTemplateSampleModel,
                ),
            )?;
        let packaged_with_model = self
            .packaged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::PackagedWithModel,
                ),
            )?;
        let procedure_template_packaged_with_model = self
            .procedure_template_packaged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedureTemplatePackagedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_sample = match self.procedure_sample {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(mut procedure_sample) => {
                procedure_sample = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_sample,
                        procedure,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedureSample,
                        )
                    })?;
                procedure_sample
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedureSample,
                        )
                    })?
            }
        };
        let procedure_packaged_with = match self.procedure_packaged_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_packaged_with,
            ) => {
                procedure_packaged_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_packaged_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedurePackagedWith,
                        )
                    })?;
                procedure_packaged_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedurePackagedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            sample,
            sample_model,
            procedure_template_sample_model,
            procedure_sample,
            packaged_with_model,
            procedure_template_packaged_with_model,
            procedure_packaged_with,
        })
    }
}
