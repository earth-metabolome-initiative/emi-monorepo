impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::PackagingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure as diesel::Insertable<
            <crate::PackagingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::PackagingProcedure>,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
    >,
    crate::PackagingProcedureTemplate: web_common_traits::database::Read<C>,
    crate::Procedure: web_common_traits::database::Read<C>,
    crate::Procedure: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::ProcedureAsset: web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::PackagingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("packaging_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingProcedure = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
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
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let Some(procedure_template) = self.procedure_template {
            let packaging_procedure_templates = crate::PackagingProcedureTemplate::read(
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
            let procedure_assets = crate::ProcedureAsset::read(procedure_sample, conn)?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::procedure_template_sample_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::sample(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::sample_model(
                self,
                procedure_assets.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_packaged_with) = self
            .procedure_packaged_with
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_packaged_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::packaged_with_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PackagingProcedureSettable>::procedure_template_packaged_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
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
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::PackagingProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_sample = match self.procedure_sample {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(mut procedure_sample) => {
                procedure_sample = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_sample,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedureSample,
                        )
                    })?;
                procedure_sample
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
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
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PackagingProcedureAttribute::ProcedurePackagedWith,
                        )
                    })?;
                procedure_packaged_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
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
