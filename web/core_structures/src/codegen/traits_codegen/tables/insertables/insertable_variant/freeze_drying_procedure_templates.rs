impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate,
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
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::freeze_drying_procedure_templates::FreezeDryingProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes,
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
            "freeze_drying_procedure_templates",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplate = self
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
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Kelvin,
                ),
            )?;
        let kelvin_tolerance_percentage = self
            .kelvin_tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::KelvinTolerancePercentage,
                ),
            )?;
        let pascal = self
            .pascal
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Pascal,
                ),
            )?;
        let seconds = self
            .seconds
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Seconds,
                ),
            )?;
        let freeze_dried_with_model = self
            .freeze_dried_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::FreezeDriedWithModel,
                ),
            )?;
        let procedure_template_freeze_dried_with_model = self
            .procedure_template_freeze_dried_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::ProcedureTemplateFreezeDriedWithModel,
                ),
            )?;
        let freeze_dried_container_model = self
            .freeze_dried_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::FreezeDriedContainerModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_freeze_dried_container_model = self
            .procedure_template_freeze_dried_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::ProcedureTemplateFreezeDriedContainerModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            kelvin,
            kelvin_tolerance_percentage,
            pascal,
            seconds,
            freeze_dried_with_model,
            procedure_template_freeze_dried_with_model,
            freeze_dried_container_model,
            foreign_procedure_template,
            procedure_template_freeze_dried_container_model,
        })
    }
}
