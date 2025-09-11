impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
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
    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
        ProcedureTemplate,
        EffectiveExtensionAttribute = <ProcedureTemplate as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute,
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
        self.set_most_concrete_table("aliquoting_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureTemplate = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .procedure_template_aliquoted_into_model(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .procedure_template_aliquoted_with_model(conn)?
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
            procedure_template_aliquoted_from_model,
        ) = self.procedure_template_aliquoted_from_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_aliquoted_from_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateSettable>::aliquoted_from_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_aliquoted_into_model,
        ) = self.procedure_template_aliquoted_into_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_aliquoted_into_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateSettable>::aliquoted_into_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_aliquoted_with_model,
        ) = self.procedure_template_aliquoted_with_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_aliquoted_with_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateSettable>::aliquoted_with_model(
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
            self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateSettable>::pipette_tip_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let liters = self
            .liters
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::Liters,
                ),
            )?;
        let aliquoted_from_model = self
            .aliquoted_from_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::AliquotedFromModel,
                ),
            )?;
        let aliquoted_into_model = self
            .aliquoted_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::AliquotedIntoModel,
                ),
            )?;
        let aliquoted_with_model = self
            .aliquoted_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::AliquotedWithModel,
                ),
            )?;
        let pipette_tip_model = self
            .pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::PipetteTipModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    <crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute as web_common_traits::database::FromExtensionAttribute<
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute,
                        ProcedureTemplate,
                    >>::from_extension_attribute(attribute)
                })
            })?;
        let procedure_template_aliquoted_from_model = match self
            .procedure_template_aliquoted_from_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_aliquoted_from_model,
            ) => {
                procedure_template_aliquoted_from_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_aliquoted_from_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplateAliquotedFromModel,
                        )
                    })?;
                procedure_template_aliquoted_from_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplateAliquotedFromModel,
                        )
                    })?
            }
        };
        let procedure_template_aliquoted_into_model = match self
            .procedure_template_aliquoted_into_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_aliquoted_into_model,
            ) => {
                procedure_template_aliquoted_into_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_aliquoted_into_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplateAliquotedIntoModel,
                        )
                    })?;
                procedure_template_aliquoted_into_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplateAliquotedIntoModel,
                        )
                    })?
            }
        };
        let procedure_template_aliquoted_with_model = match self
            .procedure_template_aliquoted_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_aliquoted_with_model,
            ) => {
                procedure_template_aliquoted_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_aliquoted_with_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplateAliquotedWithModel,
                        )
                    })?;
                procedure_template_aliquoted_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplateAliquotedWithModel,
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
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplatePipetteTipModel,
                        )
                    })?;
                procedure_template_pipette_tip_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureTemplateAttribute::ProcedureTemplatePipetteTipModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            liters,
            aliquoted_from_model,
            procedure_template_aliquoted_from_model,
            aliquoted_into_model,
            procedure_template_aliquoted_into_model,
            aliquoted_with_model,
            procedure_template_aliquoted_with_model,
            pipette_tip_model,
            procedure_template_pipette_tip_model,
        })
    }
}
