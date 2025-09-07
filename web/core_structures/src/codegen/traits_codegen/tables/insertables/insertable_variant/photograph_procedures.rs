impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::PhotographProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure as diesel::Insertable<
            <crate::PhotographProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::PhotographProcedure>,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
    >,
    crate::PhotographProcedureTemplate: web_common_traits::database::Read<C>,
    crate::Procedure: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::Procedure as diesel::Identifiable>::Id,
    >,
    <<crate::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Procedure as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Procedure as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::Procedure,
    >,
    crate::ProcedureAsset: web_common_traits::database::Read<C>,
    crate::ProcedureAsset: web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::PhotographProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute,
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
        self.set_most_concrete_table("photograph_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedure = self
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
            let photograph_procedure_templates = crate::PhotographProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_asset_model(
                self,
                photograph_procedure_templates
                    .procedure_template_photographed_asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_with_model(
                self,
                photograph_procedure_templates.procedure_template_photographed_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_photographed_asset,
        ) = self.procedure_photographed_asset
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_photographed_asset,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::photographed_asset(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_asset_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_photographed_with,
        ) = self.procedure_photographed_with
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_photographed_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::photographed_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureSettable>::procedure_template_photographed_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let photographed_asset = self
            .photographed_asset
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::PhotographedAsset,
                ),
            )?;
        let procedure_template_photographed_asset_model = self
            .procedure_template_photographed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedureTemplatePhotographedAssetModel,
                ),
            )?;
        let procedure_template_photographed_with_model = self
            .procedure_template_photographed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedureTemplatePhotographedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_photographed_asset = match self.procedure_photographed_asset {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_photographed_asset,
            ) => {
                procedure_photographed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_photographed_asset,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedAsset,
                        )
                    })?;
                procedure_photographed_asset
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedAsset,
                        )
                    })?
            }
        };
        let procedure_photographed_with = match self.procedure_photographed_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_photographed_with,
            ) => {
                procedure_photographed_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_photographed_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedWith,
                        )
                    })?;
                procedure_photographed_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureAttribute::ProcedurePhotographedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            photographed_asset,
            procedure_template_photographed_asset_model,
            procedure_photographed_asset,
            photographed_with: self.photographed_with,
            procedure_template_photographed_with_model,
            procedure_photographed_with,
        })
    }
}
