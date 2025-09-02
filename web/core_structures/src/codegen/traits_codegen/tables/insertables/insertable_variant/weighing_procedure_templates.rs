impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate,
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
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::weighing_procedure_templates::WeighingProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes,
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
            "weighing_procedure_templates",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplate = self
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
        let weighed_container_model = self
            .weighed_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes::WeighedContainerModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_weighed_container_model = self
            .procedure_template_weighed_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes::ProcedureTemplateWeighedContainerModel,
                ),
            )?;
        let weighed_with_model = self
            .weighed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes::WeighedWithModel,
                ),
            )?;
        let procedure_template_weighed_with_model = self
            .procedure_template_weighed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes::ProcedureTemplateWeighedWithModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            weighed_container_model,
            foreign_procedure_template,
            procedure_template_weighed_container_model,
            weighed_with_model,
            procedure_template_weighed_with_model,
        })
    }
}
