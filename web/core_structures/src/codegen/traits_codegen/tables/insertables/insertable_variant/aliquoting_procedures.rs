impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::AliquotingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedure as diesel::Insertable<
            <crate::AliquotingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::AliquotingProcedure>,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute,
    >,
    crate::AliquotingProcedureTemplate: web_common_traits::database::Read<C>,
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
    type Row = crate::AliquotingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute,
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
        self.set_most_concrete_table("aliquoting_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedure = self
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
            let aliquoting_procedure_templates = crate::AliquotingProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_pipette_tip_model(
                self,
                aliquoting_procedure_templates.procedure_template_pipette_tip_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_aliquoted_from_model(
                self,
                aliquoting_procedure_templates.procedure_template_aliquoted_from_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_aliquoted_into_model(
                self,
                aliquoting_procedure_templates.procedure_template_aliquoted_into_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_aliquoted_with_model(
                self,
                aliquoting_procedure_templates.procedure_template_aliquoted_with_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_aliquoted_with) = self
            .procedure_aliquoted_with
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_aliquoted_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::aliquoted_with(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::aliquoted_with_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_aliquoted_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_pipette_tip) = self
            .procedure_pipette_tip
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_pipette_tip,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::pipette_tip_model(
                self,
                procedure_assets.asset_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_pipette_tip_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_aliquoted_from) = self
            .procedure_aliquoted_from
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_aliquoted_from,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::aliquoted_from(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_aliquoted_from_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_aliquoted_into) = self
            .procedure_aliquoted_into
        {
            let procedure_assets = crate::ProcedureAsset::read(
                procedure_aliquoted_into,
                conn,
            )?;
            if let Some(asset) = procedure_assets.asset {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::aliquoted_into(
                    self,
                    asset,
                )?;
            }
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureSettable>::procedure_template_aliquoted_into_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let aliquoted_with_model = self
            .aliquoted_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::AliquotedWithModel,
                ),
            )?;
        let procedure_template_aliquoted_with_model = self
            .procedure_template_aliquoted_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureTemplateAliquotedWithModel,
                ),
            )?;
        let pipette_tip_model = self
            .pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::PipetteTipModel,
                ),
            )?;
        let procedure_template_pipette_tip_model = self
            .procedure_template_pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureTemplatePipetteTipModel,
                ),
            )?;
        let aliquoted_from = self
            .aliquoted_from
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::AliquotedFrom,
                ),
            )?;
        let procedure_template_aliquoted_from_model = self
            .procedure_template_aliquoted_from_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureTemplateAliquotedFromModel,
                ),
            )?;
        let aliquoted_into = self
            .aliquoted_into
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::AliquotedInto,
                ),
            )?;
        let procedure_template_aliquoted_into_model = self
            .procedure_template_aliquoted_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureTemplateAliquotedIntoModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_aliquoted_with = match self.procedure_aliquoted_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_aliquoted_with,
            ) => {
                procedure_aliquoted_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_aliquoted_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureAliquotedWith,
                        )
                    })?;
                procedure_aliquoted_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureAliquotedWith,
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
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedurePipetteTip,
                        )
                    })?;
                procedure_pipette_tip
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedurePipetteTip,
                        )
                    })?
            }
        };
        let procedure_aliquoted_from = match self.procedure_aliquoted_from {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_aliquoted_from,
            ) => {
                procedure_aliquoted_from = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_aliquoted_from,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureAliquotedFrom,
                        )
                    })?;
                procedure_aliquoted_from
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureAliquotedFrom,
                        )
                    })?
            }
        };
        let procedure_aliquoted_into = match self.procedure_aliquoted_into {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_aliquoted_into,
            ) => {
                procedure_aliquoted_into = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_aliquoted_into,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureAliquotedInto,
                        )
                    })?;
                procedure_aliquoted_into
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureAttribute::ProcedureAliquotedInto,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            aliquoted_with: self.aliquoted_with,
            aliquoted_with_model,
            procedure_template_aliquoted_with_model,
            procedure_aliquoted_with,
            pipette_tip_model,
            procedure_template_pipette_tip_model,
            procedure_pipette_tip,
            aliquoted_from,
            procedure_template_aliquoted_from_model,
            procedure_aliquoted_from,
            aliquoted_into,
            procedure_template_aliquoted_into_model,
            procedure_aliquoted_into,
        })
    }
}
