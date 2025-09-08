impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::StorageProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedure as diesel::Insertable<
            <crate::StorageProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::StorageProcedure>,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute,
    >,
    crate::Procedure: web_common_traits::database::Read<C>,
    crate::Procedure: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::ProcedureAsset: web_common_traits::database::Read<C>,
    crate::StorageProcedureTemplate: web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::StorageProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute,
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
        self.set_most_concrete_table("storage_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableStorageProcedure = self
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
            let storage_procedure_templates = crate::StorageProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::procedure_template_stored_into_model(
                self,
                storage_procedure_templates.procedure_template_stored_into_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::procedure_template_stored_asset_model(
                self,
                storage_procedure_templates.procedure_template_stored_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_stored_asset) = self
            .procedure_stored_asset
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_stored_asset,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::procedure_template_stored_asset_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::stored_asset(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::stored_asset_model(
                self,
                procedure_assets.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_stored_into) = self
            .procedure_stored_into
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_stored_into,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::procedure_template_stored_into_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::stored_into(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::StorageProcedureSettable>::stored_into_model(
                self,
                procedure_assets.asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let stored_asset = self
            .stored_asset
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::StoredAsset,
                ),
            )?;
        let stored_asset_model = self
            .stored_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::StoredAssetModel,
                ),
            )?;
        let procedure_template_stored_asset_model = self
            .procedure_template_stored_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::ProcedureTemplateStoredAssetModel,
                ),
            )?;
        let stored_into = self
            .stored_into
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::StoredInto,
                ),
            )?;
        let stored_into_model = self
            .stored_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::StoredIntoModel,
                ),
            )?;
        let procedure_template_stored_into_model = self
            .procedure_template_stored_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::ProcedureTemplateStoredIntoModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::StorageProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_stored_asset = match self.procedure_stored_asset {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_stored_asset,
            ) => {
                procedure_stored_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_stored_asset,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::ProcedureStoredAsset,
                        )
                    })?;
                procedure_stored_asset
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::ProcedureStoredAsset,
                        )
                    })?
            }
        };
        let procedure_stored_into = match self.procedure_stored_into {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_stored_into,
            ) => {
                procedure_stored_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_stored_into,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::ProcedureStoredInto,
                        )
                    })?;
                procedure_stored_into
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::StorageProcedureAttribute::ProcedureStoredInto,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            stored_asset,
            stored_asset_model,
            procedure_template_stored_asset_model,
            procedure_stored_asset,
            stored_into,
            stored_into_model,
            procedure_template_stored_into_model,
            procedure_stored_into,
        })
    }
}
