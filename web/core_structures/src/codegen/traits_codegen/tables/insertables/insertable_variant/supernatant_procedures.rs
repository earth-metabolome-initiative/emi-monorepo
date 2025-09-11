impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
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
    crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        Procedure,
        EffectiveExtensionAttribute = <Procedure as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute,
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
        self.set_most_concrete_table("supernatant_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableSupernatantProcedure = self
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
            let supernatant_procedure_templates = crate::codegen::structs_codegen::tables::supernatant_procedure_templates::SupernatantProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_supernatant_destination_model(
                self,
                supernatant_procedure_templates
                    .procedure_template_supernatant_destination_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_transferred_with_model(
                self,
                supernatant_procedure_templates.procedure_template_transferred_with_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_pipette_tip_model(
                self,
                supernatant_procedure_templates.procedure_template_pipette_tip_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_stratified_source_model(
                self,
                supernatant_procedure_templates
                    .procedure_template_stratified_source_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_stratified_source,
        ) = self.procedure_stratified_source
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_stratified_source,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_stratified_source_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::stratified_source(
                    self,
                    asset,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_supernatant_destination,
        ) = self.procedure_supernatant_destination
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_supernatant_destination,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_supernatant_destination_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::supernatant_destination(
                    self,
                    asset,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_transferred_with,
        ) = self.procedure_transferred_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_transferred_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_transferred_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::transferred_with_model(
                self,
                procedure_assets.asset_model,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::transferred_with(
                    self,
                    asset,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_pipette_tip) = self
            .procedure_pipette_tip
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_pipette_tip,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::pipette_tip_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::pipette_tip_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureSettable>::procedure_template_pipette_tip_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let stratified_source = self
            .stratified_source
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::StratifiedSource,
                ),
            )?;
        let procedure_template_stratified_source_model = self
            .procedure_template_stratified_source_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureTemplateStratifiedSourceModel,
                ),
            )?;
        let supernatant_destination = self
            .supernatant_destination
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::SupernatantDestination,
                ),
            )?;
        let procedure_template_supernatant_destination_model = self
            .procedure_template_supernatant_destination_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureTemplateSupernatantDestinationModel,
                ),
            )?;
        let transferred_with = self
            .transferred_with
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::TransferredWith,
                ),
            )?;
        let transferred_with_model = self
            .transferred_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::TransferredWithModel,
                ),
            )?;
        let procedure_template_transferred_with_model = self
            .procedure_template_transferred_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureTemplateTransferredWithModel,
                ),
            )?;
        let pipette_tip_model = self
            .pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::PipetteTipModel,
                ),
            )?;
        let procedure_template_pipette_tip_model = self
            .procedure_template_pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureTemplatePipetteTipModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    <crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute as web_common_traits::database::FromExtensionAttribute<
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
                        Procedure,
                    >>::from_extension_attribute(attribute)
                })
            })?;
        let procedure_stratified_source = match self.procedure_stratified_source {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_stratified_source,
            ) => {
                procedure_stratified_source = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_stratified_source,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureStratifiedSource,
                        )
                    })?;
                procedure_stratified_source
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureStratifiedSource,
                        )
                    })?
            }
        };
        let procedure_supernatant_destination = match self
            .procedure_supernatant_destination
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_supernatant_destination,
            ) => {
                procedure_supernatant_destination = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_supernatant_destination,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureSupernatantDestination,
                        )
                    })?;
                procedure_supernatant_destination
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureSupernatantDestination,
                        )
                    })?
            }
        };
        let procedure_transferred_with = match self.procedure_transferred_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_transferred_with,
            ) => {
                procedure_transferred_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_transferred_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureTransferredWith,
                        )
                    })?;
                procedure_transferred_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedureTransferredWith,
                        )
                    })?
            }
        };
        let procedure_pipette_tip = match self.procedure_pipette_tip {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_pipette_tip,
            ) => {
                procedure_pipette_tip = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_pipette_tip,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedurePipetteTip,
                        )
                    })?;
                procedure_pipette_tip
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::SupernatantProcedureAttribute::ProcedurePipetteTip,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            stratified_source,
            procedure_template_stratified_source_model,
            procedure_stratified_source,
            supernatant_destination,
            procedure_template_supernatant_destination_model,
            procedure_supernatant_destination,
            transferred_with,
            transferred_with_model,
            procedure_template_transferred_with_model,
            procedure_transferred_with,
            pipette_tip_model,
            procedure_template_pipette_tip_model,
            procedure_pipette_tip,
        })
    }
}
