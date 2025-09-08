impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::FreezingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedure as diesel::Insertable<
            <crate::FreezingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::FreezingProcedure>,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute,
    >,
    crate::FreezingProcedureTemplate: web_common_traits::database::Read<C>,
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
    type Row = crate::FreezingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute,
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
        self.set_most_concrete_table("freezing_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedure = self
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
            let freezing_procedure_templates = crate::FreezingProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::procedure_template_frozen_with_model(
                self,
                freezing_procedure_templates.procedure_template_frozen_with_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::procedure_template_frozen_container_model(
                self,
                freezing_procedure_templates.procedure_template_frozen_container_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_frozen_container,
        ) = self.procedure_frozen_container
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_frozen_container,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::frozen_container(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::frozen_container_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::procedure_template_frozen_container_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_frozen_with) = self
            .procedure_frozen_with
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_frozen_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::frozen_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::frozen_with_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureSettable>::procedure_template_frozen_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let frozen_container = self
            .frozen_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::FrozenContainer,
                ),
            )?;
        let frozen_container_model = self
            .frozen_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::FrozenContainerModel,
                ),
            )?;
        let procedure_template_frozen_container_model = self
            .procedure_template_frozen_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::ProcedureTemplateFrozenContainerModel,
                ),
            )?;
        let frozen_with_model = self
            .frozen_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::FrozenWithModel,
                ),
            )?;
        let procedure_template_frozen_with_model = self
            .procedure_template_frozen_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::ProcedureTemplateFrozenWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_frozen_container = match self.procedure_frozen_container {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_frozen_container,
            ) => {
                procedure_frozen_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_frozen_container,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::ProcedureFrozenContainer,
                        )
                    })?;
                procedure_frozen_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::ProcedureFrozenContainer,
                        )
                    })?
            }
        };
        let procedure_frozen_with = match self.procedure_frozen_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_frozen_with,
            ) => {
                procedure_frozen_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_frozen_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::ProcedureFrozenWith,
                        )
                    })?;
                procedure_frozen_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureAttribute::ProcedureFrozenWith,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            frozen_container,
            frozen_container_model,
            procedure_template_frozen_container_model,
            procedure_frozen_container,
            frozen_with: self.frozen_with,
            frozen_with_model,
            procedure_template_frozen_with_model,
            procedure_frozen_with,
        })
    }
}
