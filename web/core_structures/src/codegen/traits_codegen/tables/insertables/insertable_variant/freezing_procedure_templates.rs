impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
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
        self.set_most_concrete_table("freezing_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplate = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure_template(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .procedure_template_frozen_with_model(conn)?
            .can_update(user_id, conn)?
        {
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
            Some(procedure_template_frozen_with_model),
        ) = self.procedure_template_frozen_with_model
        {
            if let Some(procedure_template_asset_models) = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_frozen_with_model,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateSettable>::frozen_with_model(
                    self,
                    procedure_template_asset_models.asset_model,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            Some(procedure_template_frozen_container_model),
        ) = self.procedure_template_frozen_container_model
        {
            if let Some(procedure_template_asset_models) = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_frozen_container_model,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateSettable>::frozen_container_model(
                    self,
                    procedure_template_asset_models.asset_model,
                )?;
            }
        }
        let kelvin = self
            .kelvin
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::Kelvin,
                ),
            )?;
        let kelvin_tolerance_percentage = self
            .kelvin_tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::KelvinTolerancePercentage,
                ),
            )?;
        let frozen_with_model = self
            .frozen_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::FrozenWithModel,
                ),
            )?;
        let frozen_container_model = self
            .frozen_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::FrozenContainerModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateExtensionAttribute::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAttribute::ProcedureTemplate,
                    ),
                ))
            })?;
        let procedure_template_frozen_with_model = match self
            .procedure_template_frozen_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => {
                id.mint_primary_key(user_id, conn)
                    .map_err(|_| {
                        common_traits::prelude::BuilderError::IncompleteBuild(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::ProcedureTemplateFrozenWithModel(
                                crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                            ),
                        )
                    })?
            }
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_frozen_with_model,
            ) => {
                procedure_template_frozen_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_frozen_with_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::ProcedureTemplateFrozenWithModel,
                        )
                    })?;
                procedure_template_frozen_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::ProcedureTemplateFrozenWithModel,
                        )
                    })?
            }
        };
        let procedure_template_frozen_container_model = match self
            .procedure_template_frozen_container_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => {
                id.mint_primary_key(user_id, conn)
                    .map_err(|_| {
                        common_traits::prelude::BuilderError::IncompleteBuild(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::ProcedureTemplateFrozenContainerModel(
                                crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute::Id,
                            ),
                        )
                    })?
            }
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_frozen_container_model,
            ) => {
                procedure_template_frozen_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_frozen_container_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::ProcedureTemplateFrozenContainerModel,
                        )
                    })?;
                procedure_template_frozen_container_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute::ProcedureTemplateFrozenContainerModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            kelvin,
            kelvin_tolerance_percentage,
            seconds: self.seconds,
            frozen_with_model,
            procedure_template_frozen_with_model,
            frozen_container_model,
            procedure_template_frozen_container_model,
        })
    }
}
