impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute,
    >,
    crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate: web_common_traits::database::Read<
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
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute,
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
        self.set_most_concrete_table("capping_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedure = self
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
            let capping_procedure_templates = crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable>::procedure_template_capped_with_model(
                self,
                capping_procedure_templates.procedure_template_capped_with_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable>::procedure_template_capped_container_model(
                self,
                capping_procedure_templates.procedure_template_capped_container_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_capped_container,
        ) = self.procedure_capped_container
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_capped_container,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable>::capped_container(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable>::capped_container_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable>::procedure_template_capped_container_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_capped_with) = self
            .procedure_capped_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_capped_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable>::capped_with_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CappingProcedureSettable>::procedure_template_capped_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let capped_container = self
            .capped_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::CappedContainer,
                ),
            )?;
        let capped_container_model = self
            .capped_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::CappedContainerModel,
                ),
            )?;
        let procedure_template_capped_container_model = self
            .procedure_template_capped_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::ProcedureTemplateCappedContainerModel,
                ),
            )?;
        let capped_with_model = self
            .capped_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::CappedWithModel,
                ),
            )?;
        let procedure_template_capped_with_model = self
            .procedure_template_capped_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::ProcedureTemplateCappedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::CappingProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_capped_container = match self.procedure_capped_container {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_capped_container,
            ) => {
                procedure_capped_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_capped_container,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::ProcedureCappedContainer,
                        )
                    })?;
                procedure_capped_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::ProcedureCappedContainer,
                        )
                    })?
            }
        };
        let procedure_capped_with = match self.procedure_capped_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_capped_with,
            ) => {
                procedure_capped_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_capped_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::ProcedureCappedWith,
                        )
                    })?;
                procedure_capped_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CappingProcedureAttribute::ProcedureCappedWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            capped_container,
            capped_container_model,
            procedure_template_capped_container_model,
            procedure_capped_container,
            capped_with_model,
            procedure_template_capped_with_model,
            procedure_capped_with,
        })
    }
}
