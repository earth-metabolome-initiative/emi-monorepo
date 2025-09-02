impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
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
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes,
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
            "photograph_procedure_templates",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate = self
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
        let photographed_with_model = self
            .photographed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes::PhotographedWithModel,
                ),
            )?;
        let procedure_template_photographed_with_model = self
            .procedure_template_photographed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes::ProcedureTemplatePhotographedWithModel,
                ),
            )?;
        let photographed_asset_model = self
            .photographed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes::PhotographedAssetModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_photographed_asset_model = self
            .procedure_template_photographed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes::ProcedureTemplatePhotographedAssetModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            photographed_with_model,
            procedure_template_photographed_with_model,
            photographed_asset_model,
            foreign_procedure_template,
            procedure_template_photographed_asset_model,
        })
    }
}
