impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateAttributes,
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
            "disposal_procedure_templates",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplate = self
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
        let disposed_asset_model = self
            .disposed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateAttributes::DisposedAssetModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_disposed_asset_model = self
            .procedure_template_disposed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateAttributes::ProcedureTemplateDisposedAssetModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            disposed_asset_model,
            foreign_procedure_template,
            procedure_template_disposed_asset_model,
        })
    }
}
