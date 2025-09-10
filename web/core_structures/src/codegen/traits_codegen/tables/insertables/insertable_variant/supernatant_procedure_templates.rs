impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute,
    >,
    crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute,
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
        self.set_most_concrete_table("supernatant_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureTemplate = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .supernatant_pm_compatibility_rules(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .procedure_template_pipette_tip_model(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .procedure_template_supernatant_destination_model(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .procedure_template_transferred_with_model(conn)?
            .can_update(user_id, conn)?
        {
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
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_stratified_source_model,
        ) = self.procedure_template_stratified_source_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_stratified_source_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateSettable>::stratified_source_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_supernatant_destination_model,
        ) = self.procedure_template_supernatant_destination_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_supernatant_destination_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateSettable>::supernatant_destination_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_transferred_with_model,
        ) = self.procedure_template_transferred_with_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_transferred_with_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateSettable>::transferred_with_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_pipette_tip_model,
        ) = self.procedure_template_pipette_tip_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_pipette_tip_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateSettable>::pipette_tip_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let liters = self
            .liters
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::Liters,
                ),
            )?;
        let stratified_source_model = self
            .stratified_source_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::StratifiedSourceModel,
                ),
            )?;
        let supernatant_destination_model = self
            .supernatant_destination_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::SupernatantDestinationModel,
                ),
            )?;
        let transferred_with_model = self
            .transferred_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::TransferredWithModel,
                ),
            )?;
        let pipette_tip_model = self
            .pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::PipetteTipModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateExtensionAttribute::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::ProcedureTemplate,
                    ),
                ))
            })?;
        let procedure_template_stratified_source_model = match self
            .procedure_template_stratified_source_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_stratified_source_model,
            ) => {
                procedure_template_stratified_source_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_stratified_source_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplateStratifiedSourceModel,
                        )
                    })?;
                procedure_template_stratified_source_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplateStratifiedSourceModel,
                        )
                    })?
            }
        };
        let procedure_template_supernatant_destination_model = match self
            .procedure_template_supernatant_destination_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_supernatant_destination_model,
            ) => {
                procedure_template_supernatant_destination_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_supernatant_destination_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplateSupernatantDestinationModel,
                        )
                    })?;
                procedure_template_supernatant_destination_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplateSupernatantDestinationModel,
                        )
                    })?
            }
        };
        let procedure_template_transferred_with_model = match self
            .procedure_template_transferred_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_transferred_with_model,
            ) => {
                procedure_template_transferred_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_transferred_with_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplateTransferredWithModel,
                        )
                    })?;
                procedure_template_transferred_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplateTransferredWithModel,
                        )
                    })?
            }
        };
        let procedure_template_pipette_tip_model = match self
            .procedure_template_pipette_tip_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_pipette_tip_model,
            ) => {
                procedure_template_pipette_tip_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_pipette_tip_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplatePipetteTipModel,
                        )
                    })?;
                procedure_template_pipette_tip_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureTemplateAttribute::ProcedureTemplatePipetteTipModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            liters,
            stratified_source_model,
            procedure_template_stratified_source_model,
            supernatant_destination_model,
            procedure_template_supernatant_destination_model,
            transferred_with_model,
            procedure_template_transferred_with_model,
            pipette_tip_model,
            procedure_template_pipette_tip_model,
        })
    }
}
