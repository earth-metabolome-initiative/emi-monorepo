impl<ProcedureTemplate> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type Row = crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
    >;
}
impl<ProcedureTemplate> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplate;
}
#[cfg(feature = "backend")]
impl<ProcedureTemplate> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplate,
        Row = crate::codegen::structs_codegen::tables::freezing_procedure_templates::FreezingProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("freezing_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplate = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
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
    Self::Error: web_common_traits::database::FromExtension<
        <ProcedureTemplate as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
{
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_frozen_with_model,
        ) = self.procedure_template_frozen_with_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_frozen_with_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateSettable>::frozen_with_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_frozen_container_model,
        ) = self.procedure_template_frozen_container_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_frozen_container_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::FreezingProcedureTemplateSettable>::frozen_container_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
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
            .map_err(Self::Error::from_extension)?;
        let procedure_template_frozen_with_model = match self
            .procedure_template_frozen_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
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
            web_common_traits::database::IdOrBuilder::Id(id) => id,
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
