impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
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
        self.set_most_concrete_table("disposal_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.procedure_template(conn)?.can_update(user_id, conn)? {
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
            if let Some(disposal_procedure_templates) = crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate::read(
                procedure_template,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable>::procedure_template_disposed_asset_model(
                    self,
                    disposal_procedure_templates.procedure_template_disposed_asset_model,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            Some(procedure_disposed_asset),
        ) = self.procedure_disposed_asset
        {
            if let Some(procedure_assets) = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_disposed_asset,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable>::disposed_asset(
                    self,
                    procedure_assets.asset,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable>::procedure_template_disposed_asset_model(
                    self,
                    procedure_assets.procedure_template_asset_model,
                )?;
            }
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let procedure_template_disposed_asset_model = self
            .procedure_template_disposed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureTemplateDisposedAssetModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::DisposalProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_disposed_asset = match self.procedure_disposed_asset {
            web_common_traits::database::IdOrBuilder::Id(id) => {
                id.mint_primary_key(user_id, conn)
                    .map_err(|_| {
                        common_traits::prelude::BuilderError::IncompleteBuild(
                            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureDisposedAsset(
                                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
                            ),
                        )
                    })?
            }
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_disposed_asset,
            ) => {
                procedure_disposed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_disposed_asset,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureDisposedAsset,
                        )
                    })?;
                procedure_disposed_asset
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureDisposedAsset,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            disposed_asset: self.disposed_asset,
            procedure_template_disposed_asset_model,
            procedure_disposed_asset,
        })
    }
}
