impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate,
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
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes,
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
            "capping_procedure_templates",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplate = self
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
        let container_model = self
            .container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes::ContainerModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_container_model = self
            .procedure_template_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes::ProcedureTemplateContainerModel,
                ),
            )?;
        let capped_with_model = self
            .capped_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes::CappedWithModel,
                ),
            )?;
        let procedure_template_capped_with_model = self
            .procedure_template_capped_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes::ProcedureTemplateCappedWithModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCappingProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            container_model,
            foreign_procedure_template,
            procedure_template_container_model,
            capped_with_model,
            procedure_template_capped_with_model,
        })
    }
}
