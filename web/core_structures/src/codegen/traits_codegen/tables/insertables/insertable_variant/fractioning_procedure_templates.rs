impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate,
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
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::fractioning_procedure_templates::FractioningProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes,
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
            "fractioning_procedure_templates",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplate = self
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
        let kilograms = self
            .kilograms
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::Kilograms,
                ),
            )?;
        let tolerance_percentage = self
            .tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::TolerancePercentage,
                ),
            )?;
        let weighed_with_model = self
            .weighed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::WeighedWithModel,
                ),
            )?;
        let procedure_template_weighed_with_model = self
            .procedure_template_weighed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::ProcedureTemplateWeighedWithModel,
                ),
            )?;
        let fragment_container_model = self
            .fragment_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::FragmentContainerModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_fragment_container_model = self
            .procedure_template_fragment_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::ProcedureTemplateFragmentContainerModel,
                ),
            )?;
        let fragment_placed_into_model = self
            .fragment_placed_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::FragmentPlacedIntoModel,
                ),
            )?;
        let procedure_template_fragment_placed_into_model = self
            .procedure_template_fragment_placed_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::ProcedureTemplateFragmentPlacedIntoModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            kilograms,
            tolerance_percentage,
            weighed_with_model,
            procedure_template_weighed_with_model,
            fragment_container_model,
            foreign_procedure_template,
            procedure_template_fragment_container_model,
            fragment_placed_into_model,
            procedure_template_fragment_placed_into_model,
        })
    }
}
