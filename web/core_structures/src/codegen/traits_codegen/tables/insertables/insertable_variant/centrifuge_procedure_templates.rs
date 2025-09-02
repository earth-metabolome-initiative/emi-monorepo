impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable>::most_concrete_table(
            self,
            "centrifuge_procedure_templates",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let kelvin = self
            .kelvin
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::Kelvin,
                ),
            )?;
        let kelvin_tolerance_percentage = self
            .kelvin_tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::KelvinTolerancePercentage,
                ),
            )?;
        let seconds = self
            .seconds
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::Seconds,
                ),
            )?;
        let rotation_per_minute = self
            .rotation_per_minute
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::RotationPerMinute,
                ),
            )?;
        let centrifuged_with_model = self
            .centrifuged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::CentrifugedWithModel,
                ),
            )?;
        let procedure_template_centrifuged_with_model = self
            .procedure_template_centrifuged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::ProcedureTemplateCentrifugedWithModel,
                ),
            )?;
        let centrifuged_container_model = self
            .centrifuged_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::CentrifugedContainerModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_centrifuged_container_model = self
            .procedure_template_centrifuged_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::ProcedureTemplateCentrifugedContainerModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            kelvin,
            kelvin_tolerance_percentage,
            seconds,
            rotation_per_minute,
            centrifuged_with_model,
            procedure_template_centrifuged_with_model,
            centrifuged_container_model,
            foreign_procedure_template,
            procedure_template_centrifuged_container_model,
        })
    }
}
