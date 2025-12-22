impl<ProcedureTemplate> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type Row = crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute,
    >;
}
impl<ProcedureTemplate> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplate;
}
#[cfg(feature = "backend")]
impl<ProcedureTemplate> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplate,
        Row = crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("ball_mill_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplate = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate,
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
    Self: crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute,
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
            procedure_template_bead_model_id,
        ) = self.procedure_template_bead_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_bead_model_id,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateSettable>::bead_model(
                self,
                procedure_template_asset_models.asset_model_id,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_milled_with_model_id,
        ) = self.procedure_template_milled_with_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_milled_with_model_id,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateSettable>::milled_with_model(
                self,
                procedure_template_asset_models.asset_model_id,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_milled_container_model_id,
        ) = self.procedure_template_milled_container_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_milled_container_model_id,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateSettable>::milled_container_model(
                self,
                procedure_template_asset_models.asset_model_id,
            )?;
        }
        let kelvin = self
            .kelvin
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Kelvin,
                ),
            )?;
        let kelvin_tolerance_percentage = self
            .kelvin_tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::KelvinTolerancePercentage,
                ),
            )?;
        let seconds = self
            .seconds
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Seconds,
                ),
            )?;
        let hertz = self
            .hertz
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::Hertz,
                ),
            )?;
        let bead_model_id = self
            .bead_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::BeadModel,
                ),
            )?;
        let number_of_beads = self
            .number_of_beads
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::NumberOfBeads,
                ),
            )?;
        let milled_with_model_id = self
            .milled_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::MilledWithModel,
                ),
            )?;
        let milled_container_model_id = self
            .milled_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::MilledContainerModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_template_bead_model_id = match self.procedure_template_bead_model_id {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_bead_model_id,
            ) => {
                procedure_template_bead_model_id = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_bead_model_id,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::ProcedureTemplateBeadModel,
                        )
                    })?;
                procedure_template_bead_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::ProcedureTemplateBeadModel,
                        )
                    })?
            }
        };
        let procedure_template_milled_with_model_id = match self
            .procedure_template_milled_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_milled_with_model_id,
            ) => {
                procedure_template_milled_with_model_id = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_milled_with_model_id,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::ProcedureTemplateMilledWithModel,
                        )
                    })?;
                procedure_template_milled_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::ProcedureTemplateMilledWithModel,
                        )
                    })?
            }
        };
        let procedure_template_milled_container_model_id = match self
            .procedure_template_milled_container_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_milled_container_model_id,
            ) => {
                procedure_template_milled_container_model_id = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_milled_container_model_id,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::ProcedureTemplateMilledContainerModel,
                        )
                    })?;
                procedure_template_milled_container_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.replace_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureTemplateAttribute::ProcedureTemplateMilledContainerModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            kelvin,
            kelvin_tolerance_percentage,
            seconds,
            hertz,
            bead_model_id,
            procedure_template_bead_model_id,
            number_of_beads,
            milled_with_model_id,
            procedure_template_milled_with_model_id,
            milled_container_model_id,
            procedure_template_milled_container_model_id,
        })
    }
}
